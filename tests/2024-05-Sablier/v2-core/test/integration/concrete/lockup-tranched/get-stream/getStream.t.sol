// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";
import { LockupTranched } from "src/types/DataTypes.sol";

import { LockupTranched_Integration_Concrete_Test } from "../LockupTranched.t.sol";

contract GetStream_LockupTranched_Integration_Concrete_Test is LockupTranched_Integration_Concrete_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override {
        LockupTranched_Integration_Concrete_Test.setUp();
        defaultStreamId = createDefaultStream();
    }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockupTranched.getStream(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_GetStream_StatusSettled() external givenNotNull {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        LockupTranched.StreamLT memory actualStream = lockupTranched.getStream(defaultStreamId);
        LockupTranched.StreamLT memory expectedStream = defaults.lockupTranchedStream();
        expectedStream.isCancelable = false;
        assertEq(actualStream, expectedStream);
    }

    modifier givenStatusNotSettled() {
        _;
    }

    function test_GetStream() external givenNotNull givenStatusNotSettled {
        uint256 streamId = createDefaultStream();
        LockupTranched.StreamLT memory actualStream = lockupTranched.getStream(streamId);
        LockupTranched.StreamLT memory expectedStream = defaults.lockupTranchedStream();
        assertEq(actualStream, expectedStream);
    }
}
