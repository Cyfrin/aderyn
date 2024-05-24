#!/usr/bin/env bash

# Pre-requisites:
# - foundry (https://getfoundry.sh)
# - bun (https://bun.sh)

# Strict mode: https://gist.github.com/vncsna/64825d5609c146e80de8b1fd623011ca
set -euo pipefail

# Delete the current artifacts
artifacts=./artifacts
rm -rf $artifacts

# Create the new artifacts directories
mkdir $artifacts \
  "$artifacts/interfaces" \
  "$artifacts/interfaces/erc20" \
  "$artifacts/interfaces/erc721" \
  "$artifacts/interfaces/hooks" \
  "$artifacts/libraries"

# Generate the artifacts with Forge
FOUNDRY_PROFILE=optimized forge build

# Copy the production artifacts
cp out-optimized/SablierV2LockupDynamic.sol/SablierV2LockupDynamic.json $artifacts
cp out-optimized/SablierV2LockupLinear.sol/SablierV2LockupLinear.json $artifacts
cp out-optimized/SablierV2LockupTranched.sol/SablierV2LockupTranched.json $artifacts
cp out-optimized/SablierV2NFTDescriptor.sol/SablierV2NFTDescriptor.json $artifacts

interfaces=./artifacts/interfaces
cp out-optimized/ISablierV2Lockup.sol/ISablierV2Lockup.json $interfaces
cp out-optimized/ISablierV2LockupDynamic.sol/ISablierV2LockupDynamic.json $interfaces
cp out-optimized/ISablierV2LockupLinear.sol/ISablierV2LockupLinear.json $interfaces
cp out-optimized/ISablierV2LockupTranched.sol/ISablierV2LockupTranched.json $interfaces
cp out-optimized/ISablierV2NFTDescriptor.sol/ISablierV2NFTDescriptor.json $interfaces

erc20=./artifacts/interfaces/erc20
cp out-optimized/IERC20.sol/IERC20.json $erc20

erc721=./artifacts/interfaces/erc721
cp out-optimized/IERC721.sol/IERC721.json $erc721
cp out-optimized/IERC721Metadata.sol/IERC721Metadata.json $erc721

hooks=./artifacts/interfaces/hooks
cp out-optimized/ISablierV2Recipient.sol/ISablierV2Recipient.json $hooks
cp out-optimized/ISablierV2Sender.sol/ISablierV2Sender.json $hooks

libraries=./artifacts/libraries
cp out-optimized/Errors.sol/Errors.json $libraries

# Format the artifacts with Prettier
bun prettier --write ./artifacts
