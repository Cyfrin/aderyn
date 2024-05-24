// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";
import { LockupLinear } from "src/types/DataTypes.sol";

import { LockupLinear_Integration_Concrete_Test } from "../LockupLinear.t.sol";

contract GetStream_LockupLinear_Integration_Concrete_Test is LockupLinear_Integration_Concrete_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override {
        LockupLinear_Integration_Concrete_Test.setUp();
        defaultStreamId = createDefaultStream();
    }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockupLinear.getStream(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_GetStream_StatusSettled() external givenNotNull {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        LockupLinear.StreamLL memory actualStream = lockupLinear.getStream(defaultStreamId);
        LockupLinear.StreamLL memory expectedStream = defaults.lockupLinearStream();
        expectedStream.isCancelable = false;
        assertEq(actualStream, expectedStream);
    }

    modifier givenStatusNotSettled() {
        _;
    }

    function test_GetStream() external givenNotNull givenStatusNotSettled {
        LockupLinear.StreamLL memory actualStream = lockupLinear.getStream(defaultStreamId);
        LockupLinear.StreamLL memory expectedStream = defaults.lockupLinearStream();
        assertEq(actualStream, expectedStream);
    }
}
