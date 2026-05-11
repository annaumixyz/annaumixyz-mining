#!/bin/bash

CYAN="\033[96m"
GREEN="\033[92m"
YELLOW="\033[93m"
RED="\033[91m"
WHITE="\033[97m"
RESET="\033[0m"

clear

echo -e "${CYAN}"
cat << "EOF"
      ___      _   _ _   _    _    _   _ __  __ ___ __  ____   ________ 
     / _ \    | \ | | \ | |  / \  | | | |  \/  |_ _|\ \/ /\ \ / /__  /
    / /_\ \   |  \| |  \| | / _ \ | | | | |\/| || |  \  /  \ V /  / / 
   /  _  \    | |\  | |\  |/ ___ \| |_| | |  | || |  /  \   | |  / /_ 
  /_/   \_\   |_| \_|_| \_/_/   \_\\___/|_|  |_|___|/_/\_\  |_| /____|

                         A N N A U M I X Y Z   M I N I N G
EOF
echo -e "${RESET}"

echo -e "${CYAN}====================================================================${RESET}"
echo -e "${WHITE} Custom GPU Mining Framework${RESET}"
echo -e "${CYAN}====================================================================${RESET}"
echo -e "${GREEN} Miner       :${RESET} AnnaumiXYZ Mining"
echo -e "${GREEN} Mode        :${RESET} DRY RUN default"
echo -e "${GREEN} Wallets     :${RESET} wallets.txt"
echo -e "${GREEN} Config      :${RESET} config.json"
echo -e "${GREEN} GPU Engine  :${RESET} CUDA"
echo -e "${GREEN} Broadcast   :${RESET} Flashbots / RPC"
echo -e "${CYAN}====================================================================${RESET}"
echo ""
