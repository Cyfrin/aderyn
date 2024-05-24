// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup, LockupTranched } from "src/types/DataTypes.sol";

import { Lockup_Invariant_Test } from "./Lockup.t.sol";
import { LockupTranchedCreateHandler } from "./handlers/LockupTranchedCreateHandler.sol";
import { LockupTranchedHandler } from "./handlers/LockupTranchedHandler.sol";

/// @dev Invariant tests for {SablierV2LockupTranched}.
contract LockupTranched_Invariant_Test is Lockup_Invariant_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    LockupTranchedHandler internal tranchedHandler;
    LockupTranchedCreateHandler internal tranchedCreateHandler;

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Lockup_Invariant_Test.setUp();

        // Deploy the LockupTranched handlers.
        tranchedHandler = new LockupTranchedHandler({
            asset_: dai,
            timestampStore_: timestampStore,
            lockupStore_: lockupStore,
            lockupTranched_: lockupTranched
        });
        tranchedCreateHandler = new LockupTranchedCreateHandler({
            asset_: dai,
            timestampStore_: timestampStore,
            lockupStore_: lockupStore,
            lockupTranched_: lockupTranched
        });

        // Label the contracts.
        vm.label({ account: address(tranchedHandler), newLabel: "LockupTranchedHandler" });
        vm.label({ account: address(tranchedCreateHandler), newLabel: "LockupTranchedCreateHandler" });

        // Cast the LockupTranched contract and handler.
        lockup = lockupTranched;
        lockupHandler = tranchedHandler;

        // Target the LockupTranched handlers for invariant testing.
        targetContract(address(tranchedHandler));
        targetContract(address(tranchedCreateHandler));

        // Prevent these contracts from being fuzzed as `msg.sender`.
        excludeSender(address(tranchedHandler));
        excludeSender(address(tranchedCreateHandler));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     INVARIANTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Settled streams must not appear as cancelable in {SablierV2LockupTranched.getStream}.
    function invariant_StatusSettled_GetStream() external view {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockupTranched.statusOf(streamId) == Lockup.Status.SETTLED) {
                assertFalse(
                    lockupTranched.getStream(streamId).isCancelable,
                    "Invariant violation: stream returned by getStream() is cancelable"
                );
            }
        }
    }

    /// @dev Unordered tranche timestamps are not allowed.
    function invariant_TrancheTimestampsOrdered() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            LockupTranched.Tranche[] memory tranches = lockupTranched.getTranches(streamId);
            uint40 previousTimestamp = tranches[0].timestamp;
            for (uint256 j = 1; j < tranches.length; ++j) {
                assertGt(tranches[j].timestamp, previousTimestamp, "Invariant violated: tranche timestamps not ordered");
                previousTimestamp = tranches[j].timestamp;
            }
        }
    }
}
