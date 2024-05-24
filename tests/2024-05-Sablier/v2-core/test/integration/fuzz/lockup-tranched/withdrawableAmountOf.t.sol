// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ZERO } from "@prb/math/src/UD60x18.sol";

import { Broker, LockupTranched } from "src/types/DataTypes.sol";

import { LockupTranched_Integration_Fuzz_Test } from "./LockupTranched.t.sol";
import { WithdrawableAmountOf_Integration_Shared_Test } from "../../shared/lockup/withdrawableAmountOf.t.sol";

contract WithdrawableAmountOf_LockupTranched_Integration_Fuzz_Test is
    LockupTranched_Integration_Fuzz_Test,
    WithdrawableAmountOf_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Fuzz_Test, WithdrawableAmountOf_Integration_Shared_Test)
    {
        LockupTranched_Integration_Fuzz_Test.setUp();
        WithdrawableAmountOf_Integration_Shared_Test.setUp();

        resetPrank({ msgSender: users.sender });
    }

    modifier whenStartTimeInThePast() {
        _;
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - End time in the past
    /// - End time in the present
    /// - End time in the future
    /// - Status streaming
    /// - Status settled
    function testFuzz_WithdrawableAmountOf_NoPreviousWithdrawals(uint40 timeJump) external whenStartTimeInThePast {
        timeJump = boundUint40(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() * 2);

        // Create the stream with a custom total amount. The broker fee is disabled so that it doesn't interfere with
        // the calculations.
        LockupTranched.CreateWithTimestamps memory params = defaults.createWithTimestampsLT();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.totalAmount = defaults.DEPOSIT_AMOUNT();
        uint256 streamId = lockupTranched.createWithTimestamps(params);

        // Simulate the passage of time.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;
        vm.warp({ newTimestamp: blockTimestamp });

        // Run the test.
        uint128 actualWithdrawableAmount = lockupTranched.withdrawableAmountOf(streamId);
        uint128 expectedWithdrawableAmount =
            calculateStreamedAmountForTranches(blockTimestamp, defaults.tranches(), defaults.DEPOSIT_AMOUNT());
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    modifier whenWithWithdrawals() {
        _;
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - End time in the past
    /// - End time in the present
    /// - End time in the future
    /// - Multiple withdraw amounts
    /// - Status streaming
    /// - Status settled
    /// - Status depleted
    /// - Withdraw amount equal to deposited amount and not
    function testFuzz_WithdrawableAmountOf(
        uint40 timeJump,
        uint128 withdrawAmount
    )
        external
        whenStartTimeInThePast
        whenWithWithdrawals
    {
        timeJump = boundUint40(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() * 2);

        // Define the block timestamp.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;

        // Bound the withdraw amount.
        uint128 streamedAmount =
            calculateStreamedAmountForTranches(blockTimestamp, defaults.tranches(), defaults.DEPOSIT_AMOUNT());
        withdrawAmount = boundUint128(withdrawAmount, 1, streamedAmount);

        // Create the stream with a custom total amount. The broker fee is disabled so that it doesn't interfere with
        // the calculations.
        LockupTranched.CreateWithTimestamps memory params = defaults.createWithTimestampsLT();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.totalAmount = defaults.DEPOSIT_AMOUNT();
        uint256 streamId = lockupTranched.createWithTimestamps(params);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: blockTimestamp });

        // Make the withdrawal.
        lockupTranched.withdraw({ streamId: streamId, to: users.recipient, amount: withdrawAmount });

        // Run the test.
        uint128 actualWithdrawableAmount = lockupTranched.withdrawableAmountOf(streamId);
        uint128 expectedWithdrawableAmount = streamedAmount - withdrawAmount;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }
}
