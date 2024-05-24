// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";
import { LockupDynamic } from "src/types/DataTypes.sol";

import { LockupDynamic_Integration_Concrete_Test } from "../LockupDynamic.t.sol";

contract GetStream_LockupDynamic_Integration_Concrete_Test is LockupDynamic_Integration_Concrete_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override {
        LockupDynamic_Integration_Concrete_Test.setUp();
        defaultStreamId = createDefaultStream();
    }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockupDynamic.getStream(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_GetStream_StatusSettled() external givenNotNull {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        LockupDynamic.StreamLD memory actualStream = lockupDynamic.getStream(defaultStreamId);
        LockupDynamic.StreamLD memory expectedStream = defaults.lockupDynamicStream();
        expectedStream.isCancelable = false;
        assertEq(actualStream, expectedStream);
    }

    modifier givenStatusNotSettled() {
        _;
    }

    function test_GetStream() external givenNotNull givenStatusNotSettled {
        uint256 streamId = createDefaultStream();
        LockupDynamic.StreamLD memory actualStream = lockupDynamic.getStream(streamId);
        LockupDynamic.StreamLD memory expectedStream = defaults.lockupDynamicStream();
        assertEq(actualStream, expectedStream);
    }
}
