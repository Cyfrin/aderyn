// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { LockupDynamic_Integration_Concrete_Test } from "../LockupDynamic.t.sol";
import { WithdrawableAmountOf_Integration_Concrete_Test } from
    "../../lockup/withdrawable-amount-of/withdrawableAmountOf.t.sol";

contract WithdrawableAmountOf_LockupDynamic_Integration_Concrete_Test is
    LockupDynamic_Integration_Concrete_Test,
    WithdrawableAmountOf_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Concrete_Test, WithdrawableAmountOf_Integration_Concrete_Test)
    {
        LockupDynamic_Integration_Concrete_Test.setUp();
        WithdrawableAmountOf_Integration_Concrete_Test.setUp();
    }

    function test_WithdrawableAmountOf_StartTimeInThePresent()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
    {
        vm.warp({ newTimestamp: defaults.START_TIME() });
        uint128 actualWithdrawableAmount = lockupDynamic.withdrawableAmountOf(defaultStreamId);
        uint128 expectedWithdrawableAmount = 0;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    modifier givenStartTimeInThePast() {
        _;
    }

    function test_WithdrawableAmountOf_NoPreviousWithdrawals()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
        givenStartTimeInThePast
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + defaults.CLIFF_DURATION() + 3750 seconds });

        // Run the test.
        uint128 actualWithdrawableAmount = lockupDynamic.withdrawableAmountOf(defaultStreamId);
        // The second term is 7,500*0.5^{0.5}
        uint128 expectedWithdrawableAmount = defaults.segments()[0].amount + 5303.30085889910643e18;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    modifier whenWithWithdrawals() {
        _;
    }

    function test_WithdrawableAmountOf()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
        givenStartTimeInThePast
        whenWithWithdrawals
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + defaults.CLIFF_DURATION() + 3750 seconds });

        // Make the withdrawal.
        lockupDynamic.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: defaults.WITHDRAW_AMOUNT() });

        // Run the test.
        uint128 actualWithdrawableAmount = lockupDynamic.withdrawableAmountOf(defaultStreamId);

        // The second term is 7,500*0.5^{0.5}
        uint128 expectedWithdrawableAmount =
            defaults.segments()[0].amount + 5303.30085889910643e18 - defaults.WITHDRAW_AMOUNT();
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }
}
