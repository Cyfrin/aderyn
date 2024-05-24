// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { LockupDynamic_Fork_Test } from "../LockupDynamic.t.sol";
import { LockupLinear_Fork_Test } from "../LockupLinear.t.sol";
import { LockupTranched_Fork_Test } from "../LockupTranched.t.sol";

/// @dev An ERC-20 asset with 2 decimals.
IERC20 constant ASSET = IERC20(0xdB25f211AB05b1c97D595516F45794528a807ad8);
address constant HOLDER = 0x1bee4F735062CD00841d6997964F187f5f5F5Ac9;

contract EURS_LockupDynamic_Fork_Test is LockupDynamic_Fork_Test(ASSET, HOLDER) { }

contract EURS_LockupLinear_Fork_Test is LockupLinear_Fork_Test(ASSET, HOLDER) { }

contract EURS_LockupTranched_Fork_Test is LockupTranched_Fork_Test(ASSET, HOLDER) { }
