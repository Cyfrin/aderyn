// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup } from "src/types/DataTypes.sol";

import { Lockup_Invariant_Test } from "./Lockup.t.sol";
import { LockupLinearHandler } from "./handlers/LockupLinearHandler.sol";
import { LockupLinearCreateHandler } from "./handlers/LockupLinearCreateHandler.sol";

/// @dev Invariant tests for {SablierV2LockupLinear}.
contract LockupLinear_Invariant_Test is Lockup_Invariant_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    LockupLinearHandler internal lockupLinearHandler;
    LockupLinearCreateHandler internal lockupLinearCreateHandler;

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Lockup_Invariant_Test.setUp();

        // Deploy the lockupLinear contract handlers.
        lockupLinearHandler = new LockupLinearHandler({
            asset_: dai,
            timestampStore_: timestampStore,
            lockupStore_: lockupStore,
            lockupLinear_: lockupLinear
        });
        lockupLinearCreateHandler = new LockupLinearCreateHandler({
            asset_: dai,
            timestampStore_: timestampStore,
            lockupStore_: lockupStore,
            lockupLinear_: lockupLinear
        });

        // Label the handler contracts.
        vm.label({ account: address(lockupLinearHandler), newLabel: "LockupLinearHandler" });
        vm.label({ account: address(lockupLinearCreateHandler), newLabel: "LockupLinearCreateHandler" });

        // Cast the lockupLinear contract as {ISablierV2Lockup} and the lockupLinear handler as {LockupHandler}.
        lockup = lockupLinear;
        lockupHandler = lockupLinearHandler;

        // Target the lockupLinear handlers for invariant testing.
        targetContract(address(lockupLinearHandler));
        targetContract(address(lockupLinearCreateHandler));

        // Prevent these contracts from being fuzzed as `msg.sender`.
        excludeSender(address(lockupLinearHandler));
        excludeSender(address(lockupLinearCreateHandler));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     INVARIANTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev If it is not zero, the cliff time must be strictly greater than the start time.
    function invariant_CliffTimeGtStartTimeOrZero() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockupLinear.getCliffTime(streamId) > 0) {
                assertGt(
                    lockupLinear.getCliffTime(streamId),
                    lockupLinear.getStartTime(streamId),
                    "Invariant violated: cliff time <= start time"
                );
            }
        }
    }

    /// @dev The end time must not be less than or equal to the cliff time.
    function invariant_EndTimeGtCliffTime() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            assertGt(
                lockupLinear.getEndTime(streamId),
                lockupLinear.getCliffTime(streamId),
                "Invariant violated: end time <= cliff time"
            );
        }
    }

    /// @dev Settled streams must not appear as cancelable in {SablierV2LockupLinear.getStream}.
    function invariant_StatusSettled_GetStream() external view {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockupLinear.statusOf(streamId) == Lockup.Status.SETTLED) {
                assertFalse(
                    lockupLinear.getStream(streamId).isCancelable,
                    "Invariant violation: stream returned by getStream() is cancelable"
                );
            }
        }
    }
}
