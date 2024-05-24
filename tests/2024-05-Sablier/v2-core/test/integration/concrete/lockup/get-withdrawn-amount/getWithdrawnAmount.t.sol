// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { GetWithdrawnAmount_Integration_Shared_Test } from "../../../shared/lockup/getWithdrawnAmount.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract GetWithdrawnAmount_Integration_Concrete_Test is
    Integration_Test,
    GetWithdrawnAmount_Integration_Shared_Test
{
    function setUp() public virtual override(Integration_Test, GetWithdrawnAmount_Integration_Shared_Test) {
        GetWithdrawnAmount_Integration_Shared_Test.setUp();
    }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.getWithdrawnAmount(nullStreamId);
    }

    function test_GetWithdrawnAmount_NoPreviousWithdrawals() external givenNotNull {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = 0;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }

    function test_GetWithdrawnAmount() external givenNotNull givenPreviousWithdrawals {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });

        // Set the withdraw amount to the streamed amount.
        uint128 withdrawAmount = lockup.streamedAmountOf(defaultStreamId);

        // Make the withdrawal.
        lockup.withdraw({ streamId: defaultStreamId, to: users.recipient, amount: withdrawAmount });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }
}
