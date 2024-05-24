// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { ISablierV2LockupDynamic } from "src/interfaces/ISablierV2LockupDynamic.sol";

import { LockupStore } from "../stores/LockupStore.sol";
import { TimestampStore } from "../stores/TimestampStore.sol";
import { LockupHandler } from "./LockupHandler.sol";

/// @dev This contract and not {SablierV2LockupDynamic} is exposed to Foundry for invariant testing. The goal is
/// to bound and restrict the inputs that get passed to the real-world contract to avoid getting reverts.
contract LockupDynamicHandler is LockupHandler {
    constructor(
        IERC20 asset_,
        TimestampStore timestampStore_,
        LockupStore lockupStore_,
        ISablierV2LockupDynamic lockupDynamic_
    )
        LockupHandler(asset_, timestampStore_, lockupStore_, lockupDynamic_)
    { }
}
