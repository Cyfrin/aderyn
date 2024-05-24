// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ZERO } from "@prb/math/src/UD60x18.sol";

import { Broker, LockupLinear } from "src/types/DataTypes.sol";

import { LockupLinear_Integration_Fuzz_Test } from "./LockupLinear.t.sol";
import { WithdrawableAmountOf_Integration_Shared_Test } from "../../shared/lockup/withdrawableAmountOf.t.sol";

contract WithdrawableAmountOf_LockupLinear_Integration_Fuzz_Test is
    LockupLinear_Integration_Fuzz_Test,
    WithdrawableAmountOf_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupLinear_Integration_Fuzz_Test, WithdrawableAmountOf_Integration_Shared_Test)
    {
        LockupLinear_Integration_Fuzz_Test.setUp();
        WithdrawableAmountOf_Integration_Shared_Test.setUp();
    }

    function testFuzz_WithdrawableAmountOf_CliffTimeInTheFuture(uint40 timeJump)
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
    {
        timeJump = boundUint40(timeJump, 0, defaults.CLIFF_DURATION() - 1);
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });
        uint128 actualWithdrawableAmount = lockupLinear.withdrawableAmountOf(defaultStreamId);
        uint128 expectedWithdrawableAmount = 0;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    modifier whenCliffTimeNotInTheFuture() {
        resetPrank({ msgSender: users.sender });
        _;
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - End time in the past
    /// - End time in the present
    /// - End time in the future
    /// - Status streaming
    /// - Status settled
    function testFuzz_WithdrawableAmountOf_NoPreviousWithdrawals(
        uint40 timeJump,
        uint128 depositAmount
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenCliffTimeNotInTheFuture
    {
        vm.assume(depositAmount != 0);
        timeJump = boundUint40(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() * 2);

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: depositAmount });

        // Create the stream. The broker fee is disabled so that it doesn't interfere with the calculations.
        LockupLinear.CreateWithTimestamps memory params = defaults.createWithTimestampsLL();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.totalAmount = depositAmount;
        uint256 streamId = lockupLinear.createWithTimestamps(params);

        // Simulate the passage of time.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;
        vm.warp({ newTimestamp: blockTimestamp });

        // Run the test.
        uint128 actualWithdrawableAmount = lockupLinear.withdrawableAmountOf(streamId);
        uint128 expectedWithdrawableAmount = calculateStreamedAmount(blockTimestamp, depositAmount);
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    modifier givenPreviousWithdrawals() {
        _;
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - End time in the past
    /// - End time in the present
    /// - End time in the future
    /// - Multiple deposit amounts
    /// - Multiple withdraw amounts
    /// - Status streaming
    /// - Status settled
    /// - Status depleted
    /// - Withdraw amount equal to deposited amount and not
    function testFuzz_WithdrawableAmountOf(
        uint40 timeJump,
        uint128 depositAmount,
        uint128 withdrawAmount
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenCliffTimeNotInTheFuture
        givenPreviousWithdrawals
    {
        timeJump = boundUint40(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() * 2);
        depositAmount = boundUint128(depositAmount, 10_000, MAX_UINT128);

        // Define the block timestamp.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;

        // Bound the withdraw amount.
        uint128 streamedAmount = calculateStreamedAmount(blockTimestamp, depositAmount);
        withdrawAmount = boundUint128(withdrawAmount, 1, streamedAmount);

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: depositAmount });

        // Create the stream. The broker fee is disabled so that it doesn't interfere with the calculations.
        LockupLinear.CreateWithTimestamps memory params = defaults.createWithTimestampsLL();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.totalAmount = depositAmount;
        uint256 streamId = lockupLinear.createWithTimestamps(params);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: blockTimestamp });

        // Make the withdrawal.
        lockupLinear.withdraw({ streamId: streamId, to: users.recipient, amount: withdrawAmount });

        // Run the test.
        uint128 actualWithdrawableAmount = lockupLinear.withdrawableAmountOf(streamId);
        uint128 expectedWithdrawableAmount = streamedAmount - withdrawAmount;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }
}
