// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract IsDepleted_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) { }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.isDepleted(nullStreamId);
    }

    modifier givenNotNull() {
        defaultStreamId = createDefaultStream();
        _;
    }

    function test_IsDepleted_StreamNotDepleted() external givenNotNull {
        bool isDepleted = lockup.isDepleted(defaultStreamId);
        assertFalse(isDepleted, "isDepleted");
    }

    modifier givenStreamDepleted() {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        _;
    }

    function test_IsDepleted() external givenNotNull givenStreamDepleted {
        bool isDepleted = lockup.isDepleted(defaultStreamId);
        assertTrue(isDepleted, "isDepleted");
    }
}
