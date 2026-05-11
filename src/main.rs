use alloy::network::EthereumWallet;
use alloy::primitives::{address, keccak256, Address, B256, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use eyre::{eyre, Result};
use rand::Rng;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod gpu;

const HASH_CONTRACT_ADDRESS: Address = address!("AC7b5d06fa1e77D08aea40d46cB7C5923A87A0cc");
const DEFAULT_RPC_URL: &str = "https://eth.llamarpc.com";
const EPOCH_BLOCKS: u64 = 100;
const EPOCH_POLL_INTERVAL: Duration = Duration::from_secs(15);
const STATS_INTERVAL: Duration = Duration::from_secs(2);

sol! {
    #[sol(rpc)]
    contract HashToken {
        function currentDifficulty() external view returns (uint256);
        function totalMints() external view returns (uint256);
        function genesisComplete() external view returns (bool);
        function getChallenge(address miner) external view returns (bytes32);
        function miningState() external view returns (
            uint256 era,
            uint256 reward,
            uint256 difficulty,
            uint256 minted,
            uint256 remaining,
            uint256 epoch,
            uint256 epochBlocksLeft
        );
        function mine(uint256 nonce) external;
    }
}

struct Solution {
    nonce: U256,
    epoch: u64,
}

fn banner() {
    println!("=================================================");
    println!("              ANNAUMIXYZ MINING");
    println!("=================================================");
    println!(" GPU OpenCL HASH miner");
    println!(" Contract: {HASH_CONTRACT_ADDRESS}");
    println!("=================================================\n");
}

#[inline]
fn check_proof(challenge: &B256, nonce: U256, difficulty: U256) -> bool {
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(challenge.as_slice());
    buf[32..].copy_from_slice(&nonce.to_be_bytes::<32>());
    let hash = keccak256(buf);
    U256::from_be_bytes::<32>(hash.0) < difficulty
}

fn run_cpu_workers(
    challenge: B256,
    difficulty: U256,
    epoch: u64,
    start_nonce: U256,
    stop_flag: Arc<AtomicBool>,
    attempts_counter: Arc<AtomicU64>,
    num_threads: usize,
) -> Option<Solution> {
    let solution_slot: Mutex<Option<Solution>> = Mutex::new(None);
    let stride = U256::from(num_threads);

    std::thread::scope(|s| {
        for tid in 0..num_threads {
            let stop_flag = &stop_flag;
            let attempts_counter = &attempts_counter;
            let solution_slot = &solution_slot;

            s.spawn(move || {
                let mut nonce = start_nonce + U256::from(tid);
                let mut local_attempts: u64 = 0;

                loop {
                    if stop_flag.load(Ordering::Relaxed) {
                        attempts_counter.fetch_add(local_attempts, Ordering::Relaxed);
                        return;
                    }

                    if check_proof(&challenge, nonce, difficulty) {
                        let mut slot = solution_slot.lock().unwrap();
                        if slot.is_none() {
                            *slot = Some(Solution { nonce, epoch });
                        }
                        stop_flag.store(true, Ordering::Relaxed);
                        attempts_counter.fetch_add(local_attempts, Ordering::Relaxed);
                        return;
                    }

                    nonce += stride;
                    local_attempts += 1;

                    if local_attempts & 0x3fff == 0 {
                        attempts_counter.fetch_add(local_attempts, Ordering::Relaxed);
                        local_attempts = 0;
                    }
                }
            });
        }
    });

    solution_slot.into_inner().ok().flatten()
}

fn short_hex(bytes: &[u8]) -> String {
    hex::encode(&bytes[..8.min(bytes.len())])
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    banner();

    let raw_key = match std::env::var("PRIVATE_KEY") {
        Ok(v) => v,
        Err(_) => {
            println!("PRIVATE_KEY tidak ditemukan di .env");
            rpassword::prompt_password("Private key: ")?
        }
    };

    let key = raw_key.trim().trim_start_matches("0x");
    if key.len() != 64 {
        return Err(eyre!("Private key harus 64 hex chars"));
    }

    let signer: PrivateKeySigner = key.parse()?;
    let miner_address = signer.address();
    let wallet = EthereumWallet::from(signer);

    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| DEFAULT_RPC_URL.to_string());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse()?);

    let contract = HashToken::new(HASH_CONTRACT_ADDRESS, provider.clone());

    let num_threads = std::env::var("MINER_THREADS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or_else(num_cpus::get);

    let gpu_enabled = std::env::var("GPU").ok().as_deref() == Some("1");

    println!("Wallet : {miner_address}");
    println!("RPC    : {rpc_url}");
    println!("CPU    : {num_threads} threads");
    println!("GPU    : {}", if gpu_enabled { "ON" } else { "OFF" });

    match contract.miningState().call().await {
        Ok(s) => {
            println!("\nMining State:");
            println!("Era          : {}", s.era);
            println!("Reward       : {}", s.reward);
            println!("Difficulty   : {}", s.difficulty);
            println!("Minted       : {}", s.minted);
            println!("Remaining    : {}", s.remaining);
            println!("Epoch        : {}", s.epoch);
            println!("Epoch blocks : {}", s.epochBlocksLeft);
        }
        Err(e) => eprintln!("Warning miningState gagal: {e}"),
    }

    match contract.genesisComplete().call().await {
        Ok(v) if !v._0 => return Err(eyre!("Genesis belum complete, mining belum dibuka")),
        Ok(_) => println!("Genesis complete, mining open"),
        Err(e) => eprintln!("Warning genesisComplete gagal: {e}"),
    }

    let gpu_miner = if gpu_enabled {
        let batch = std::env::var("GPU_BATCH")
            .ok()
            .and_then(|v| v.parse::<usize>().ok());

        match gpu::GpuMiner::new(batch) {
            Ok(g) => {
                println!("GPU device : {}", g.device_name());
                println!("GPU batch  : {}", g.batch_size());
                g.self_test()?;
                println!("GPU self-test passed");
                Some(Arc::new(g))
            }
            Err(e) => {
                eprintln!("GPU init gagal, fallback CPU: {e}");
                None
            }
        }
    } else {
        None
    };

    let shutdown = Arc::new(AtomicBool::new(false));
    {
        let shutdown = shutdown.clone();
        tokio::spawn(async move {
            if tokio::signal::ctrl_c().await.is_ok() {
                println!("\nStopping...");
                shutdown.store(true, Ordering::Relaxed);
            }
        });
    }

    while !shutdown.load(Ordering::Relaxed) {
        let block_num = provider.get_block_number().await?;
        let epoch = block_num / EPOCH_BLOCKS;

        let challenge = contract.getChallenge(miner_address).call().await?._0;
        let difficulty = contract.currentDifficulty().call().await?._0;

        println!("\nRound start");
        println!("Block     : {block_num}");
        println!("Epoch     : {epoch}");
        println!("Challenge : 0x{}...", short_hex(challenge.as_slice()));
        println!("Difficulty: {difficulty}");

        let start_nonce_u64: u64 = rand::thread_rng().gen();
        let start_nonce = U256::from(start_nonce_u64);

        let stop_flag = Arc::new(AtomicBool::new(false));
        let attempts_counter = Arc::new(AtomicU64::new(0));
        let round_start = Instant::now();

        let watchdog = {
            let stop_flag = stop_flag.clone();
            let attempts_counter = attempts_counter.clone();
            let shutdown = shutdown.clone();
            let provider = provider.clone();

            tokio::spawn(async move {
                let mut last_print = Instant::now();
                let mut last_attempts: u64 = 0;
                let mut last_poll = Instant::now();

                loop {
                    tokio::time::sleep(Duration::from_millis(500)).await;

                    if stop_flag.load(Ordering::Relaxed) {
                        break;
                    }

                    if shutdown.load(Ordering::Relaxed) {
                        stop_flag.store(true, Ordering::Relaxed);
                        break;
                    }

                    if last_print.elapsed() >= STATS_INTERVAL {
                        let total = attempts_counter.load(Ordering::Relaxed);
                        let delta = total.saturating_sub(last_attempts);
                        let secs = last_print.elapsed().as_secs_f64().max(0.001);
                        let rate = delta as f64 / secs;

                        eprint!(
                            "\rHashrate: {:>12.2} H/s | attempts: {:>14}",
                            rate, total
                        );

                        last_attempts = total;
                        last_print = Instant::now();
                    }

                    if last_poll.elapsed() >= EPOCH_POLL_INTERVAL {
                        last_poll = Instant::now();

                        if let Ok(bn) = provider.get_block_number().await {
                            let current_epoch = bn / EPOCH_BLOCKS;
                            if current_epoch != epoch {
                                eprintln!("\nEpoch changed, restarting round");
                                stop_flag.store(true, Ordering::Relaxed);
                                break;
                            }
                        }
                    }
                }
            })
        };

        let mining_result: Option<Solution> = {
            let stop_flag = stop_flag.clone();
            let attempts_counter = attempts_counter.clone();

            if let Some(g) = gpu_miner.as_ref().cloned() {
                tokio::task::spawn_blocking(move || {
                    match g.mine(challenge, difficulty, start_nonce_u64, stop_flag, attempts_counter)
                    {
                        Ok(Some(nonce_u64)) => Some(Solution {
                            nonce: U256::from(nonce_u64),
                            epoch,
                        }),
                        Ok(None) => None,
                        Err(e) => {
                            eprintln!("GPU mining error: {e}");
                            None
                        }
                    }
                })
                .await?
            } else {
                tokio::task::spawn_blocking(move || {
                    run_cpu_workers(
                        challenge,
                        difficulty,
                        epoch,
                        start_nonce,
                        stop_flag,
                        attempts_counter,
                        num_threads,
                    )
                })
                .await?
            }
        };

        stop_flag.store(true, Ordering::Relaxed);
        let _ = watchdog.await;
        eprintln!();

        let Some(sol) = mining_result else {
            continue;
        };

        println!("FOUND NONCE: {}", sol.nonce);
        println!("Epoch      : {}", sol.epoch);
        println!("Time       : {:?}", round_start.elapsed());

        if !check_proof(&challenge, sol.nonce, difficulty) {
            eprintln!("CPU verify failed, skip submit");
            continue;
        }

        let priority_gwei: f64 = std::env::var("PRIORITY_GWEI")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5.0);

        let max_fee_gwei: f64 = std::env::var("MAX_FEE_GWEI")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100.0);

        let priority_wei = (priority_gwei * 1e9) as u128;
        let max_fee_wei = (max_fee_gwei * 1e9) as u128;

        println!("Submitting transaction...");

        let tx = contract
            .mine(sol.nonce)
            .max_priority_fee_per_gas(priority_wei)
            .max_fee_per_gas(max_fee_wei);

        match tx.send().await {
            Ok(pending) => {
                let tx_hash = *pending.tx_hash();
                println!("TX submitted: {tx_hash}");
                println!("https://etherscan.io/tx/{tx_hash}");
            }
            Err(e) => eprintln!("Submit failed: {e}"),
        }
    }

    Ok(())
}
