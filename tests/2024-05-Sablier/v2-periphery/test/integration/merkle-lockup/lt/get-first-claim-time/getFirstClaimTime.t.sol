// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { MerkleLockup_Integration_Test } from "../../MerkleLockup.t.sol";

contract GetFirstClaimTime_Integration_Test is MerkleLockup_Integration_Test {
    function setUp() public virtual override {
        MerkleLockup_Integration_Test.setUp();
    }

    function test_GetFirstClaimTime_BeforeFirstClaim() external view {
        uint256 firstClaimTime = merkleLT.getFirstClaimTime();
        assertEq(firstClaimTime, 0);
    }

    modifier afterFirstClaim() {
        // Make the first claim to set `_firstClaimTime`.
        claimLT();
        _;
    }

    function test_GetFirstClaimTime() external afterFirstClaim {
        uint256 firstClaimTime = merkleLT.getFirstClaimTime();
        assertEq(firstClaimTime, block.timestamp);
    }
}
