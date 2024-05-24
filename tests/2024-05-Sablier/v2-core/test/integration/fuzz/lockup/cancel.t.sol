// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup } from "src/types/DataTypes.sol";

import { Cancel_Integration_Shared_Test } from "../../shared/lockup/cancel.t.sol";
import { Integration_Test } from "../../Integration.t.sol";

abstract contract Cancel_Integration_Fuzz_Test is Integration_Test, Cancel_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, Cancel_Integration_Shared_Test) {
        Cancel_Integration_Shared_Test.setUp();
    }

    function testFuzz_Cancel_StatusPending(uint256 timeJump)
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamWarm
        whenCallerAuthorized
        givenStreamCancelable
    {
        timeJump = _bound(timeJump, 1 seconds, 100 weeks);

        // Warp to the past.
        vm.warp({ newTimestamp: getBlockTimestamp() - timeJump });

        // Cancel the stream.
        lockup.cancel(defaultStreamId);

        // Assert that the stream's status is "DEPLETED".
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.DEPLETED;
        assertEq(actualStatus, expectedStatus);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(defaultStreamId);
        assertFalse(isCancelable, "isCancelable");
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - Multiple values for the block timestamp
    /// - With and without withdrawals
    function testFuzz_Cancel(
        uint256 timeJump,
        uint128 withdrawAmount
    )
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamWarm
        whenCallerAuthorized
        givenStreamCancelable
        givenStatusStreaming
        givenRecipientContract
        givenRecipientImplementsHook
        whenRecipientDoesNotRevert
        whenNoRecipientReentrancy
    {
        timeJump = _bound(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() - 1);

        // Create the stream.
        uint256 streamId = createDefaultStreamWithRecipient(address(goodRecipient));

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Bound the withdraw amount.
        uint128 streamedAmount = lockup.streamedAmountOf(streamId);
        withdrawAmount = boundUint128(withdrawAmount, 0, streamedAmount - 1);

        // Make the withdrawal only if the amount is greater than zero.
        if (withdrawAmount > 0) {
            lockup.withdraw({ streamId: streamId, to: address(goodRecipient), amount: withdrawAmount });
        }

        // Expect the assets to be refunded to the Sender.
        uint128 senderAmount = lockup.refundableAmountOf(streamId);
        expectCallToTransfer({ to: users.sender, value: senderAmount });

        // Expect the relevant events to be emitted.
        uint128 recipientAmount = lockup.withdrawableAmountOf(streamId);
        vm.expectEmit({ emitter: address(lockup) });
        emit CancelLockupStream(streamId, users.sender, address(goodRecipient), dai, senderAmount, recipientAmount);
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: streamId });

        // Cancel the stream.
        lockup.cancel(streamId);

        // Assert that the stream's status is "CANCELED".
        Lockup.Status actualStatus = lockup.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.CANCELED;
        assertEq(actualStatus, expectedStatus);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(streamId);
        assertFalse(isCancelable, "isCancelable");

        // Assert that the NFT has not been burned.
        address actualNFTOwner = lockup.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = address(goodRecipient);
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }
}
