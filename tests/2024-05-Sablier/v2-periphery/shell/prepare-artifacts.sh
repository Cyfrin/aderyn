#!/usr/bin/env bash

# Notes:
# - The script must be run from the repo's root directory

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
  "$artifacts/libraries"

# Generate the artifacts with Forge
FOUNDRY_PROFILE=optimized forge build

# Copy the production artifacts
cp out-optimized/SablierV2BatchLockup.sol/SablierV2BatchLockup.json $artifacts
cp out-optimized/SablierV2MerkleLL.sol/SablierV2MerkleLL.json $artifacts
cp out-optimized/SablierV2MerkleLockupFactory.sol/SablierV2MerkleLockupFactory.json $artifacts
cp out-optimized/SablierV2MerkleLT.sol/SablierV2MerkleLT.json $artifacts

interfaces=./artifacts/interfaces
cp out-optimized/ISablierV2BatchLockup.sol/ISablierV2BatchLockup.json $interfaces
cp out-optimized/ISablierV2MerkleLL.sol/ISablierV2MerkleLL.json $interfaces
cp out-optimized/ISablierV2MerkleLockupFactory.sol/ISablierV2MerkleLockupFactory.json $interfaces
cp out-optimized/ISablierV2MerkleLT.sol/ISablierV2MerkleLT.json $interfaces

erc20=./artifacts/interfaces/erc20
cp out-optimized/IERC20.sol/IERC20.json $erc20

libraries=./artifacts/libraries
cp out-optimized/Errors.sol/Errors.json $libraries

# Format the artifacts with Prettier
bun prettier --write ./artifacts
