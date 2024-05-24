// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { LockupLinear_Integration_Concrete_Test } from "../LockupLinear.t.sol";

contract GetCliffTime_LockupLinear_Integration_Concrete_Test is LockupLinear_Integration_Concrete_Test {
    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockupLinear.getCliffTime(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_GetCliffTime() external givenNotNull {
        uint256 streamId = createDefaultStream();
        uint40 actualCliffTime = lockupLinear.getCliffTime(streamId);
        uint40 expectedCliffTime = defaults.CLIFF_TIME();
        assertEq(actualCliffTime, expectedCliffTime, "cliffTime");
    }
}
