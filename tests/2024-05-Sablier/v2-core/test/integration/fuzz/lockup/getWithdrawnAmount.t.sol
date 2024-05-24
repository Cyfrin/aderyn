// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { GetWithdrawnAmount_Integration_Shared_Test } from "../../shared/lockup/getWithdrawnAmount.t.sol";
import { Integration_Test } from "../../Integration.t.sol";

abstract contract GetWithdrawnAmount_Integration_Fuzz_Test is
    Integration_Test,
    GetWithdrawnAmount_Integration_Shared_Test
{
    function setUp() public virtual override(Integration_Test, GetWithdrawnAmount_Integration_Shared_Test) {
        GetWithdrawnAmount_Integration_Shared_Test.setUp();
    }

    function testFuzz_GetWithdrawnAmount_NoPreviousWithdrawals(uint256 timeJump) external givenNotNull {
        timeJump = _bound(timeJump, 0 seconds, defaults.TOTAL_DURATION() * 2);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = 0;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    function testFuzz_GetWithdrawnAmount(
        uint256 timeJump,
        uint128 withdrawAmount
    )
        external
        givenNotNull
        givenPreviousWithdrawals
    {
        timeJump = _bound(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() - 1 seconds);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Bound the withdraw amount.
        uint128 streamedAmount = lockup.streamedAmountOf(defaultStreamId);
        withdrawAmount = boundUint128(withdrawAmount, 1, streamedAmount);

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: withdrawAmount });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }
}
