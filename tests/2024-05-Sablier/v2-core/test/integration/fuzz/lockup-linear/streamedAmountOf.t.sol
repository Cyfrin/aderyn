// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ZERO } from "@prb/math/src/UD60x18.sol";

import { Broker, LockupLinear } from "src/types/DataTypes.sol";

import { StreamedAmountOf_Integration_Shared_Test } from "../../shared/lockup/streamedAmountOf.t.sol";
import { LockupLinear_Integration_Fuzz_Test } from "./LockupLinear.t.sol";

contract StreamedAmountOf_LockupLinear_Integration_Fuzz_Test is
    LockupLinear_Integration_Fuzz_Test,
    StreamedAmountOf_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupLinear_Integration_Fuzz_Test, StreamedAmountOf_Integration_Shared_Test)
    {
        LockupLinear_Integration_Fuzz_Test.setUp();
        StreamedAmountOf_Integration_Shared_Test.setUp();
        defaultStreamId = createDefaultStream();

        resetPrank({ msgSender: users.sender });
    }

    function testFuzz_StreamedAmountOf_CliffTimeInTheFuture(uint40 timeJump)
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
    {
        timeJump = boundUint40(timeJump, 0, defaults.CLIFF_DURATION() - 1);
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });
        uint128 actualStreamedAmount = lockupLinear.streamedAmountOf(defaultStreamId);
        uint128 expectedStreamedAmount = 0;
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    modifier whenCliffTimeNotInTheFuture() {
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
        uint40 timeJump,
        uint128 depositAmount
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenCliffTimeNotInTheFuture
    {
        vm.assume(depositAmount != 0);
        timeJump = boundUint40(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() * 2);

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: depositAmount });

        // Create the stream with the fuzzed deposit amount.
        LockupLinear.CreateWithTimestamps memory params = defaults.createWithTimestampsLL();
        params.broker = Broker({ account: address(0), fee: ZERO });
        params.totalAmount = depositAmount;
        uint256 streamId = lockupLinear.createWithTimestamps(params);

        // Simulate the passage of time.
        uint40 blockTimestamp = defaults.START_TIME() + timeJump;
        vm.warp({ newTimestamp: blockTimestamp });

        // Run the test.
        uint128 actualStreamedAmount = lockupLinear.streamedAmountOf(streamId);
        uint128 expectedStreamedAmount = calculateStreamedAmount(blockTimestamp, depositAmount);
        assertEq(actualStreamedAmount, expectedStreamedAmount, "streamedAmount");
    }

    /// @dev The streamed amount must never go down over time.
    function testFuzz_StreamedAmountOf_Monotonicity(
        uint40 timeWarp0,
        uint40 timeWarp1,
        uint128 depositAmount
    )
        external
        givenNotNull
        givenStreamHasNotBeenCanceled
        whenCliffTimeNotInTheFuture
    {
        vm.assume(depositAmount != 0);
        timeWarp0 = boundUint40(timeWarp0, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() - 1);
        timeWarp1 = boundUint40(timeWarp1, timeWarp0, defaults.TOTAL_DURATION());

        // Mint enough assets to the Sender.
        deal({ token: address(dai), to: users.sender, give: depositAmount });

        // Create the stream with the fuzzed deposit amount.
        LockupLinear.CreateWithTimestamps memory params = defaults.createWithTimestampsLL();
        params.totalAmount = depositAmount;
        uint256 streamId = lockupLinear.createWithTimestamps(params);

        // Warp to the future for the first time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeWarp0 });

        // Calculate the streamed amount at this midpoint in time.
        uint128 streamedAmount0 = lockupLinear.streamedAmountOf(streamId);

        // Warp to the future for the second time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeWarp1 });

        // Assert that this streamed amount is greater than or equal to the previous streamed amount.
        uint128 streamedAmount1 = lockupLinear.streamedAmountOf(streamId);
        assertGe(streamedAmount1, streamedAmount0, "streamedAmount");
    }
}
