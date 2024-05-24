// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { LockupDynamic_Fork_Test } from "../LockupDynamic.t.sol";
import { LockupLinear_Fork_Test } from "../LockupLinear.t.sol";
import { LockupTranched_Fork_Test } from "../LockupTranched.t.sol";

/// @dev An ERC-20 asset with a large total supply.
IERC20 constant ASSET = IERC20(0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE);
address constant HOLDER = 0x73AF3bcf944a6559933396c1577B257e2054D935;

contract SHIB_LockupDynamic_Fork_Test is LockupDynamic_Fork_Test(ASSET, HOLDER) { }

contract SHIB_LockupLinear_Fork_Test is LockupLinear_Fork_Test(ASSET, HOLDER) { }

contract SHIB_LockupTranched_Fork_Test is LockupTranched_Fork_Test(ASSET, HOLDER) { }
