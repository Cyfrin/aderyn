// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract RefundableAmountOf_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) { }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.refundableAmountOf(nullStreamId);
    }

    modifier givenNotNull() {
        defaultStreamId = createDefaultStream();
        _;
    }

    function test_RefundableAmountOf_StreamNotCancelable() external givenNotNull {
        uint256 streamId = createDefaultStreamNotCancelable();
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        uint128 actualRefundableAmount = lockup.refundableAmountOf(streamId);
        uint128 expectedRefundableAmount = 0;
        assertEq(actualRefundableAmount, expectedRefundableAmount, "refundableAmount");
    }

    modifier givenStreamIsCancelable() {
        _;
    }

    modifier givenStreamHasBeenCanceled() {
        _;
    }

    function test_RefundableAmountOf_StreamHasBeenCanceled_StatusCanceled()
        external
        givenNotNull
        givenStreamIsCancelable
        givenStreamHasBeenCanceled
    {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        lockup.cancel(defaultStreamId);
        uint128 actualRefundableAmount = lockup.refundableAmountOf(defaultStreamId);
        uint128 expectedRefundableAmount = 0;
        assertEq(actualRefundableAmount, expectedRefundableAmount, "refundableAmount");
    }

    /// @dev This test warps a second time to ensure that {refundableAmountOf} ignores the current time.
    function test_RefundableAmountOf_StreamHasBeenCanceled_StatusDepleted()
        external
        givenNotNull
        givenStreamIsCancelable
        givenStreamHasBeenCanceled
    {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        lockup.cancel(defaultStreamId);
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() + 10 seconds });
        uint128 actualRefundableAmount = lockup.refundableAmountOf(defaultStreamId);
        uint128 expectedRefundableAmount = 0;
        assertEq(actualRefundableAmount, expectedRefundableAmount, "refundableAmount");
    }

    modifier givenStreamHasNotBeenCanceled() {
        _;
    }

    function test_RefundableAmountOf_StatusPending()
        external
        givenNotNull
        givenStreamIsCancelable
        givenStreamHasNotBeenCanceled
    {
        vm.warp({ newTimestamp: getBlockTimestamp() - 1 seconds });
        uint128 actualRefundableAmount = lockup.refundableAmountOf(defaultStreamId);
        uint128 expectedReturnableAmount = defaults.DEPOSIT_AMOUNT();
        assertEq(actualRefundableAmount, expectedReturnableAmount, "refundableAmount");
    }

    function test_RefundableAmountOf_StatusStreaming()
        external
        givenNotNull
        givenStreamIsCancelable
        givenStreamHasNotBeenCanceled
    {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        uint128 actualRefundableAmount = lockup.refundableAmountOf(defaultStreamId);
        uint128 expectedReturnableAmount = defaults.REFUND_AMOUNT();
        assertEq(actualRefundableAmount, expectedReturnableAmount, "refundableAmount");
    }

    function test_RefundableAmountOf_StatusSettled()
        external
        givenNotNull
        givenStreamIsCancelable
        givenStreamHasNotBeenCanceled
    {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        uint128 actualRefundableAmount = lockup.refundableAmountOf(defaultStreamId);
        uint128 expectedReturnableAmount = 0;
        assertEq(actualRefundableAmount, expectedReturnableAmount, "refundableAmount");
    }

    function test_RefundableAmountOf_StatusDepleted()
        external
        givenNotNull
        givenStreamIsCancelable
        givenStreamHasNotBeenCanceled
    {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        uint128 actualRefundableAmount = lockup.refundableAmountOf(defaultStreamId);
        uint128 expectedReturnableAmount = 0;
        assertEq(actualRefundableAmount, expectedReturnableAmount, "refundableAmount");
    }
}
