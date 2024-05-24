// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";
import { LockupTranched } from "src/types/DataTypes.sol";

import { LockupTranched_Integration_Concrete_Test } from "../LockupTranched.t.sol";

contract GetTimestamps_LockupTranched_Integration_Concrete_Test is LockupTranched_Integration_Concrete_Test {
    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockupTranched.getTimestamps(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_GetTimestamps() external givenNotNull {
        uint256 streamId = createDefaultStream();
        LockupTranched.Timestamps memory actualTimestamps = lockupTranched.getTimestamps(streamId);
        LockupTranched.Timestamps memory expectedTimestamps = defaults.lockupTranchedTimestamps();
        assertEq(actualTimestamps, expectedTimestamps);
    }
}
