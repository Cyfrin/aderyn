// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";
import { ISablierV2Recipient } from "src/interfaces/hooks/ISablierV2Recipient.sol";
import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract Renounce_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) {
        defaultStreamId = createDefaultStream();
    }

    function test_RevertWhen_DelegateCalled() external givenStreamWarm {
        bytes memory callData = abi.encodeCall(ISablierV2Lockup.renounce, defaultStreamId);
        (bool success, bytes memory returnData) = address(lockup).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    modifier whenNotDelegateCalled() {
        _;
    }

    function test_RevertGiven_Null() external whenNotDelegateCalled {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.renounce(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    modifier givenStreamCold() {
        _;
    }

    function test_RevertGiven_StatusDepleted() external whenNotDelegateCalled givenStreamCold {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamDepleted.selector, defaultStreamId));
        lockup.renounce(defaultStreamId);
    }

    function test_RevertGiven_StatusCanceled() external whenNotDelegateCalled givenStreamCold {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        lockup.cancel(defaultStreamId);
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamCanceled.selector, defaultStreamId));
        lockup.renounce(defaultStreamId);
    }

    function test_RevertGiven_StatusSettled() external whenNotDelegateCalled givenStreamCold {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamSettled.selector, defaultStreamId));
        lockup.renounce(defaultStreamId);
    }

    /// @dev This modifier runs the test twice: once with a "PENDING" status, and once with a "STREAMING" status.
    modifier givenStreamWarm() {
        vm.warp({ newTimestamp: getBlockTimestamp() - 1 seconds });
        _;
        vm.warp({ newTimestamp: defaults.START_TIME() });
        defaultStreamId = createDefaultStream();
        _;
    }

    function test_RevertWhen_CallerNotSender() external whenNotDelegateCalled givenStreamWarm {
        // Make Eve the caller in this test.
        resetPrank({ msgSender: users.eve });

        // Run the test.
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2Lockup_Unauthorized.selector, defaultStreamId, users.eve)
        );
        lockup.renounce(defaultStreamId);
    }

    modifier whenCallerSender() {
        _;
    }

    function test_RevertGiven_StreamNotCancelable() external whenNotDelegateCalled givenStreamWarm whenCallerSender {
        // Create the not cancelable stream.
        uint256 notCancelableStreamId = createDefaultStreamNotCancelable();

        // Run the test.
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2Lockup_StreamNotCancelable.selector, notCancelableStreamId)
        );
        lockup.renounce(notCancelableStreamId);
    }

    modifier givenStreamCancelable() {
        _;
    }

    function test_Renounce_RecipientNotContract()
        external
        whenNotDelegateCalled
        givenStreamWarm
        whenCallerSender
        givenStreamCancelable
    {
        lockup.renounce(defaultStreamId);
        bool isCancelable = lockup.isCancelable(defaultStreamId);
        assertFalse(isCancelable, "isCancelable");
    }

    modifier givenRecipientContract() {
        _;
    }

    function test_Renounce_RecipientDoesNotImplementHook()
        external
        whenNotDelegateCalled
        givenStreamWarm
        whenCallerSender
        givenStreamCancelable
        givenRecipientContract
    {
        // Create the stream with a no-op contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(noop));

        // Expect a call to the hook.
        vm.expectCall(address(noop), abi.encodeCall(ISablierV2Recipient.onLockupStreamRenounced, (streamId)));

        // Renounce the stream.
        lockup.renounce(streamId);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(streamId);
        assertFalse(isCancelable, "isCancelable");
    }

    modifier givenRecipientImplementsHook() {
        _;
    }

    function test_Renounce_RecipientReverts()
        external
        whenNotDelegateCalled
        givenStreamWarm
        whenCallerSender
        givenStreamCancelable
        givenRecipientContract
        givenRecipientImplementsHook
    {
        // Create the stream with a reverting contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(revertingRecipient));

        // Expect a call to the hook.
        vm.expectCall(
            address(revertingRecipient), abi.encodeCall(ISablierV2Recipient.onLockupStreamRenounced, (streamId))
        );

        // Renounce the stream.
        lockup.renounce(streamId);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(streamId);
        assertFalse(isCancelable, "isCancelable");
    }

    modifier whenRecipientDoesNotRevert() {
        _;
    }

    function test_Renounce_RecipientReentrancy()
        external
        whenNotDelegateCalled
        givenStreamWarm
        whenCallerSender
        givenStreamCancelable
        givenRecipientContract
        givenRecipientImplementsHook
        whenRecipientDoesNotRevert
    {
        // Create the stream with a reentrant contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(reentrantRecipient));

        // Expect a call to the hook.
        vm.expectCall(
            address(reentrantRecipient), abi.encodeCall(ISablierV2Recipient.onLockupStreamRenounced, (streamId))
        );

        // Renounce the stream.
        lockup.renounce(streamId);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(streamId);
        assertFalse(isCancelable, "isCancelable");
    }

    modifier whenNoRecipientReentrancy() {
        _;
    }

    function test_Renounce()
        external
        whenNotDelegateCalled
        givenStreamWarm
        whenCallerSender
        givenStreamCancelable
        givenRecipientContract
        givenRecipientImplementsHook
        whenRecipientDoesNotRevert
        whenNoRecipientReentrancy
    {
        // Create the stream with a contract as the stream's recipient.
        uint256 streamId = createDefaultStreamWithRecipient(address(goodRecipient));

        // Expect a call to the hook.
        vm.expectCall(address(goodRecipient), abi.encodeCall(ISablierV2Recipient.onLockupStreamRenounced, (streamId)));

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit RenounceLockupStream(streamId);
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: streamId });

        // Renounce the stream.
        lockup.renounce(streamId);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(streamId);
        assertFalse(isCancelable, "isCancelable");
    }
}
