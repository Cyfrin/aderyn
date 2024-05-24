// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { LockupTranched_Integration_Concrete_Test } from "../LockupTranched.t.sol";
import { StreamedAmountOf_Integration_Concrete_Test } from "../../lockup/streamed-amount-of/streamedAmountOf.t.sol";

contract StreamedAmountOf_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    StreamedAmountOf_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, StreamedAmountOf_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        StreamedAmountOf_Integration_Concrete_Test.setUp();
    }

    function test_StreamedAmountOf_StartTimeInTheFuture()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
    {
        vm.warp({ newTimestamp: 0 });
        uint128 actualStreamedAmount = lockupTranched.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = 0;
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    function test_StreamedAmountOf_StartTimeInThePresent()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
    {
        vm.warp({ newTimestamp: defaults.START_TIME() });
        uint128 actualStreamedAmount = lockupTranched.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = 0;
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    modifier givenMultipleTranches() {
        _;
    }

    function test_StreamedAmountOf_CurrentTimestamp1st()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
        givenMultipleTranches
        whenStartTimeInThePast
    {
        // Warp 1 second to the future.
        vm.warp({ newTimestamp: defaults.START_TIME() + 1 seconds });

        // Run the test.
        uint128 actualStreamedAmount = lockupTranched.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = 0;
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    modifier givenCurrentTimestampNot1st() {
        _;
    }

    function test_StreamedAmountOf()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
        whenStartTimeInThePast
        givenMultipleTranches
        givenCurrentTimestampNot1st
    {
        vm.warp({ newTimestamp: defaults.END_TIME() - 1 seconds });

        // Run the test.
        uint128 actualStreamedAmount = lockupTranched.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = defaults.tranches()[0].amount + defaults.tranches()[1].amount;
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }
}
