// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { LockupDynamic_Fork_Test } from "../LockupDynamic.t.sol";
import { LockupLinear_Fork_Test } from "../LockupLinear.t.sol";
import { LockupTranched_Fork_Test } from "../LockupTranched.t.sol";

/// @dev An ERC-20 asset with 6 decimals.
IERC20 constant ASSET = IERC20(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);
address constant HOLDER = 0x09528d637deb5857dc059dddE6316D465a8b3b69;

contract USDC_LockupDynamic_Fork_Test is LockupDynamic_Fork_Test(ASSET, HOLDER) { }

contract USDC_LockupLinear_Fork_Test is LockupLinear_Fork_Test(ASSET, HOLDER) { }

contract USDC_LockupTranched_Fork_Test is LockupTranched_Fork_Test(ASSET, HOLDER) { }
