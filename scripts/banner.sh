#!/bin/bash

CYAN="\033[96m"
GREEN="\033[92m"
YELLOW="\033[93m"
WHITE="\033[97m"
RESET="\033[0m"

clear

echo -e "${CYAN}"
cat << "EOF"
 █████╗ ███╗   ██╗███╗   ██╗ █████╗ ██╗   ██╗███╗   ███╗██╗██╗  ██╗██╗   ██╗███████╗
██╔══██╗████╗  ██║████╗  ██║██╔══██╗██║   ██║████╗ ████║██║╚██╗██╔╝╚██╗ ██╔╝╚══███╔╝
███████║██╔██╗ ██║██╔██╗ ██║███████║██║   ██║██╔████╔██║██║ ╚███╔╝  ╚████╔╝   ███╔╝
██╔══██║██║╚██╗██║██║╚██╗██║██╔══██║██║   ██║██║╚██╔╝██║██║ ██╔██╗   ╚██╔╝   ███╔╝
██║  ██║██║ ╚████║██║ ╚████║██║  ██║╚██████╔╝██║ ╚═╝ ██║██║██╔╝ ██╗   ██║   ███████╗
╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝

                         A N N A U M I X Y Z   M I N I N G
EOF
echo -e "${RESET}"

echo -e "${CYAN}====================================================================${RESET}"
echo -e "${WHITE} Custom GPU Mining Framework${RESET}"
echo -e "${CYAN}====================================================================${RESET}"
echo -e "${GREEN} Miner       :${RESET} AnnaumiXYZ Mining"
echo -e "${GREEN} Mode        :${RESET} DRY RUN / LIVE"
echo -e "${GREEN} Wallet      :${RESET} .env PRIVATE_KEY"
echo -e "${GREEN} Config      :${RESET} .env"
echo -e "${GREEN} GPU Engine  :${RESET} OpenCL"
echo -e "${GREEN} Network     :${RESET} Ethereum Mainnet"
echo -e "${GREEN} Broadcast   :${RESET} RPC Submitter"
echo -e "${CYAN}====================================================================${RESET}"
echo ""
