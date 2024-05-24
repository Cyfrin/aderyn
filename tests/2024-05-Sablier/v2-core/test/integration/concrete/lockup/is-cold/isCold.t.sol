// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract IsCold_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) { }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.isCold(nullStreamId);
    }

    modifier givenNotNull() {
        defaultStreamId = createDefaultStream();
        _;
    }

    function test_IsCold_StatusPending() external givenNotNull {
        vm.warp({ newTimestamp: getBlockTimestamp() - 1 seconds });
        bool isCold = lockup.isCold(defaultStreamId);
        assertFalse(isCold, "isCold");
    }

    function test_IsCold_StatusStreaming() external givenNotNull {
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });
        bool isCold = lockup.isCold(defaultStreamId);
        assertFalse(isCold, "isCold");
    }

    function test_IsCold_StatusSettled() external givenNotNull {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        bool isCold = lockup.isCold(defaultStreamId);
        assertTrue(isCold, "isCold");
    }

    function test_IsCold_StatusCanceled() external givenNotNull {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        lockup.cancel(defaultStreamId);
        bool isCold = lockup.isCold(defaultStreamId);
        assertTrue(isCold, "isCold");
    }

    function test_IsCold_StatusDepleted() external givenNotNull {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });
        bool isCold = lockup.isCold(defaultStreamId);
        assertTrue(isCold, "isCold");
    }
}
