// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ZERO } from "@prb/math/src/UD60x18.sol";
import { Broker, LockupTranched } from "src/types/DataTypes.sol";

import { StreamedAmountOf_Integration_Shared_Test } from "../../shared/lockup/streamedAmountOf.t.sol";
import { LockupTranched_Integration_Fuzz_Test } from "./LockupTranched.t.sol";

contract StreamedAmountOf_LockupTranched_Integration_Fuzz_Test is
    LockupTranched_Integration_Fuzz_Test,
    StreamedAmountOf_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Fuzz_Test, StreamedAmountOf_Integration_Shared_Test)
    {
        LockupTranched_Integration_Fuzz_Test.setUp();
        StreamedAmountOf_Integration_Shared_Test.setUp();

        resetPrank({ msgSender: users.sender });
    }

    modifier givenMultipleTranches() {
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
        LockupTranched.Tranche[] memory tranches,
        uint40 timeJump
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenStartTimeInThePast
        givenMultipleTranches
        whenCurrentTimestampNot1st
    {
        vm.assume(tranches.length > 1);

        // Fuzz the tranche timestamps.
        fuzzTrancheTimestamps(tranches, defaults.START_TIME());

        // Fuzz the tranche amounts.
        (uint128 totalAmount,) =
            fuzzTranchedStreamAmounts({ upperBound: MAX_UINT128, tranches: tranches, brokerFee: ZERO });

        // Bound the time jump.
        uint40 firstTrancheDuration = tranches[1].timestamp - tranches[0].timestamp;
        uint40 totalDuration = tranches[tranches.length - 1].timestamp - defaults.START_TIME();
        timeJump = boundUint40(timeJump, firstTrancheDuration, totalDuration + 100 seconds);

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: totalAmount });

        // Create the stream with the fuzzed tranches.
        LockupTranched.CreateWithTimestamps memory params = defaults.createWithTimestampsLT();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.tranches = tranches;
        params.totalAmount = totalAmount;
        uint256 streamId = lockupTranched.createWithTimestamps(params);

        // Simulate the passage of time.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;
        vm.warp({ newTimestamp: blockTimestamp });

        // Run the test.
        uint128 actualStreamedAmount = lockupTranched.streamedAmountOf(streamId);
        uint128 expectedStreamedAmount = calculateStreamedAmountForTranches(blockTimestamp, tranches, totalAmount);
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    /// @dev The streamed amount must never go down over time.
    function testFuzz_StreamedAmountOf_Monotonicity(
        LockupTranched.Tranche[] memory tranches,
        uint40 timeWarp0,
        uint40 timeWarp1
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenStartTimeInThePast
        givenMultipleTranches
        whenCurrentTimestampNot1st
    {
        vm.assume(tranches.length > 1);

        // Fuzz the tranche timestamps.
        fuzzTrancheTimestamps(tranches, defaults.START_TIME());

        // Fuzz the tranche amounts.
        (uint128 totalAmount,) =
            fuzzTranchedStreamAmounts({ upperBound: MAX_UINT128, tranches: tranches, brokerFee: ZERO });

        // Bound the time warps.
        uint40 firstTrancheDuration = tranches[1].timestamp - tranches[0].timestamp;
        uint40 totalDuration = tranches[tranches.length - 1].timestamp - defaults.START_TIME();
        timeWarp0 = boundUint40(timeWarp0, firstTrancheDuration, totalDuration - 1);
        timeWarp1 = boundUint40(timeWarp1, timeWarp0, totalDuration);

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: totalAmount });

        // Create the stream with the fuzzed tranches.
        LockupTranched.CreateWithTimestamps memory params = defaults.createWithTimestampsLT();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.tranches = tranches;
        params.totalAmount = totalAmount;
        uint256 streamId = lockupTranched.createWithTimestamps(params);

        // Warp to the future for the first time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeWarp0 });

        // Calculate the streamed amount at this midpoint in time.
        uint128 streamedAmount0 = lockupTranched.streamedAmountOf(streamId);

        // Warp to the future for the second time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeWarp1 });

        // Assert that this streamed amount is greater than or equal to the previous streamed amount.
        uint128 streamedAmount1 = lockupTranched.streamedAmountOf(streamId);
        assertGe(streamedAmount1, streamedAmount0, "streamedAmount");
    }
}
