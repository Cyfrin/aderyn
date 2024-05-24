// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";
import { LockupDynamic } from "src/types/DataTypes.sol";

import { LockupDynamic_Integration_Concrete_Test } from "../LockupDynamic.t.sol";

contract GetSegments_LockupDynamic_Integration_Concrete_Test is LockupDynamic_Integration_Concrete_Test {
    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockupDynamic.getSegments(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_GetSegments() external givenNotNull {
        uint256 streamId = createDefaultStream();
        LockupDynamic.Segment[] memory actualSegments = lockupDynamic.getSegments(streamId);
        LockupDynamic.Segment[] memory expectedSegments = defaults.segments();
        assertEq(actualSegments, expectedSegments, "segments");
    }
}
