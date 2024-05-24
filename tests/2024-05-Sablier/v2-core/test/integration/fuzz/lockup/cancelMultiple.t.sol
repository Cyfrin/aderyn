// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Solarray } from "solarray/src/Solarray.sol";

import { Lockup } from "src/types/DataTypes.sol";

import { Integration_Test } from "../../Integration.t.sol";
import { CancelMultiple_Integration_Shared_Test } from "../../shared/lockup/cancelMultiple.t.sol";

abstract contract CancelMultiple_Integration_Fuzz_Test is Integration_Test, CancelMultiple_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, CancelMultiple_Integration_Shared_Test) {
        CancelMultiple_Integration_Shared_Test.setUp();
    }

    function testFuzz_CancelMultiple(
        uint256 timeJump,
        uint40 endTime
    )
        external
        whenNotDelegateCalled
        givenNoNull
        givenAllStreamsWarm
        whenCallerAuthorizedAllStreams
        givenAllStreamsCancelable
    {
        timeJump = _bound(timeJump, 0 seconds, defaults.TOTAL_DURATION() - 1 seconds);
        endTime = boundUint40(endTime, defaults.END_TIME(), defaults.END_TIME() + defaults.TOTAL_DURATION());

        // Create a new stream with a different end time.
        uint256 streamId = createDefaultStreamWithEndTime(endTime);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Create the stream IDs array.
        uint256[] memory streamIds = Solarray.uint256s(testStreamIds[0], streamId);

        // Expect the assets to be refunded to the Sender.
        uint128 senderAmount0 = lockup.refundableAmountOf(streamIds[0]);
        expectCallToTransfer({ to: users.sender, value: senderAmount0 });
        uint128 senderAmount1 = lockup.refundableAmountOf(streamIds[1]);
        expectCallToTransfer({ to: users.sender, value: senderAmount1 });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit CancelLockupStream({
            streamId: streamIds[0],
            sender: users.sender,
            recipient: users.recipient,
            asset: dai,
            senderAmount: senderAmount0,
            recipientAmount: defaults.DEPOSIT_AMOUNT() - senderAmount0
        });
        vm.expectEmit({ emitter: address(lockup) });
        emit CancelLockupStream({
            streamId: streamIds[1],
            sender: users.sender,
            recipient: users.recipient,
            asset: dai,
            senderAmount: senderAmount1,
            recipientAmount: defaults.DEPOSIT_AMOUNT() - senderAmount1
        });

        // Cancel the streams.
        lockup.cancelMultiple(streamIds);

        // Assert that the streams have been updated.
        Lockup.Status expectedStatus0 =
            senderAmount0 == defaults.DEPOSIT_AMOUNT() ? Lockup.Status.DEPLETED : Lockup.Status.CANCELED;
        Lockup.Status expectedStatus1 =
            senderAmount1 == defaults.DEPOSIT_AMOUNT() ? Lockup.Status.DEPLETED : Lockup.Status.CANCELED;
        assertEq(lockup.statusOf(streamIds[0]), expectedStatus0, "status0");
        assertEq(lockup.statusOf(streamIds[1]), expectedStatus1, "status1");

        // Assert that the streams are not cancelable anymore.
        assertFalse(lockup.isCancelable(streamIds[0]), "isCancelable0");
        assertFalse(lockup.isCancelable(streamIds[1]), "isCancelable1");

        // Assert that the refunded amounts have been updated.
        uint128 expectedRefundedAmount0 = senderAmount0;
        uint128 expectedRefundedAmount1 = senderAmount1;
        assertEq(lockup.getRefundedAmount(streamIds[0]), expectedRefundedAmount0, "refundedAmount0");
        assertEq(lockup.getRefundedAmount(streamIds[1]), expectedRefundedAmount1, "refundedAmount1");

        // Assert that the NFTs have not been burned.
        address expectedNFTOwner = users.recipient;
        assertEq(lockup.getRecipient(streamIds[0]), expectedNFTOwner, "NFT owner0");
        assertEq(lockup.getRecipient(streamIds[1]), expectedNFTOwner, "NFT owner1");
    }
}
