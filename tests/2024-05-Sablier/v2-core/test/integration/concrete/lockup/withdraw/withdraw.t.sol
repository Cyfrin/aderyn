// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";
import { ISablierV2Recipient } from "src/interfaces/hooks/ISablierV2Recipient.sol";
import { ISablierV2Sender } from "src/interfaces/hooks/ISablierV2Sender.sol";
import { Errors } from "src/libraries/Errors.sol";

import { Lockup } from "src/types/DataTypes.sol";

import { Withdraw_Integration_Shared_Test } from "../../../shared/lockup/withdraw.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract Withdraw_Integration_Concrete_Test is Integration_Test, Withdraw_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, Withdraw_Integration_Shared_Test) {
        Withdraw_Integration_Shared_Test.setUp();
    }

    /*//////////////////////////////////////////////////////////////////////////
                                       TESTS
    //////////////////////////////////////////////////////////////////////////*/

    function test_RevertWhen_DelegateCalled() external {
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        bytes memory callData =
            abi.encodeCall(ISablierV2Lockup.withdraw, (defaultStreamId, users.recipient, withdrawAmount));
        (bool success, bytes memory returnData) = address(lockup).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    function test_RevertGiven_Null() external whenNotDelegateCalled {
        uint256 nullStreamId = 1729;
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.withdraw({ streamId: nullStreamId, to: users.recipient, amount: withdrawAmount });
    }

    function test_RevertGiven_StreamDepleted() external whenNotDelegateCalled givenNotNull {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });

        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamDepleted.selector, defaultStreamId));
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: withdrawAmount });
    }

    function test_RevertWhen_ToZeroAddress() external whenNotDelegateCalled givenNotNull givenStreamNotDepleted {
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_WithdrawToZeroAddress.selector, defaultStreamId));
        lockup.withdraw({ streamId: defaultStreamId, to: address(0), amount: withdrawAmount });
    }

    function test_RevertWhen_WithdrawAmountZero()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
    {
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_WithdrawAmountZero.selector, defaultStreamId));
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: 0 });
    }

    function test_RevertWhen_Overdraw()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
    {
        uint128 withdrawableAmount = 0;
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2Lockup_Overdraw.selector, defaultStreamId, MAX_UINT128, withdrawableAmount
            )
        );
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: MAX_UINT128 });
    }

    function test_RevertWhen_CallerUnknown()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
    {
        address unknownCaller = address(0xCAFE);

        // Make Eve the caller in this test.
        resetPrank({ msgSender: unknownCaller });

        // Run the test.
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2Lockup_WithdrawalAddressNotRecipient.selector,
                defaultStreamId,
                unknownCaller,
                unknownCaller
            )
        );
        lockup.withdraw({ streamId: defaultStreamId, to: unknownCaller, amount: withdrawAmount });
    }

    function test_RevertWhen_CallerSender()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
    {
        // Make the Sender the caller in this test.
        resetPrank({ msgSender: users.sender });

        // Run the test.
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2Lockup_WithdrawalAddressNotRecipient.selector,
                defaultStreamId,
                users.sender,
                users.sender
            )
        );
        lockup.withdraw({ streamId: defaultStreamId, to: users.sender, amount: withdrawAmount });
    }

    function test_RevertWhen_CallerFormerRecipient()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
    {
        // Transfer the stream to Alice.
        lockup.transferFrom(users.recipient, users.alice, defaultStreamId);

        // Run the test.
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2Lockup_WithdrawalAddressNotRecipient.selector,
                defaultStreamId,
                users.recipient,
                users.recipient
            )
        );
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: withdrawAmount });
    }

    function test_Withdraw_CallerApprovedOperator()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
    {
        // Approve the operator to handle the stream.
        lockup.approve({ to: users.operator, tokenId: defaultStreamId });

        // Make the operator the caller in this test.
        resetPrank({ msgSender: users.operator });

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.operator, amount: defaults.WITHDRAW_AMOUNT() });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = defaults.WITHDRAW_AMOUNT();
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    function test_Withdraw_SenderNotContract()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
        whenCallerRecipient
    {
        test_Withdraw_CallerRecipient(defaultStreamId, users.sender);
    }

    function test_Withdraw_SenderDoesNotImplementHook()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
        whenCallerRecipient
        givenSenderContract
    {
        // Create the stream with a noop contract as the stream's sender.
        uint256 streamId = createDefaultStreamWithSender(address(noop));

        test_Withdraw_CallerRecipient(streamId, address(noop));
    }

    modifier givenSenderImplementsHook() {
        _;
    }

    function test_Withdraw_ReentrancySender()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
        whenCallerRecipient
        givenSenderContract
        givenSenderImplementsHook
    {
        // Create the stream with a reentrant contract as the stream's sender.
        uint256 streamId = createDefaultStreamWithSender(address(reentrantSender));

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Halve the withdraw amount so that the recipient can re-entry and make another withdrawal.
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT() / 2;

        // Expect a call to the hook.
        vm.expectCall(
            address(reentrantSender),
            abi.encodeCall(
                ISablierV2Sender.onLockupStreamWithdrawn, (streamId, users.recipient, users.alice, withdrawAmount)
            )
        );

        // Make the withdrawal.
        lockup.withdraw({ streamId: streamId, to: users.alice, amount: withdrawAmount });

        // Assert that the stream's status is still "STREAMING".
        Lockup.Status actualStatus = lockup.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(streamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    modifier whenNoSenderReentrancy() {
        _;
    }

    function test_Withdraw_RevertingSender()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
        whenCallerRecipient
        givenSenderContract
        givenSenderImplementsHook
        whenNoSenderReentrancy
    {
        // Create the stream with a contract as the stream's sender.
        uint256 streamId = createDefaultStreamWithSender(address(revertingSender));

        test_Withdraw_CallerRecipient(streamId, address(revertingSender));
    }

    modifier whenSenderDoesNotRevert() {
        _;
    }

    function test_Withdraw_GoodSender()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressNotRecipient
        whenCallerRecipient
        givenSenderContract
        givenSenderImplementsHook
        whenNoSenderReentrancy
        whenSenderDoesNotRevert
    {
        // Create the stream with a contract as the stream's sender.
        uint256 streamId = createDefaultStreamWithSender(address(goodSender));

        test_Withdraw_CallerRecipient(streamId, address(goodSender));
    }

    function test_Withdraw_CallerUnknownAddress()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
    {
        // Make the unknown address the caller in this test.
        resetPrank({ msgSender: address(0xCAFE) });

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: defaults.WITHDRAW_AMOUNT() });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = defaults.WITHDRAW_AMOUNT();
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    function test_Withdraw_CallerRecipient()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: defaults.WITHDRAW_AMOUNT() });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = defaults.WITHDRAW_AMOUNT();
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    modifier whenCallerSender() {
        resetPrank({ msgSender: users.sender });
        _;
    }

    function test_Withdraw_EndTimeNotInTheFuture()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
        whenCallerSender
    {
        // Warp to the stream's end.
        vm.warp({ newTimestamp: defaults.END_TIME() });

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: defaults.DEPOSIT_AMOUNT() });

        // Assert that the stream's status is "DEPLETED".
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.DEPLETED;
        assertEq(actualStatus, expectedStatus);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(defaultStreamId);
        assertFalse(isCancelable, "isCancelable");

        // Assert that the NFT has not been burned.
        address actualNFTowner = lockup.ownerOf({ tokenId: defaultStreamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTowner, expectedNFTOwner, "NFT owner");
    }

    modifier givenEndTimeInTheFuture() {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });
        _;
    }

    function test_Withdraw_StreamHasBeenCanceled()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
        whenCallerSender
        givenEndTimeInTheFuture
    {
        // Cancel the stream.
        lockup.cancel(defaultStreamId);

        // Set the withdraw amount to the withdrawable amount.
        uint128 withdrawAmount = lockup.withdrawableAmountOf(defaultStreamId);

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: withdrawAmount });

        // Assert that the stream's status is "DEPLETED".
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.DEPLETED;
        assertEq(actualStatus, expectedStatus);

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");

        // Assert that the NFT has not been burned.
        address actualNFTowner = lockup.ownerOf({ tokenId: defaultStreamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTowner, expectedNFTOwner, "NFT owner");
    }

    function test_Withdraw_RecipientNotContract()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
        whenCallerSender
        givenEndTimeInTheFuture
        whenStreamHasNotBeenCanceled
    {
        test_Withdraw_CallerSender(defaultStreamId, users.recipient);
    }

    function test_Withdraw_RecipientDoesNotImplementHook()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
        whenCallerSender
        givenEndTimeInTheFuture
        whenStreamHasNotBeenCanceled
        givenRecipientContract
    {
        // Create the stream with a noop contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(noop));

        test_Withdraw_CallerSender(streamId, address(noop));
    }

    modifier givenRecipientImplementsHook() {
        _;
    }

    function test_Withdraw_RecipientReverts()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
        whenCallerSender
        givenEndTimeInTheFuture
        whenStreamHasNotBeenCanceled
        givenRecipientContract
        givenRecipientImplementsHook
    {
        // Create the stream with a reverting contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(revertingRecipient));

        test_Withdraw_CallerSender(streamId, address(revertingRecipient));
    }

    modifier whenRecipientDoesNotRevert() {
        _;
    }

    function test_Withdraw_RecipientReentrancy()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
        whenCallerSender
        givenEndTimeInTheFuture
        whenStreamHasNotBeenCanceled
        givenRecipientContract
        givenRecipientImplementsHook
        whenRecipientDoesNotRevert
    {
        // Create the stream with a reentrant contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(reentrantRecipient));

        // Halve the withdraw amount so that the recipient can re-entry and make another withdrawal.
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT() / 2;

        // Expect a call to the hook.
        vm.expectCall(
            address(reentrantRecipient),
            abi.encodeCall(
                ISablierV2Recipient.onLockupStreamWithdrawn,
                (streamId, users.sender, address(reentrantRecipient), withdrawAmount)
            )
        );

        // Make the withdrawal.
        lockup.withdraw({ streamId: streamId, to: address(reentrantRecipient), amount: withdrawAmount });

        // Assert that the stream's status is still "STREAMING".
        Lockup.Status actualStatus = lockup.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(streamId);
        uint128 expectedWithdrawnAmount = defaults.WITHDRAW_AMOUNT();
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    modifier whenNoRecipientReentrancy() {
        _;
    }

    function test_Withdraw()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenWithdrawalAddressIsRecipient
        whenCallerSender
        givenEndTimeInTheFuture
        whenStreamHasNotBeenCanceled
        givenRecipientContract
        givenRecipientImplementsHook
        whenRecipientDoesNotRevert
        whenNoRecipientReentrancy
    {
        // Create the stream with a contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(goodRecipient));

        test_Withdraw_CallerSender(streamId, address(goodRecipient));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                  INTERNAL HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    function test_Withdraw_CallerRecipient(uint256 streamId, address sender) internal {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Set the withdraw amount to the default amount.
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();

        // Expect a call to the hook if the sender is a contract.
        if (sender.code.length > 0) {
            vm.expectCall(
                address(sender),
                abi.encodeCall(
                    ISablierV2Sender.onLockupStreamWithdrawn, (streamId, users.recipient, users.alice, withdrawAmount)
                )
            );
        }

        // Make the withdrawal.
        lockup.withdraw({ streamId: streamId, to: users.alice, amount: withdrawAmount });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(streamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    function test_Withdraw_CallerSender(uint256 streamId, address recipient) internal {
        // Set the withdraw amount to the default amount.
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();

        // Expect the assets to be transferred to the recipient contract.
        expectCallToTransfer({ to: address(recipient), value: withdrawAmount });

        // Expect a call to the hook if the recipient is a contract.
        if (recipient.code.length > 0) {
            vm.expectCall(
                address(recipient),
                abi.encodeCall(
                    ISablierV2Recipient.onLockupStreamWithdrawn,
                    (streamId, users.sender, address(recipient), withdrawAmount)
                )
            );
        }

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream({ streamId: streamId, to: address(recipient), asset: dai, amount: withdrawAmount });
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: streamId });

        // Make the withdrawal.
        lockup.withdraw({ streamId: streamId, to: address(recipient), amount: withdrawAmount });

        // Assert that the stream's status is still "STREAMING".
        Lockup.Status actualStatus = lockup.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(streamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }
}
