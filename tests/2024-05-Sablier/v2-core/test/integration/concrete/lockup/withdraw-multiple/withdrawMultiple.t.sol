// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Solarray } from "solarray/src/Solarray.sol";

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";
import { Errors } from "src/libraries/Errors.sol";
import { Lockup } from "src/types/DataTypes.sol";

import { WithdrawMultiple_Integration_Shared_Test } from "../../../shared/lockup/withdrawMultiple.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract WithdrawMultiple_Integration_Concrete_Test is
    Integration_Test,
    WithdrawMultiple_Integration_Shared_Test
{
    function setUp() public virtual override(Integration_Test, WithdrawMultiple_Integration_Shared_Test) {
        WithdrawMultiple_Integration_Shared_Test.setUp();
    }

    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData = abi.encodeCall(ISablierV2Lockup.withdrawMultiple, (testStreamIds, testAmounts));
        (bool success, bytes memory returnData) = address(lockup).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    function test_RevertWhen_ArrayCountsNotEqual() external whenNotDelegateCalled {
        uint256[] memory streamIds = new uint256[](2);
        uint128[] memory amounts = new uint128[](1);
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2Lockup_WithdrawArrayCountsNotEqual.selector, streamIds.length, amounts.length
            )
        );
        lockup.withdrawMultiple(streamIds, amounts);
    }

    modifier whenArrayCountsAreEqual() {
        _;
    }

    function test_WithdrawMultiple_ArrayCountsZero() external whenNotDelegateCalled whenArrayCountsAreEqual {
        uint256[] memory streamIds = new uint256[](0);
        uint128[] memory amounts = new uint128[](0);
        lockup.withdrawMultiple(streamIds, amounts);
    }

    modifier whenArrayCountsNotZero() {
        _;
    }

    function test_RevertGiven_OnlyNull()
        external
        whenNotDelegateCalled
        whenArrayCountsAreEqual
        whenArrayCountsNotZero
    {
        uint256 nullStreamId = 1729;
        uint128 withdrawAmount = defaults.WITHDRAW_AMOUNT();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.withdrawMultiple({
            streamIds: Solarray.uint256s(nullStreamId),
            amounts: Solarray.uint128s(withdrawAmount)
        });
    }

    function test_RevertGiven_SomeNull()
        external
        whenNotDelegateCalled
        whenArrayCountsAreEqual
        whenArrayCountsNotZero
    {
        uint256 nullStreamId = 1729;
        uint256[] memory streamIds = Solarray.uint256s(testStreamIds[0], testStreamIds[1], nullStreamId);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Expect the relevant error to be thrown.
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));

        // Withdraw from multiple streams.
        lockup.withdrawMultiple({ streamIds: streamIds, amounts: testAmounts });
    }

    function test_RevertGiven_AllStatusesDepleted()
        external
        whenNotDelegateCalled
        whenArrayCountsAreEqual
        whenArrayCountsNotZero
        givenNoNull
    {
        uint256[] memory streamIds = Solarray.uint256s(testStreamIds[0]);
        uint128[] memory amounts = Solarray.uint128s(defaults.WITHDRAW_AMOUNT());

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.END_TIME() });

        // Deplete the first test stream.
        lockup.withdrawMax({ streamId: testStreamIds[0], to: users.recipient });

        // Expect the relevant error to be thrown.
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamDepleted.selector, testStreamIds[0]));

        // Withdraw from multiple streams.
        lockup.withdrawMultiple(streamIds, amounts);
    }

    function test_RevertGiven_SomeStatusesDepleted()
        external
        whenNotDelegateCalled
        whenArrayCountsAreEqual
        whenArrayCountsNotZero
        givenNoNull
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.END_TIME() });

        // Deplete the first test stream.
        lockup.withdrawMax({ streamId: testStreamIds[0], to: users.recipient });

        // Expect the relevant error to be thrown.
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamDepleted.selector, testStreamIds[0]));

        // Withdraw from multiple streams.
        lockup.withdrawMultiple({ streamIds: testStreamIds, amounts: testAmounts });
    }

    function test_RevertWhen_SomeAmountsZero()
        external
        whenNotDelegateCalled
        whenArrayCountsAreEqual
        whenArrayCountsNotZero
        givenNoNull
        givenNoDepletedStream
        whenToNonZeroAddress
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Run the test.
        uint128[] memory amounts = Solarray.uint128s(defaults.WITHDRAW_AMOUNT(), 0, 0);
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_WithdrawAmountZero.selector, testStreamIds[1]));
        lockup.withdrawMultiple({ streamIds: testStreamIds, amounts: amounts });
    }

    function test_RevertWhen_SomeAmountsOverdraw()
        external
        whenNotDelegateCalled
        whenArrayCountsAreEqual
        whenArrayCountsNotZero
        givenNoNull
        givenNoDepletedStream
        whenToNonZeroAddress
        whenNoAmountZero
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Run the test.
        uint128 withdrawableAmount = lockup.withdrawableAmountOf(testStreamIds[2]);
        uint128[] memory amounts = Solarray.uint128s(testAmounts[0], testAmounts[1], MAX_UINT128);
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2Lockup_Overdraw.selector, testStreamIds[2], MAX_UINT128, withdrawableAmount
            )
        );
        lockup.withdrawMultiple({ streamIds: testStreamIds, amounts: amounts });
    }

    function test_WithdrawMultiple()
        external
        whenNotDelegateCalled
        whenArrayCountsAreEqual
        whenArrayCountsNotZero
        givenNoNull
        givenNoDepletedStream
        whenToNonZeroAddress
        whenNoAmountZero
        whenNoAmountOverdraws
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: earlyStopTime });

        // Cancel the 3rd stream.
        resetPrank({ msgSender: users.sender });
        lockup.cancel(testStreamIds[2]);

        // Run the test with the caller provided in {whenCallerAuthorizedAllStreams}.
        resetPrank({ msgSender: caller });

        // Expect the withdrawals to be made.
        expectCallToTransfer({ to: users.recipient, value: testAmounts[0] });
        expectCallToTransfer({ to: users.recipient, value: testAmounts[1] });
        expectCallToTransfer({ to: users.recipient, value: testAmounts[2] });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream({
            streamId: testStreamIds[0],
            to: users.recipient,
            asset: dai,
            amount: testAmounts[0]
        });
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream({
            streamId: testStreamIds[1],
            to: users.recipient,
            asset: dai,
            amount: testAmounts[1]
        });
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream({
            streamId: testStreamIds[2],
            to: users.recipient,
            asset: dai,
            amount: testAmounts[2]
        });

        // Make the withdrawals.
        lockup.withdrawMultiple({ streamIds: testStreamIds, amounts: testAmounts });

        // Assert that the statuses have been updated.
        assertEq(lockup.statusOf(testStreamIds[0]), Lockup.Status.STREAMING, "status0");
        assertEq(lockup.statusOf(testStreamIds[1]), Lockup.Status.DEPLETED, "status1");
        assertEq(lockup.statusOf(testStreamIds[2]), Lockup.Status.CANCELED, "status2");

        // Assert that the withdrawn amounts have been updated.
        assertEq(lockup.getWithdrawnAmount(testStreamIds[0]), testAmounts[0], "withdrawnAmount0");
        assertEq(lockup.getWithdrawnAmount(testStreamIds[1]), testAmounts[1], "withdrawnAmount1");
        assertEq(lockup.getWithdrawnAmount(testStreamIds[2]), testAmounts[2], "withdrawnAmount2");

        // Assert that the stream NFTs have not been burned.
        assertEq(lockup.getRecipient(testStreamIds[0]), users.recipient, "NFT owner0");
        assertEq(lockup.getRecipient(testStreamIds[1]), users.recipient, "NFT owner1");
        assertEq(lockup.getRecipient(testStreamIds[2]), users.recipient, "NFT owner2");
    }
}
