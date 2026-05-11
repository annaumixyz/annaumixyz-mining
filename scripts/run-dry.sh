#!/bin/bash

bash scripts/banner.sh

echo "Starting AnnaumiXYZ GPU miner..."
echo "Pastikan .env sudah dibuat."

cargo run --release
