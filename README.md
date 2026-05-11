# 🐎 ANNAUMIXYZ MINING

<div align="center">

```txt
                         >>\.
                        /_  )`.
                       /  _)`^)`.   _.---. _
                      (_,' \  `^-)""      `.\
                            |              | \
                            \              / |
                           / \  /.___.'\  (_/
                          /   \/       /   /
                         /   /        /   /
                        /___/        /___/


 █████╗ ███╗   ██╗███╗   ██╗ █████╗ ██╗   ██╗███╗   ███╗██╗██╗  ██╗██╗   ██╗███████╗
██╔══██╗████╗  ██║████╗  ██║██╔══██╗██║   ██║████╗ ████║██║╚██╗██╔╝╚██╗ ██╔╝╚══███╔╝
███████║██╔██╗ ██║██╔██╗ ██║███████║██║   ██║██╔████╔██║██║ ╚███╔╝  ╚████╔╝   ███╔╝
██╔══██║██║╚██╗██║██║╚██╗██║██╔══██║██║   ██║██║╚██╔╝██║██║ ██╔██╗   ╚██╔╝   ███╔╝
██║  ██║██║ ╚████║██║ ╚████║██║  ██║╚██████╔╝██║ ╚═╝ ██║██║██╔╝ ██╗   ██║   ███████╗
╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝

═══════════════════════════════════════════════════════════════════════════════

               ⚡ GPU OPENCL HASH MINER • ONCHAIN POW ⚡

═══════════════════════════════════════════════════════════════════════════════
```

### 🚀 High Performance GPU Onchain Mining Framework

GPU Powered • Ethereum Network • Rust + OpenCL

</div>

---

# ✨ FEATURES

- ⚡ GPU OpenCL mining engine
- 🧠 Rust RPC on-chain controller
- 🔥 Keccak256 hashing engine
- 🚀 High-speed parallel GPU batching
- 🌐 Ethereum RPC support
- 🔐 Secure `.env` wallet system
- 📊 Real-time hashrate monitoring
- 🐎 Custom AnnaumiXYZ terminal banner
- 🧪 Dry-run mode
- ⛏️ Live mining mode
- 📦 Makefile build system
- 🎮 CPU fallback support
- 🔄 Epoch auto-refresh
- 🛰️ RPC auto polling
- 💻 Linux optimized

---

# 📦 REPOSITORY STRUCTURE

```txt
annaumixyz-mining
├── src/
│   ├── main.rs
│   ├── gpu.rs
│   └── keccak_kernel.cl
│
├── scripts/
│   ├── banner.sh
│   ├── run-dry.sh
│   └── run-live.sh
│
├── docs/
│   └── FLOW.md
│
├── Cargo.toml
├── Makefile
├── .env.example
├── install.sh
├── .gitignore
└── README.md
```

---

# ⚙️ REQUIREMENTS

## Ubuntu / Debian

```bash
sudo apt update

sudo apt install -y \
build-essential \
pkg-config \
ocl-icd-opencl-dev \
clinfo \
curl \
git \
nano
```

---

# 🦀 INSTALL RUST

```bash
curl https://sh.rustup.rs -sSf | sh
```

Enable environment:

```bash
source ~/.cargo/env
```

Check installation:

```bash
cargo --version
```

---

# 🎮 GPU CHECK

```bash
clinfo | head
```

If GPU appears:
- OpenCL Version
- Platform Name
- Device Name

then mining is ready.

---

# 🚀 INSTALLATION

Clone repository:

```bash
git clone https://github.com/annaumixyz/annaumixyz-mining.git
```

Open project:

```bash
cd annaumixyz-mining
```

Run installer:

```bash
bash install.sh
```

---

# 🔑 WALLET SETUP

Copy environment:

```bash
cp .env.example .env
```

Edit wallet config:

```bash
nano .env
```

Example:

```env
PRIVATE_KEY=0xYOUR_PRIVATE_KEY
RPC_URL=https://eth.llamarpc.com
GPU=1
GPU_BATCH=4194304
MINER_THREADS=8
PRIORITY_GWEI=5
MAX_FEE_GWEI=100
```

---

# 🧪 DRY RUN

Safe testing mode:

```bash
bash scripts/run-dry.sh
```

This mode:
- tests GPU
- tests RPC
- tests hashing
- validates mining flow

without real transaction submission.

---

# ⛏️ LIVE MINING

```bash
bash scripts/run-live.sh
```

---

# 🧹 BUILD

Compile release binary:

```bash
cargo build --release
```

Binary output:

```txt
target/release/annaumixyz-mining
```

---

# ⚡ MAKEFILE COMMANDS

Build:

```bash
make build
```

Run:

```bash
make run
```

Clean:

```bash
make clean
```

Install binary:

```bash
make install
```

---

# 🔥 MINING FLOW

```txt
Wallet
   ↓
Fetch Challenge
   ↓
GPU OpenCL Mining
   ↓
Keccak256 Hashing
   ↓
Nonce Found
   ↓
Proof Validation
   ↓
Submit Transaction
```

---

# 🧠 GPU ENGINE

Engine includes:
- OpenCL GPU kernel
- parallel nonce search
- Keccak256 hashing
- difficulty validation
- GPU batching system

Kernel file:

```txt
src/keccak_kernel.cl
```

---

# 🌐 ETHEREUM NETWORK

Default:

```txt
Ethereum Mainnet
```

Contract:

```txt
0xAC7b5d06fa1e77D08aea40d46cB7C5923A87A0cc
```

RPC Example:

```txt
https://eth.llamarpc.com
```

---

# 🖥️ SUPPORTED HARDWARE

## NVIDIA
- RTX Series
- GTX Series
- Tesla
- Quadro

## AMD
- RX Series
- Radeon Pro

OpenCL required.

---

# 📊 PERFORMANCE

Mining performance depends on:
- GPU model
- VRAM
- OpenCL driver
- batch size
- RPC latency

---

# 🐎 CUSTOM TERMINAL BANNER

Features:
- horse ASCII logo
- ANNAUMIXYZ branding
- neon terminal style
- GPU mining dashboard
- Ethereum onchain display

---

# 🔐 SECURITY

Never upload:
- `.env`
- private keys
- wallet backups
- VPS credentials

Sensitive files already protected via `.gitignore`.

---

# ⚠️ DISCLAIMER

This project is intended for:
- blockchain research
- educational purposes
- mining experimentation
- GPU compute learning

Use responsibly.

---

<div align="center">

# 👑 ANNAUMIXYZ

### SPEED • POWER • FOCUS • PROFIT

🐎 GPU Powered • Onchain Mining • Rust + OpenCL 🐎

# DONATE
EVM : 0x3be0650d0d0408a0de0fd761ba2c88ee430a0af0

</div>
