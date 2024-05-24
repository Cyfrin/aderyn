// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { CreateWithTimestamps_LockupDynamic_BatchLockup_Fork_Test } from "../batch-lockup/createWithTimestampsLD.t.sol";
import { CreateWithTimestamps_LockupLinear_BatchLockup_Fork_Test } from "../batch-lockup/createWithTimestampsLL.t.sol";
import { CreateWithTimestamps_LockupTranched_BatchLockup_Fork_Test } from "../batch-lockup/createWithTimestampsLT.t.sol";
import { MerkleLL_Fork_Test } from "../merkle-lockup/MerkleLL.t.sol";
import { MerkleLT_Fork_Test } from "../merkle-lockup/MerkleLT.t.sol";

/// @dev An ERC-20 asset with 6 decimals.
IERC20 constant usdc = IERC20(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);

contract USDC_CreateWithTimestamps_LockupDynamic_BatchLockup_Fork_Test is
    CreateWithTimestamps_LockupDynamic_BatchLockup_Fork_Test(usdc)
{ }

contract USDC_CreateWithTimestamps_LockupLinear_BatchLockup_Fork_Test is
    CreateWithTimestamps_LockupLinear_BatchLockup_Fork_Test(usdc)
{ }

contract USDC_CreateWithTimestamps_LockupTranched_BatchLockup_Fork_Test is
    CreateWithTimestamps_LockupTranched_BatchLockup_Fork_Test(usdc)
{ }

contract USDC_MerkleLL_Fork_Test is MerkleLL_Fork_Test(usdc) { }

contract USDC_MerkleLT_Fork_Test is MerkleLT_Fork_Test(usdc) { }
