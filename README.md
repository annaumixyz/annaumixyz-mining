# AnnaumiXYZ Mining

Custom GPU mining framework based on HASH-style architecture.

## Features

- CUDA GPU mining
- Multi GPU support
- Multi wallet support
- Flashbots support
- Dry run mode
- Custom mining framework

## Mining Flow

1. Load wallets
2. Fetch challenge
3. GPU brute force nonce
4. Local verification
5. Broadcast transaction

## Safety

Never upload:
- wallets.txt
- private keys
- live config

Always start using dry-run mode.
