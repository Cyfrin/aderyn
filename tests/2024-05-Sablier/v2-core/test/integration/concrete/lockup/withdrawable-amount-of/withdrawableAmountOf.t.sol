// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { WithdrawableAmountOf_Integration_Shared_Test } from "../../../shared/lockup/withdrawableAmountOf.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract WithdrawableAmountOf_Integration_Concrete_Test is
    Integration_Test,
    WithdrawableAmountOf_Integration_Shared_Test
{
    function setUp() public virtual override(Integration_Test, WithdrawableAmountOf_Integration_Shared_Test) {
        WithdrawableAmountOf_Integration_Shared_Test.setUp();
    }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.withdrawableAmountOf(nullStreamId);
    }

    function test_WithdrawableAmountOf_StreamHasBeenCanceled_StatusCanceled()
        external
        givenNotNull
        givenStreamHasBeenCanceled
    {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        lockup.cancel(defaultStreamId);
        uint128 actualWithdrawableAmount = lockup.withdrawableAmountOf(defaultStreamId);
        uint256 expectedWithdrawableAmount = defaults.CLIFF_AMOUNT();
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    /// @dev This test warps a second time to ensure that {withdrawableAmountOf} ignores the current time.
    function test_WithdrawableAmountOf_StreamHasBeenCanceled_StatusDepleted()
        external
        givenNotNull
        givenStreamHasBeenCanceled
    {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        lockup.cancel(defaultStreamId);
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() + 10 seconds });
        uint128 actualWithdrawableAmount = lockup.withdrawableAmountOf(defaultStreamId);
        uint128 expectedWithdrawableAmount = 0;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    function test_WithdrawableAmountOf_StatusPending() external givenNotNull givenStreamHasNotBeenCanceled {
        vm.warp({ newTimestamp: getBlockTimestamp() - 1 seconds });
        uint128 actualWithdrawableAmount = lockup.withdrawableAmountOf(defaultStreamId);
        uint128 expectedWithdrawableAmount = 0;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    function test_WithdrawableAmountOf_StatusSettled() external givenNotNull givenStreamHasNotBeenCanceled {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        uint128 actualWithdrawableAmount = lockup.withdrawableAmountOf(defaultStreamId);
        uint128 expectedWithdrawableAmount = defaults.DEPOSIT_AMOUNT();
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }

    function test_WithdrawableAmountOf_StatusDepleted() external givenNotNull givenStreamHasNotBeenCanceled {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        uint128 actualWithdrawableAmount = lockup.withdrawableAmountOf(defaultStreamId);
        uint128 expectedWithdrawableAmount = 0;
        assertEq(actualWithdrawableAmount, expectedWithdrawableAmount, "withdrawableAmount");
    }
}
