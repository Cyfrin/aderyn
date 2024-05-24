// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup } from "src/types/DataTypes.sol";

import { Withdraw_Integration_Shared_Test } from "../../shared/lockup/withdraw.t.sol";
import { Integration_Test } from "../../Integration.t.sol";

abstract contract Withdraw_Integration_Fuzz_Test is Integration_Test, Withdraw_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, Withdraw_Integration_Shared_Test) {
        Withdraw_Integration_Shared_Test.setUp();
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - Multiple caller addresses.
    function testFuzz_Withdraw_UnknownCaller(address caller)
        external
        whenNotDelegateCalled
        givenNotNull
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
    {
        vm.assume(caller != users.sender && caller != users.recipient);

        // Make the fuzzed address the caller in this test.
        resetPrank({ msgSender: caller });

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: defaults.WITHDRAW_AMOUNT() });

        // Assert that the stream's status is still "STREAMING".
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = defaults.WITHDRAW_AMOUNT();
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - Multiple values for the withdrawal address.
    function testFuzz_Withdraw_CallerApprovedOperator(address to)
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamNotDepleted
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
    {
        vm.assume(to != address(0));

        // Approve the operator to handle the stream.
        lockup.approve({ to: users.operator, tokenId: defaultStreamId });

        // Make the operator the caller in this test.
        resetPrank({ msgSender: users.operator });

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: to, amount: defaults.WITHDRAW_AMOUNT() });

        // Assert that the stream's status is still "STREAMING".
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = defaults.WITHDRAW_AMOUNT();
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - Multiple values for the block timestamp.
    /// - Multiple values for the withdrawal address.
    /// - Multiple withdraw amounts.
    function testFuzz_Withdraw_StreamHasBeenCanceled(
        uint256 timeJump,
        address to,
        uint128 withdrawAmount
    )
        external
        whenNotDelegateCalled
        givenNotNull
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenCallerRecipient
    {
        timeJump = _bound(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() - 1 seconds);
        vm.assume(to != address(0));

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Cancel the stream.
        resetPrank({ msgSender: users.sender });
        lockup.cancel({ streamId: defaultStreamId });
        resetPrank({ msgSender: users.recipient });

        // Bound the withdraw amount.
        uint128 withdrawableAmount = lockup.withdrawableAmountOf(defaultStreamId);
        withdrawAmount = boundUint128(withdrawAmount, 1, withdrawableAmount);

        // Expect the assets to be transferred to the fuzzed `to` address.
        expectCallToTransfer({ to: to, value: withdrawAmount });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream(defaultStreamId, to, dai, withdrawAmount);
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: defaultStreamId });

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: to, amount: withdrawAmount });

        // Check if the stream has been depleted.
        uint128 refundedAmount = lockup.getRefundedAmount(defaultStreamId);
        bool isDepleted = withdrawAmount == defaults.DEPOSIT_AMOUNT() - refundedAmount;

        // Assert that the stream's status is correct.
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = isDepleted ? Lockup.Status.DEPLETED : Lockup.Status.CANCELED;
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

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - End time in the past
    /// - End time in the present
    /// - End time in the future
    /// - Multiple values for the withdrawal address
    /// - Multiple withdraw amounts
    function testFuzz_Withdraw(
        uint256 timeJump,
        address to,
        uint128 withdrawAmount
    )
        external
        whenNotDelegateCalled
        givenNotNull
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
        whenStreamHasNotBeenCanceled
    {
        timeJump = _bound(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() * 2);
        vm.assume(to != address(0));

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Bound the withdraw amount.
        uint128 withdrawableAmount = lockup.withdrawableAmountOf(defaultStreamId);
        withdrawAmount = boundUint128(withdrawAmount, 1, withdrawableAmount);

        // Expect the assets to be transferred to the fuzzed `to` address.
        expectCallToTransfer({ to: to, value: withdrawAmount });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream(defaultStreamId, to, dai, withdrawAmount);
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: defaultStreamId });

        // Make the withdrawal.
        lockup.withdraw(defaultStreamId, to, withdrawAmount);

        // Check if the stream is depleted or settled. It is possible for the stream to be just settled
        // and not depleted because the withdraw amount is fuzzed.
        bool isDepleted = withdrawAmount == defaults.DEPOSIT_AMOUNT();
        bool isSettled = lockup.refundableAmountOf(defaultStreamId) == 0;

        // Assert that the stream's status is correct.
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus;
        if (isDepleted) {
            expectedStatus = Lockup.Status.DEPLETED;
        } else if (isSettled) {
            expectedStatus = Lockup.Status.SETTLED;
        } else {
            expectedStatus = Lockup.Status.STREAMING;
        }
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
}
