#!/usr/bin/env bash

# Pre-requisites for running this script:
#
# - bash >=4.0.0
# - foundry (https://getfoundry.sh)

# Usage: ./shell/deploy-multi-chain.sh [options] [[chain1 chain2 ...]]
#   Enters interactive mode if no `.env.deployment` file is found
#
# Options:
#  --all              Deploy on all chains.
#  --broadcast        Broadcast the deployment and verify on Etherscan.
#  --deterministic    Deploy using the deterministic script.
#  -h, --help         Show available command-line options and exit.
#  -i, --interactive  Enters interactive mode and ignore .env.deployment.
#  --print            Simulate and show the deployment command.
#  -s, --script       Script to run from the `script` folder.
#  --with-gas-price   Specify gas price for transaction.
#
# Example: ./shell/deploy-multi-chain.sh # By default, deploys to Sepolia only
# Example: ./shell/deploy-multi-chain.sh --broadcast optimism polygon
# Example: ./shell/deploy-multi-chain.sh --broadcast --deterministic --print optimism
#
# Make sure to set up your `.env.deployment` file first.

# Strict mode: https://gist.github.com/vncsna/64825d5609c146e80de8b1fd623011ca
set -euo pipefail

# Color codes
EC='\033[0;31m' # Error Color
IC='\033[0;36m' # Info Color
NC='\033[0m' # No Color
SC='\033[0;32m' # Success Color
WC='\033[0;33m' # Warn Color

# Unicode characters for tick
TICK="\xE2\x9C\x94"

# Check: Bash >=4.0.0 is required for associative arrays
if ((BASH_VERSINFO[0] < 4)); then
    echo -e "${EC}Error:\nThis script requires Bash version 4.0.0 or higher.
    \nYou are currently using Bash version ${BASH_VERSINFO[0]}.${BASH_VERSINFO[1]}.${BASH_VERSINFO[2]}.
    \nPlease upgrade your Bash version and try again.${NC}"
    exit 1
fi

# Define usage
usage="\nUsage: ./shell/deploy-multi-chain.sh [-h] [--help] [--print] [-i] [--interactive] [-s] [--script]
                                     [--broadcast] [--deterministic] [--with-gas-price] [--all]
                                     [[chain1 chain2 ...]]
Examples:
    ./shell/deploy-multi-chain.sh # By default, deploys only to Sepolia
    ./shell/deploy-multi-chain.sh --broadcast optimism polygon
    ./shell/deploy-multi-chain.sh --broadcast --deterministic optimism
"

# Create deployments directory
deployments=./deployments
rm -rf ${deployments}
mkdir ${deployments}

# Addresses taken from https://docs.sablier.com/concepts/governance
export ARBITRUM_ADMIN="0xF34E41a6f6Ce5A45559B1D3Ee92E141a3De96376"
export ARBITRUM_SEPOLIA_ADMIN="0xb1bEF51ebCA01EB12001a639bDBbFF6eEcA12B9F"
export AVALANCHE_ADMIN="0x4735517616373c5137dE8bcCDc887637B8ac85Ce"
export BASE_ADMIN="0x83A6fA8c04420B3F9C7A4CF1c040b63Fbbc89B66"
export BSC_ADMIN="0x6666cA940D2f4B65883b454b7Bc7EEB039f64fa3"
export GNOSIS_ADMIN="0x72ACB57fa6a8fa768bE44Db453B1CDBa8B12A399"
export MAINNET_ADMIN="0x79Fb3e81aAc012c08501f41296CCC145a1E15844"
export OPTIMISM_ADMIN="0x43c76FE8Aec91F63EbEfb4f5d2a4ba88ef880350"
export POLYGON_ADMIN="0x40A518C5B9c1d3D6d62Ba789501CE4D526C9d9C6"
export SCROLL_ADMIN="0x0F7Ad835235Ede685180A5c611111610813457a9"
export SEPOLIA_ADMIN="0xb1bEF51ebCA01EB12001a639bDBbFF6eEcA12B9F"

# Flag for broadcast deployment
BROADCAST_DEPLOYMENT=false

# Flag for deterministic deployment
DETERMINISTIC_DEPLOYMENT=false

# Flags for gas price
GAS_PRICE=0
WITH_GAS_PRICE=false

# Flag for all chains
ON_ALL_CHAINS=false

# Flag for displaying deployment command
READ_ONLY=false

# Flag to enter interactive mode in case .env.deployment not found or --interactive is provided
INTERACTIVE=false

# Provided chains
provided_chains=()

# Script to execute
sol_script=""

# Declare the chains array
declare -A chains

# define function to initialize all configurations
function initialize {
    chains["arbitrum"]="$ARBITRUM_RPC_URL $ARBISCAN_API_KEY $ARBITRUM_ADMIN"
    chains["arbitrum_sepolia"]="$ARBITRUM_SEPOLIA_RPC_URL $ARBISCAN_API_KEY $ARBITRUM_SEPOLIA_ADMIN"
    chains["avalanche"]="$AVALANCHE_RPC_URL $SNOWTRACE_API_KEY $AVALANCHE_ADMIN"
    chains["base"]="$BASE_RPC_URL $BASESCAN_API_KEY $BASE_ADMIN"
    chains["bnb_smart_chain"]="$BSC_RPC_URL $BSCSCAN_API_KEY $BSC_ADMIN"
    chains["gnosis"]="$GNOSIS_RPC_URL $GNOSISSCAN_API_KEY $GNOSIS_ADMIN"
    chains["mainnet"]="$MAINNET_RPC_URL $ETHERSCAN_API_KEY $MAINNET_ADMIN"
    chains["optimism"]="$OPTIMISM_RPC_URL $OPTIMISTIC_API_KEY $OPTIMISM_ADMIN"
    chains["polygon"]="$POLYGON_RPC_URL $POLYGONSCAN_API_KEY $POLYGON_ADMIN"
    chains["sepolia"]="$SEPOLIA_RPC_URL $ETHERSCAN_API_KEY $SEPOLIA_ADMIN"
    chains["scroll"]="$SCROLL_RPC_URL $SCROLLSCAN_API_KEY $SCROLL_ADMIN"
}

# define function to initialize limited configurations
function initialize_interactive {
    # load values from the terminal prompt
    echo -e "1. Enter admin address: \c"
    read admin

    echo -e "2. Enter Etherscan API key: \c"
    read api_key
}

if [ -f .env.deployment ]; then
    # Source the .env.deployment file to load the variables
    source .env.deployment

    # initialize chains with all the configurations
    initialize
else
    # Set bool to enter intaractive mode
    INTERACTIVE=true

    # load values from the terminal prompt
    echo -e "${WC}Missing '.env.deployment'. Provide details below: ${NC}\n"

    # initialize chains
    initialize_interactive

fi

# Check for arguments passed to the script
for ((i=1; i<=$#; i++)); do
    # Convert the argument to lowercase
    arg=${!i,,}

    # Check if '--all' flag is provided in the arguments
    if [[ ${arg} == "--all" ]]; then
        ON_ALL_CHAINS=true
        provided_chains=("${!chains[@]}")
    fi

    # Check if '--broadcast' flag is provided the arguments
    if [[ ${arg} == "--broadcast" ]]; then
        BROADCAST_DEPLOYMENT=true
    fi

    # Check if '--deterministic' flag is provided in the arguments
    if [[ ${arg} == "--deterministic" ]]; then
        DETERMINISTIC_DEPLOYMENT=true
    fi

    # Show usage of this command with --help option
    if [[ ${arg} == "--help" || ${arg} == "-h" ]]; then
        echo -e "${usage}"
        # Get all chain names from the chains array
        names=("${!chains[@]}")
        # Sort the names
        sorted_names=($(printf "%s\n" "${names[@]}" | sort))
        # Print the header
        printf "\nSupported chains: \n%-20s %-20s\n" "Chain Name"
        printf "%-20s %-20s\n" "-----------"

        # Print the supported chains
        for chain in "${sorted_names[@]}"; do
            IFS=' ' read -r rpc_url api_key admin <<< "${chains[$chain]}"

            # Print the chain
            printf "%-20s %-20s\n" "${chain}"
        done
        exit 0
    fi

    # Check if '--interactive' flag is provided in the arguments
    if [[ ${arg} == "--interactive" || ${arg} == "-i" ]]; then
        INTERACTIVE=true
        echo -e "Interactive mode activated. Provide details below: \n"

        initialize_interactive
    fi

    # Check if '--print' flag is provided in the arguments
    if [[ ${arg} == "--print" ]]; then
        READ_ONLY=true
    fi

    # Check if '--script' flag is provided in the arguments
    if [[ ${arg} == "--script" || ${arg} == "-s" ]]; then
        files=(script/*.s.sol)

        # Present the list of available scripts
        echo "Please select a script:"
        select file in "${files[@]}"; do
            if [[ -n ${file} ]]; then
                echo -e "${SC}+${NC} You selected ${IC}${file}${NC}"
                sol_script=${file}
                break
            else
                echo -e "${EC}Invalid selection${NC}"
            fi
        done
    fi

    # Check if '--with-gas-price' flag is provided in the arguments
    if [[ ${arg} == "--with-gas-price" ]]; then
        WITH_GAS_PRICE=true

        # Increment index to get the next argument, which should be the gas price
        ((i++))
        GAS_PRICE=${!i}
        if ! [[ ${GAS_PRICE} =~ ^[0-9]+$ ]]; then
            echo -e "${EC}Error: Invalid value for --with-gas-price, must be number${NC}"
            exit 1
        fi
    fi

    # Check for passed chains
    if [[ ${arg} != "--all" &&
          ${arg} != "--broadcast" &&
          ${arg} != "--deterministic" &&
          ${arg} != "--help" &&
          ${arg} != "-h" &&
          ${arg} != "-i" &&
          ${arg} != "--interactive" &&
          ${arg} != "--print" &&
          ${arg} != "-s" &&
          ${arg} != "--script" &&
          ${arg} != "--with-gas-price" &&
          ${ON_ALL_CHAINS} == false
    ]]; then
        # check for synonyms
        if [[ ${arg} == "ethereum" ]]; then
          arg="mainnet"
        fi
        provided_chains+=("${arg}")
    fi
done

# Set the default chain to Sepolia if no chains are provided
if [ ${#provided_chains[@]} -eq 0 ]; then
    provided_chains=("sepolia")
fi

# Compile the contracts
echo "Compiling the contracts..."

# Deploy to the provided chains
for chain in "${provided_chains[@]}"; do
    # Check if the provided chain is defined
    if [[ ! -v "chains[${chain}]" ]]; then
        printf "\n${WC}Warning for '${chain}': Invalid command or chain name. Get the full list of supported chains: ${NC}"
        printf "\n\n\t${IC}./shell/deploy-multi-chain.sh --help${NC}\n"
        continue
    fi

    echo -e "\n${IC}Deployment on ${chain} started...${NC}"

    if [[ ${INTERACTIVE} == true ]]; then
        # load values from the terminal prompt
        echo -e "Enter RPC URL for ${chain}: \c"
        read rpc_url
    else
        # Split the configuration into RPC, API key, and admin
        IFS=' ' read -r rpc_url api_key admin <<< "${chains[$chain]}"
    fi

    # Declare the deployment command
    declare -a deployment_command

    deployment_command=("forge")

    # Construct the deployment command
    if [[ ${DETERMINISTIC_DEPLOYMENT} == true ]]; then
        echo -e "${SC}+${NC} Deterministic address"
        if [[ ${sol_script} == "" ]]; then
            deployment_command+=("script" "script/DeployDeterministicCore.s.sol" "--ffi")
        else
            deployment_command+=("script" "${sol_script}")
        fi
        deployment_command+=("--rpc-url" "${rpc_url}")

        ####################################################################
        # Distinct ways to construct command with string elements
        # While execution adds single quotes around them while
        # echo removes single quotes
        ####################################################################
        if [[ ${READ_ONLY} == true ]]; then
            deployment_command+=("--sig" "'run(address)'")
        else
            deployment_command+=("--sig" "run(address)")
        fi
    else
        # Construct the command
        if [[ ${sol_script} == "" ]]; then
            deployment_command+=("script" "script/DeployCore.s.sol")
        else
            deployment_command+=("script" "${sol_script}")
        fi
        deployment_command+=("--rpc-url" "${rpc_url}")

        if [[ ${READ_ONLY} == true ]]; then
            deployment_command+=("--sig" "'run(address)'")
        else
            deployment_command+=("--sig" "run(address)")
        fi
    fi

    deployment_command+=("${admin}")
    deployment_command+=("-vvv")

    # Append additional options if gas price is enabled
    if [[ ${WITH_GAS_PRICE} == true ]]; then
        gas_price_in_gwei=$(echo "scale=2; ${GAS_PRICE} / 1000000000" | bc)
        echo -e "${SC}+${NC} Max gas price: ${gas_price_in_gwei} gwei"
        deployment_command+=("--with-gas-price" "${GAS_PRICE}")
    fi

    # Append additional options if broadcast is enabled
    if [[ ${BROADCAST_DEPLOYMENT} == true ]]; then
        echo -e "${SC}+${NC} Broadcasting on-chain"
        deployment_command+=("--broadcast" "--verify" "--etherscan-api-key" "${api_key}")
    else
        echo -e "${SC}+${NC} Simulating on forked chain"
    fi

    if [[ ${READ_ONLY} == true ]]; then
        # Print deployment_command
        echo -e "${SC}+${NC} Printing command without action\n"
        echo -e "FOUNDRY_PROFILE=optimized ${deployment_command[@]}"
    else
        # Execute the deployment command and print the logs in real-time
        output=$(FOUNDRY_PROFILE=optimized "${deployment_command[@]}" |& tee /dev/fd/2) || true

        # Check for error in output
        if [[ ${output} == *"Error"* ]]; then
            exit 1
        fi

        # Create a file for the chain
        chain_file="${deployments}/${chain}.txt"
        touch "${chain_file}"

        # Extract and save contract addresses
        lockupDynamic_address=$(echo "${output}" | awk '/lockupDynamic: contract/{print $NF}')
        lockupLinear_address=$(echo "${output}" | awk '/lockupLinear: contract/{print $NF}')
        lockupTranched_address=$(echo "${output}" | awk '/lockupTranched: contract/{print $NF}')
        nftDescriptor_address=$(echo "${output}" | awk '/nftDescriptor: contract/{print $NF}')

        # Save to the chain file
        {
            echo "SablierV2LockupDynamic = ${lockupDynamic_address}"
            echo "SablierV2LockupLinear = ${lockupLinear_address}"
            echo "SablierV2LockupTranched = ${lockupTranched_address}"
            echo "SablierV2NFTDescriptor = ${nftDescriptor_address}"
        } >> "$chain_file"

        echo -e "${SC}${TICK} Deployed on ${chain}. You can find the addresses in ${chain_file}${NC}"
    fi
done

echo -e "\nEnd of it."
