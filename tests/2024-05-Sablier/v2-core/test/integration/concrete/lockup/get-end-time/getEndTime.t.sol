// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract GetEndTime_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) { }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.getEndTime(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_GetEndTime() external givenNotNull {
        uint256 streamId = createDefaultStream();
        uint40 actualEndTime = lockup.getEndTime(streamId);
        uint40 expectedEndTime = defaults.END_TIME();
        assertEq(actualEndTime, expectedEndTime, "endTime");
    }
}
