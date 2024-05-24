// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ZERO } from "@prb/math/src/UD60x18.sol";
import { Broker, LockupDynamic } from "src/types/DataTypes.sol";

import { StreamedAmountOf_Integration_Shared_Test } from "../../shared/lockup/streamedAmountOf.t.sol";
import { LockupDynamic_Integration_Fuzz_Test } from "./LockupDynamic.t.sol";

contract StreamedAmountOf_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    StreamedAmountOf_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Fuzz_Test, StreamedAmountOf_Integration_Shared_Test)
    {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        StreamedAmountOf_Integration_Shared_Test.setUp();

        resetPrank({ msgSender: users.sender });
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - End time in the past
    /// - End time in the present
    /// - End time in the future
    /// - Status streaming
    /// - Status settled
    function testFuzz_StreamedAmountOf_OneSegment(
        LockupDynamic.Segment memory segment,
        uint40 timeJump
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenStartTimeInThePast
    {
        vm.assume(segment.amount != 0);
        segment.timestamp = boundUint40(segment.timestamp, defaults.CLIFF_TIME(), defaults.END_TIME());
        timeJump = boundUint40(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() * 2);

        // Create the single-segment array.
        LockupDynamic.Segment[] memory segments = new LockupDynamic.Segment[](1);
        segments[0] = segment;

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: segment.amount });

        // Create the stream with the fuzzed segment.
        LockupDynamic.CreateWithTimestamps memory params = defaults.createWithTimestampsLD();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.segments = segments;
        params.totalAmount = segment.amount;
        uint256 streamId = lockupDynamic.createWithTimestamps(params);

        // Simulate the passage of time.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;
        vm.warp({ newTimestamp: blockTimestamp });

        // Run the test.
        uint128 actualStreamedAmount = lockupDynamic.streamedAmountOf(streamId);
        uint128 expectedStreamedAmount = calculateStreamedAmountForOneSegment(blockTimestamp, segment);
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    modifier givenMultipleSegments() {
        _;
    }

    modifier whenCurrentTimestampNot1st() {
        _;
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - End time in the past
    /// - End time in the present
    /// - End time in the future
    /// - Multiple deposit amounts
    /// - Status streaming
    /// - Status settled
    function testFuzz_StreamedAmountOf_Calculation(
        LockupDynamic.Segment[] memory segments,
        uint40 timeJump
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenStartTimeInThePast
        givenMultipleSegments
        whenCurrentTimestampNot1st
    {
        vm.assume(segments.length > 1);

        // Fuzz the segment timestamps.
        fuzzSegmentTimestamps(segments, defaults.START_TIME());

        // Fuzz the segment amounts.
        (uint128 totalAmount,) =
            fuzzDynamicStreamAmounts({ upperBound: MAX_UINT128, segments: segments, brokerFee: ZERO });

        // Bound the time jump.
        uint40 firstSegmentDuration = segments[1].timestamp - segments[0].timestamp;
        uint40 totalDuration = segments[segments.length - 1].timestamp - defaults.START_TIME();
        timeJump = boundUint40(timeJump, firstSegmentDuration, totalDuration + 100 seconds);

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: totalAmount });

        // Create the stream with the fuzzed segments.
        LockupDynamic.CreateWithTimestamps memory params = defaults.createWithTimestampsLD();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.segments = segments;
        params.totalAmount = totalAmount;
        uint256 streamId = lockupDynamic.createWithTimestamps(params);

        // Simulate the passage of time.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;
        vm.warp({ newTimestamp: blockTimestamp });

        // Run the test.
        uint128 actualStreamedAmount = lockupDynamic.streamedAmountOf(streamId);
        uint128 expectedStreamedAmount =
            calculateStreamedAmountForMultipleSegments(blockTimestamp, segments, totalAmount);
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    /// @dev The streamed amount must never go down over time.
    function testFuzz_StreamedAmountOf_Monotonicity(
        LockupDynamic.Segment[] memory segments,
        uint40 timeWarp0,
        uint40 timeWarp1
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenStartTimeInThePast
        givenMultipleSegments
        whenCurrentTimestampNot1st
    {
        vm.assume(segments.length > 1);

        // Fuzz the segment timestamps.
        fuzzSegmentTimestamps(segments, defaults.START_TIME());

        // Fuzz the segment amounts.
        (uint128 totalAmount,) =
            fuzzDynamicStreamAmounts({ upperBound: MAX_UINT128, segments: segments, brokerFee: ZERO });

        // Bound the time warps.
        uint40 firstSegmentDuration = segments[1].timestamp - segments[0].timestamp;
        uint40 totalDuration = segments[segments.length - 1].timestamp - defaults.START_TIME();
        timeWarp0 = boundUint40(timeWarp0, firstSegmentDuration, totalDuration - 1);
        timeWarp1 = boundUint40(timeWarp1, timeWarp0, totalDuration);

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: totalAmount });

        // Create the stream with the fuzzed segments.
        LockupDynamic.CreateWithTimestamps memory params = defaults.createWithTimestampsLD();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.segments = segments;
        params.totalAmount = totalAmount;
        uint256 streamId = lockupDynamic.createWithTimestamps(params);

        // Warp to the future for the first time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeWarp0 });

        // Calculate the streamed amount at this midpoint in time.
        uint128 streamedAmount0 = lockupDynamic.streamedAmountOf(streamId);

        // Warp to the future for the second time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeWarp1 });

        // Assert that this streamed amount is greater than or equal to the previous streamed amount.
        uint128 streamedAmount1 = lockupDynamic.streamedAmountOf(streamId);
        assertGe(streamedAmount1, streamedAmount0, "streamedAmount");
    }
}
