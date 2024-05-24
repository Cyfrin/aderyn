// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";
import { Lockup } from "src/types/DataTypes.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract StatusOf_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) { }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.statusOf(nullStreamId);
    }

    modifier givenNotNull() {
        defaultStreamId = createDefaultStream();
        _;
    }

    function test_StatusOf_AssetsFullyWithdrawn() external givenNotNull {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.DEPLETED;
        assertEq(actualStatus, expectedStatus);
    }

    modifier givenAssetsNotFullyWithdrawn() {
        _;
    }

    function test_StatusOf_StreamCanceled() external givenNotNull givenAssetsNotFullyWithdrawn {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        lockup.cancel(defaultStreamId);
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.CANCELED;
        assertEq(actualStatus, expectedStatus);
    }

    modifier givenStreamNotCanceled() {
        _;
    }

    function test_StatusOf_StartTimeInTheFuture()
        external
        givenNotNull
        givenAssetsNotFullyWithdrawn
        givenStreamNotCanceled
    {
        vm.warp({ newTimestamp: getBlockTimestamp() - 1 seconds });
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.PENDING;
        assertEq(actualStatus, expectedStatus);
    }

    modifier givenStartTimeNotInTheFuture() {
        _;
    }

    function test_StatusOf_RefundableAmountNotZero()
        external
        givenNotNull
        givenAssetsNotFullyWithdrawn
        givenStreamNotCanceled
        givenStartTimeNotInTheFuture
    {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.SETTLED;
        assertEq(actualStatus, expectedStatus);
    }

    modifier givenRefundableAmountNotZero() {
        _;
    }

    function test_StatusOf()
        external
        givenNotNull
        givenAssetsNotFullyWithdrawn
        givenStreamNotCanceled
        givenStartTimeNotInTheFuture
        givenRefundableAmountNotZero
    {
        vm.warp({ newTimestamp: defaults.START_TIME() + 1 seconds });
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);
    }
}
