// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { LockupDynamic } from "src/types/DataTypes.sol";

import { LockupDynamic_Integration_Concrete_Test } from "../LockupDynamic.t.sol";
import { StreamedAmountOf_Integration_Concrete_Test } from "../../lockup/streamed-amount-of/streamedAmountOf.t.sol";

contract StreamedAmountOf_LockupDynamic_Integration_Concrete_Test is
    LockupDynamic_Integration_Concrete_Test,
    StreamedAmountOf_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Concrete_Test, StreamedAmountOf_Integration_Concrete_Test)
    {
        LockupDynamic_Integration_Concrete_Test.setUp();
        StreamedAmountOf_Integration_Concrete_Test.setUp();
    }

    function test_StreamedAmountOf_StartTimeInTheFuture()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
    {
        vm.warp({ newTimestamp: 0 });
        uint128 actualStreamedAmount = lockupDynamic.streamedAmountOf(defaultStreamId);
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
        uint128 actualStreamedAmount = lockupDynamic.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = 0;
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    function test_StreamedAmountOf_OneSegment()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
        whenStartTimeInThePast
    {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + 2000 seconds });

        // Create an array with one segment.
        LockupDynamic.Segment[] memory segments = new LockupDynamic.Segment[](1);
        segments[0] = LockupDynamic.Segment({
            amount: defaults.DEPOSIT_AMOUNT(),
            exponent: defaults.segments()[1].exponent,
            timestamp: defaults.END_TIME()
        });

        // Create the stream.
        uint256 streamId = createDefaultStreamWithSegments(segments);

        // Run the test.
        uint128 actualStreamedAmount = lockupDynamic.streamedAmountOf(streamId);
        uint128 expectedStreamedAmount = 4472.13595499957941e18; // (0.2^0.5)*10,000
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    modifier givenMultipleSegments() {
        _;
    }

    function test_StreamedAmountOf_CurrentTimestamp1st()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
        givenMultipleSegments
        whenStartTimeInThePast
    {
        // Warp 1 second to the future.
        vm.warp({ newTimestamp: defaults.START_TIME() + 1 seconds });

        // Run the test.
        uint128 actualStreamedAmount = lockupDynamic.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = 0.000000053506725e18;
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    modifier givenCurrentTimestampNot1st() {
        _;
    }

    function test_StreamedAmountOf_CurrentTimestampNot1st()
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        givenStatusStreaming
        whenStartTimeInThePast
        givenMultipleSegments
        givenCurrentTimestampNot1st
    {
        // Simulate the passage of time. 750 seconds is ~10% of the way in the second segment.
        vm.warp({ newTimestamp: defaults.START_TIME() + defaults.CLIFF_DURATION() + 750 seconds });

        // Run the test.
        uint128 actualStreamedAmount = lockupDynamic.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = defaults.segments()[0].amount + 2371.708245126284505e18; // ~7,500*0.1^{0.5}
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }
}
