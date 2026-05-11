#!/bin/bash

chmod +x scripts/*.sh

bash scripts/banner.sh

echo ""
echo "AnnaumiXYZ Mining cloned successfully."
echo "Next step:"
echo "cp .env.example .env"
echo "nano .env"
echo "cargo build --release"
echo "bash scripts/run-dry.sh"
echo ""
