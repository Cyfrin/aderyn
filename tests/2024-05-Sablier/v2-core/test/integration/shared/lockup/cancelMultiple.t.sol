// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup_Integration_Shared_Test } from "./Lockup.t.sol";

abstract contract CancelMultiple_Integration_Shared_Test is Lockup_Integration_Shared_Test {
    uint40 internal originalTime;
    uint256[] internal testStreamIds;

    function setUp() public virtual override {
        originalTime = getBlockTimestamp();
        createTestStreams();
    }

    /// @dev Creates the default streams used throughout the tests.
    function createTestStreams() internal {
        // Warp back to the original timestamp.
        vm.warp({ newTimestamp: originalTime });

        // Create the test streams.
        testStreamIds = new uint256[](2);
        testStreamIds[0] = createDefaultStream();
        // Create a stream with an end time double that of the default stream so that the refund amounts are different.
        testStreamIds[1] = createDefaultStreamWithEndTime(defaults.END_TIME() + defaults.TOTAL_DURATION());
    }

    modifier givenAllStreamsCancelable() {
        _;
    }

    modifier givenAllStreamsWarm() {
        _;
    }

    modifier givenNoNull() {
        _;
    }

    modifier whenArrayCountNotZero() {
        _;
    }

    modifier whenCallerAuthorizedAllStreams() {
        _;
        vm.warp({ newTimestamp: originalTime });
        createTestStreams();
        resetPrank({ msgSender: users.sender });
        _;
    }

    modifier whenCallerUnauthorized() {
        _;
    }

    modifier whenNotDelegateCalled() {
        _;
    }
}
