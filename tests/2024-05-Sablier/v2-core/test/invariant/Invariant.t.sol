// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { StdInvariant } from "forge-std/src/StdInvariant.sol";

import { Base_Test } from "../Base.t.sol";
import { TimestampStore } from "./stores/TimestampStore.sol";

/// @notice Common logic needed by all invariant tests.
abstract contract Invariant_Test is Base_Test, StdInvariant {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    TimestampStore internal timestampStore;

    /*//////////////////////////////////////////////////////////////////////////
                                     MODIFIERS
    //////////////////////////////////////////////////////////////////////////*/

    modifier useCurrentTimestamp() {
        vm.warp(timestampStore.currentTimestamp());
        _;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Base_Test.setUp();

        // Deploy the handlers.
        timestampStore = new TimestampStore();

        // Label the handlers.
        vm.label({ account: address(timestampStore), newLabel: "TimestampStore" });

        // Prevent these contracts from being fuzzed as `msg.sender`.
        excludeSender(address(lockupDynamic));
        excludeSender(address(lockupLinear));
        excludeSender(address(timestampStore));
    }
}
