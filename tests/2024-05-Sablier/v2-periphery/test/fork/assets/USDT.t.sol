// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { CreateWithTimestamps_LockupDynamic_BatchLockup_Fork_Test } from "../batch-lockup/createWithTimestampsLD.t.sol";
import { CreateWithTimestamps_LockupLinear_BatchLockup_Fork_Test } from "../batch-lockup/createWithTimestampsLL.t.sol";
import { CreateWithTimestamps_LockupTranched_BatchLockup_Fork_Test } from "../batch-lockup/createWithTimestampsLT.t.sol";
import { MerkleLL_Fork_Test } from "../merkle-lockup/MerkleLL.t.sol";
import { MerkleLT_Fork_Test } from "../merkle-lockup/MerkleLT.t.sol";

/// @dev An ERC-20 asset that suffers from the missing return value bug.
IERC20 constant usdt = IERC20(0xdAC17F958D2ee523a2206206994597C13D831ec7);

contract USDT_CreateWithTimestamps_LockupDynamic_BatchLockup_Fork_Test is
    CreateWithTimestamps_LockupDynamic_BatchLockup_Fork_Test(usdt)
{ }

contract USDT_CreateWithTimestamps_LockupLinear_BatchLockup_Fork_Test is
    CreateWithTimestamps_LockupLinear_BatchLockup_Fork_Test(usdt)
{ }

contract USDT_CreateWithTimestamps_LockupTranched_BatchLockup_Fork_Test is
    CreateWithTimestamps_LockupTranched_BatchLockup_Fork_Test(usdt)
{ }

contract USDT_MerkleLL_Fork_Test is MerkleLL_Fork_Test(usdt) { }

contract USDT_MerkleLT_Fork_Test is MerkleLT_Fork_Test(usdt) { }
