// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";
import { Lockup } from "src/types/DataTypes.sol";

import { Invariant_Test } from "./Invariant.t.sol";
import { LockupHandler } from "./handlers/LockupHandler.sol";
import { LockupStore } from "./stores/LockupStore.sol";

/// @notice Common invariant test logic needed across contracts that inherit from {SablierV2Lockup}.
abstract contract Lockup_Invariant_Test is Invariant_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    ISablierV2Lockup internal lockup;
    LockupHandler internal lockupHandler;
    LockupStore internal lockupStore;

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Invariant_Test.setUp();

        // Deploy and label the lockup store contract.
        lockupStore = new LockupStore();
        vm.label({ account: address(lockupStore), newLabel: "LockupStore" });

        // Exclude the lockup store from being fuzzed as `msg.sender`.
        excludeSender(address(lockupStore));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     INVARIANTS
    //////////////////////////////////////////////////////////////////////////*/

    // solhint-disable max-line-length
    function invariant_ContractBalance() external useCurrentTimestamp {
        uint256 contractBalance = dai.balanceOf(address(lockup));

        uint256 lastStreamId = lockupStore.lastStreamId();
        uint256 depositedAmountsSum;
        uint256 refundedAmountsSum;
        uint256 withdrawnAmountsSum;
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            depositedAmountsSum += uint256(lockup.getDepositedAmount(streamId));
            refundedAmountsSum += uint256(lockup.getRefundedAmount(streamId));
            withdrawnAmountsSum += uint256(lockup.getWithdrawnAmount(streamId));
        }

        assertGe(
            contractBalance,
            depositedAmountsSum - refundedAmountsSum - withdrawnAmountsSum,
            unicode"Invariant violation: contract balances < Σ deposited amounts - Σ refunded amounts - Σ withdrawn amounts"
        );
    }

    function invariant_DepositedAmountGteStreamedAmount() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            assertGe(
                lockup.getDepositedAmount(streamId),
                lockup.streamedAmountOf(streamId),
                "Invariant violation: deposited amount < streamed amount"
            );
        }
    }

    function invariant_DepositedAmountGteWithdrawableAmount() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            assertGe(
                lockup.getDepositedAmount(streamId),
                lockup.withdrawableAmountOf(streamId),
                "Invariant violation: deposited amount < withdrawable amount"
            );
        }
    }

    function invariant_DepositedAmountGteWithdrawnAmount() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            assertGe(
                lockup.getDepositedAmount(streamId),
                lockup.getWithdrawnAmount(streamId),
                "Invariant violation: deposited amount < withdrawn amount"
            );
        }
    }

    function invariant_DepositedAmountNotZero() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            uint128 depositAmount = lockup.getDepositedAmount(streamId);
            assertNotEq(depositAmount, 0, "Invariant violated: stream non-null, deposited amount zero");
        }
    }

    function invariant_EndTimeGtStartTime() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            assertGt(
                lockup.getEndTime(streamId),
                lockup.getStartTime(streamId),
                "Invariant violation: end time <= start time"
            );
        }
    }

    function invariant_NextStreamId() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 nextStreamId = lockup.nextStreamId();
            assertEq(nextStreamId, lastStreamId + 1, "Invariant violation: next stream ID not incremented");
        }
    }

    function invariant_StartTimeNotZero() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            uint40 startTime = lockup.getStartTime(streamId);
            assertGt(startTime, 0, "Invariant violated: start time zero");
        }
    }

    function invariant_StatusCanceled() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockup.statusOf(streamId) == Lockup.Status.CANCELED) {
                assertGt(
                    lockup.getRefundedAmount(streamId),
                    0,
                    "Invariant violation: canceled stream with a zero refunded amount"
                );
                assertFalse(lockup.isCancelable(streamId), "Invariant violation: canceled stream is cancelable");
                assertEq(
                    lockup.refundableAmountOf(streamId),
                    0,
                    "Invariant violation: canceled stream with a non-zero refundable amount"
                );
                assertGt(
                    lockup.withdrawableAmountOf(streamId),
                    0,
                    "Invariant violation: canceled stream with a zero withdrawable amount"
                );
            }
        }
    }

    function invariant_StatusDepleted() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockup.isDepleted(streamId)) {
                assertEq(
                    lockup.getDepositedAmount(streamId) - lockup.getRefundedAmount(streamId),
                    lockup.getWithdrawnAmount(streamId),
                    "Invariant violation: depleted stream with deposited amount - refunded amount != withdrawn amount"
                );
                assertFalse(lockup.isCancelable(streamId), "Invariant violation: depleted stream is cancelable");
                assertEq(
                    lockup.refundableAmountOf(streamId),
                    0,
                    "Invariant violation: depleted stream with a non-zero refundable amount"
                );
                assertEq(
                    lockup.withdrawableAmountOf(streamId),
                    0,
                    "Invariant violation: depleted stream with a non-zero withdrawable amount"
                );
            }
        }
    }

    function invariant_StatusPending() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockup.statusOf(streamId) == Lockup.Status.PENDING) {
                assertEq(
                    lockup.getRefundedAmount(streamId),
                    0,
                    "Invariant violation: pending stream with a non-zero refunded amount"
                );
                assertEq(
                    lockup.getWithdrawnAmount(streamId),
                    0,
                    "Invariant violation: pending stream with a non-zero withdrawn amount"
                );
                assertEq(
                    lockup.refundableAmountOf(streamId),
                    lockup.getDepositedAmount(streamId),
                    "Invariant violation: pending stream with refundable amount != deposited amount"
                );
                assertEq(
                    lockup.streamedAmountOf(streamId),
                    0,
                    "Invariant violation: pending stream with a non-zero streamed amount"
                );
                assertEq(
                    lockup.withdrawableAmountOf(streamId),
                    0,
                    "Invariant violation: pending stream with a non-zero withdrawable amount"
                );
            }
        }
    }

    function invariant_StatusSettled() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockup.statusOf(streamId) == Lockup.Status.SETTLED) {
                assertEq(
                    lockup.getRefundedAmount(streamId),
                    0,
                    "Invariant violation: settled stream with a non-zero refunded amount"
                );
                assertFalse(lockup.isCancelable(streamId), "Invariant violation: settled stream is cancelable");
                assertEq(
                    lockup.refundableAmountOf(streamId),
                    0,
                    "Invariant violation: settled stream with a non-zero refundable amount"
                );
                assertEq(
                    lockup.streamedAmountOf(streamId),
                    lockup.getDepositedAmount(streamId),
                    "Invariant violation: settled stream with streamed amount != deposited amount"
                );
            }
        }
    }

    function invariant_StatusStreaming() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            if (lockup.statusOf(streamId) == Lockup.Status.STREAMING) {
                assertEq(
                    lockup.getRefundedAmount(streamId),
                    0,
                    "Invariant violation: streaming stream with a non-zero refunded amount"
                );
                assertLt(
                    lockup.streamedAmountOf(streamId),
                    lockup.getDepositedAmount(streamId),
                    "Invariant violation: streaming stream with streamed amount >= deposited amount"
                );
            }
        }
    }

    /// @dev See diagram at https://i.postimg.cc/sfHsBkWB/mermaid-diagram-2023-04-25-190035.png.
    function invariant_StatusTransitions() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        if (lastStreamId == 0) {
            return;
        }

        for (uint256 i = 0; i < lastStreamId - 1; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            Lockup.Status currentStatus = lockup.statusOf(streamId);

            // If this is the first time the status is checked for this stream, skip the invariant test.
            if (!lockupStore.isPreviousStatusRecorded(streamId)) {
                lockupStore.updateIsPreviousStatusRecorded(streamId);
                return;
            }

            // Check the status transition invariants.
            Lockup.Status previousStatus = lockupStore.previousStatusOf(streamId);
            if (previousStatus == Lockup.Status.PENDING) {
                assertNotEq(
                    currentStatus, Lockup.Status.DEPLETED, "Invariant violation: pending stream turned depleted"
                );
            } else if (previousStatus == Lockup.Status.STREAMING) {
                assertNotEq(
                    currentStatus, Lockup.Status.PENDING, "Invariant violation: streaming stream turned pending"
                );
            } else if (previousStatus == Lockup.Status.SETTLED) {
                assertNotEq(currentStatus, Lockup.Status.PENDING, "Invariant violation: settled stream turned pending");
                assertNotEq(
                    currentStatus, Lockup.Status.STREAMING, "Invariant violation: settled stream turned streaming"
                );
                assertNotEq(
                    currentStatus, Lockup.Status.CANCELED, "Invariant violation: settled stream turned canceled"
                );
            } else if (previousStatus == Lockup.Status.CANCELED) {
                assertNotEq(currentStatus, Lockup.Status.PENDING, "Invariant violation: canceled stream turned pending");
                assertNotEq(
                    currentStatus, Lockup.Status.STREAMING, "Invariant violation: canceled stream turned streaming"
                );
                assertNotEq(currentStatus, Lockup.Status.SETTLED, "Invariant violation: canceled stream turned settled");
            } else if (previousStatus == Lockup.Status.DEPLETED) {
                assertEq(currentStatus, Lockup.Status.DEPLETED, "Invariant violation: depleted status changed");
            }

            // Set the current status as the previous status.
            lockupStore.updatePreviousStatusOf(streamId, currentStatus);
        }
    }

    function invariant_StreamedAmountGteWithdrawableAmount() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            assertGe(
                lockup.streamedAmountOf(streamId),
                lockup.withdrawableAmountOf(streamId),
                "Invariant violation: streamed amount < withdrawable amount"
            );
        }
    }

    function invariant_StreamedAmountGteWithdrawnAmount() external useCurrentTimestamp {
        uint256 lastStreamId = lockupStore.lastStreamId();
        for (uint256 i = 0; i < lastStreamId; ++i) {
            uint256 streamId = lockupStore.streamIds(i);
            assertGe(
                lockup.streamedAmountOf(streamId),
                lockup.getWithdrawnAmount(streamId),
                "Invariant violation: streamed amount < withdrawn amount"
            );
        }
    }
}
