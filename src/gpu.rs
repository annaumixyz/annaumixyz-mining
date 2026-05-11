use alloy::primitives::{keccak256, B256, U256};
use eyre::{eyre, Result};
use ocl::{flags, Buffer, Context, Device, Kernel, Platform, Program, Queue};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

const KERNEL_SRC: &str = include_str!("keccak_kernel.cl");
const DEFAULT_BATCH: usize = 1 << 22;

pub struct GpuMiner {
    queue: Queue,
    program: Program,
    device_name: String,
    batch_size: usize,
}

impl GpuMiner {
    pub fn new(batch_size: Option<usize>) -> Result<Self> {
        let batch_size = batch_size.unwrap_or(DEFAULT_BATCH);

        let platform = Platform::default();
        let device =
            Device::first(platform).map_err(|e| eyre!("tidak ada OpenCL device: {e}"))?;

        let device_name = device.name().unwrap_or_else(|_| "Unknown GPU".into());

        let context = Context::builder()
            .platform(platform)
            .devices(device)
            .build()?;

        let queue = Queue::new(&context, device, None)?;

        let program = Program::builder()
            .src(KERNEL_SRC)
            .devices(device)
            .build(&context)?;

        Ok(Self {
            queue,
            program,
            device_name,
            batch_size,
        })
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn self_test(&self) -> Result<()> {
        let challenge = B256::from(*b"abcdefghijklmnopqrstuvwxyz012345");
        let difficulty = U256::MAX;
        let nonce_base: u64 = 12345;

        let cw = split_challenge_le(&challenge);
        let dw = split_difficulty_be(difficulty);

        let found_nonce = Buffer::<u64>::builder()
            .queue(self.queue.clone())
            .flags(flags::MEM_READ_WRITE)
            .len(1)
            .copy_host_slice(&[0u64])
            .build()?;

        let found_flag = Buffer::<i32>::builder()
            .queue(self.queue.clone())
            .flags(flags::MEM_READ_WRITE)
            .len(1)
            .copy_host_slice(&[0i32])
            .build()?;

        let kernel = Kernel::builder()
            .program(&self.program)
            .name("mine_keccak")
            .queue(self.queue.clone())
            .global_work_size(1usize)
            .arg(cw[0])
            .arg(cw[1])
            .arg(cw[2])
            .arg(cw[3])
            .arg(dw[0])
            .arg(dw[1])
            .arg(dw[2])
            .arg(dw[3])
            .arg(nonce_base)
            .arg(&found_nonce)
            .arg(&found_flag)
            .build()?;

        unsafe {
            kernel.enq()?;
        }

        self.queue.finish()?;

        let mut flag = [0i32];
        found_flag.read(&mut flag[..]).enq()?;

        if flag[0] == 0 {
            return Err(eyre!("GPU self-test gagal"));
        }

        let mut got = [0u64];
        found_nonce.read(&mut got[..]).enq()?;

        if got[0] != nonce_base {
            return Err(eyre!("GPU self-test nonce salah"));
        }

        Ok(())
    }

    pub fn mine(
        &self,
        challenge: B256,
        difficulty: U256,
        start_nonce: u64,
        stop_flag: Arc<AtomicBool>,
        attempts_counter: Arc<AtomicU64>,
    ) -> Result<Option<u64>> {
        let cw = split_challenge_le(&challenge);
        let dw = split_difficulty_be(difficulty);

        let found_nonce = Buffer::<u64>::builder()
            .queue(self.queue.clone())
            .flags(flags::MEM_READ_WRITE)
            .len(1)
            .copy_host_slice(&[0u64])
            .build()?;

        let found_flag = Buffer::<i32>::builder()
            .queue(self.queue.clone())
            .flags(flags::MEM_READ_WRITE)
            .len(1)
            .copy_host_slice(&[0i32])
            .build()?;

        let mut nonce_base = start_nonce;

        loop {
            if stop_flag.load(Ordering::Relaxed) {
                return Ok(None);
            }

            found_flag.write(&[0i32][..]).enq()?;
            found_nonce.write(&[0u64][..]).enq()?;

            let kernel = Kernel::builder()
                .program(&self.program)
                .name("mine_keccak")
                .queue(self.queue.clone())
                .global_work_size(self.batch_size)
                .arg(cw[0])
                .arg(cw[1])
                .arg(cw[2])
                .arg(cw[3])
                .arg(dw[0])
                .arg(dw[1])
                .arg(dw[2])
                .arg(dw[3])
                .arg(nonce_base)
                .arg(&found_nonce)
                .arg(&found_flag)
                .build()?;

            unsafe {
                kernel.enq()?;
            }

            self.queue.finish()?;
            attempts_counter.fetch_add(self.batch_size as u64, Ordering::Relaxed);

            let mut flag = [0i32];
            found_flag.read(&mut flag[..]).enq()?;

            if flag[0] != 0 {
                let mut got = [0u64];
                found_nonce.read(&mut got[..]).enq()?;
                let nonce = got[0];

                let h = cpu_hash(&challenge, U256::from(nonce));
                if U256::from_be_bytes::<32>(h.0) < difficulty {
                    return Ok(Some(nonce));
                }
            }

            nonce_base = nonce_base.wrapping_add(self.batch_size as u64);
            std::thread::sleep(Duration::from_micros(1));
        }
    }
}

fn split_challenge_le(challenge: &B256) -> [u64; 4] {
    let b = challenge.as_slice();

    [
        u64::from_le_bytes(b[0..8].try_into().unwrap()),
        u64::from_le_bytes(b[8..16].try_into().unwrap()),
        u64::from_le_bytes(b[16..24].try_into().unwrap()),
        u64::from_le_bytes(b[24..32].try_into().unwrap()),
    ]
}

fn split_difficulty_be(d: U256) -> [u64; 4] {
    let b = d.to_be_bytes::<32>();

    [
        u64::from_be_bytes(b[0..8].try_into().unwrap()),
        u64::from_be_bytes(b[8..16].try_into().unwrap()),
        u64::from_be_bytes(b[16..24].try_into().unwrap()),
        u64::from_be_bytes(b[24..32].try_into().unwrap()),
    ]
}

fn cpu_hash(challenge: &B256, nonce: U256) -> B256 {
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(challenge.as_slice());
    buf[32..].copy_from_slice(&nonce.to_be_bytes::<32>());
    keccak256(buf)
}
