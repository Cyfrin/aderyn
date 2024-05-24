// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup, LockupDynamic } from "src/types/DataTypes.sol";

import { Lockup_Invariant_Test } from "./Lockup.t.sol";
import { LockupDynamicCreateHandler } from "./handlers/LockupDynamicCreateHandler.sol";
import { LockupDynamicHandler } from "./handlers/LockupDynamicHandler.sol";

/// @dev Invariant tests for {SablierV2LockupDynamic}.
contract LockupDynamic_Invariant_Test is Lockup_Invariant_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    LockupDynamicHandler internal dynamicHandler;
    LockupDynamicCreateHandler internal dynamicCreateHandler;

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Lockup_Invariant_Test.setUp();

        // Deploy the LockupDynamic handlers.
        dynamicHandler = new LockupDynamicHandler({
            asset_: dai,
            timestampStore_: timestampStore,
            lockupStore_: lockupStore,
            lockupDynamic_: lockupDynamic
        });
        dynamicCreateHandler = new LockupDynamicCreateHandler({
            asset_: dai,
            timestampStore_: timestampStore,
            lockupStore_: lockupStore,
            lockupDynamic_: lockupDynamic
        });

        // Label the contracts.
        vm.label({ account: address(dynamicHandler), newLabel: "LockupDynamicHandler" });
        vm.label({ account: address(dynamicCreateHandler), newLabel: "LockupDynamicCreateHandler" });

        // Cast the LockupDynamic contract and handler.
        lockup = lockupDynamic;
        lockupHandler = dynamicHandler;

        // Target the LockupDynamic handlers for invariant testing.
        targetContract(address(dynamicHandler));
        targetContract(address(dynamicCreateHandler));

        // Prevent these contracts from being fuzzed as `msg.sender`.
        excludeSender(address(dynamicHandler));
        excludeSender(address(dynamicCreateHandler));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     INVARIANTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Unordered segment timestamps are not allowed.
    function invariant_SegmentTimestampsOrdered() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            LockupDynamic.Segment[] memory segments = lockupDynamic.getSegments(streamId);
            uint40 previousTimestamp = segments[0].timestamp;
            for (uint256 j = 1; j < segments.length; ++j) {
                assertGt(segments[j].timestamp, previousTimestamp, "Invariant violated: segment timestamps not ordered");
                previousTimestamp = segments[j].timestamp;
            }
        }
    }

    /// @dev Settled streams must not appear as cancelable in {SablierV2LockupDynamic.getStream}.
    function invariant_StatusSettled_GetStream() external view {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockupDynamic.statusOf(streamId) == Lockup.Status.SETTLED) {
                assertFalse(
                    lockupDynamic.getStream(streamId).isCancelable,
                    "Invariant violation: stream returned by getStream() is cancelable"
                );
            }
        }
    }
}
