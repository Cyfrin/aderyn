// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract IsStream_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) { }

    function test_IsStream_Null() external view {
        uint256 nullStreamId = 1729;
        bool isStream = lockup.isStream(nullStreamId);
        assertFalse(isStream, "isStream");
    }

    modifier givenNotNull() {
        defaultStreamId = createDefaultStream();
        _;
    }

    function test_IsStream() external givenNotNull {
        bool isStream = lockup.isStream(defaultStreamId);
        assertTrue(isStream, "isStream");
    }
}
