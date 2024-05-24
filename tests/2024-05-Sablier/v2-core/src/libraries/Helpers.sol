// SPDX-License-Identifier: BUSL-1.1
pragma solidity >=0.8.22;

import { UD60x18, ud } from "@prb/math/src/UD60x18.sol";

import { Lockup, LockupDynamic, LockupLinear, LockupTranched } from "../types/DataTypes.sol";
import { Errors } from "./Errors.sol";

/// @title Helpers
/// @notice Library with helper functions needed across the Sablier V2 contracts.
library Helpers {
    /*//////////////////////////////////////////////////////////////////////////
                             INTERNAL CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Calculate the timestamps and return the segments.
    function calculateSegmentTimestamps(LockupDynamic.SegmentWithDuration[] memory segments)
        internal
        view
        returns (LockupDynamic.Segment[] memory segmentsWithTimestamps)
    {
        uint256 segmentCount = segments.length;
        segmentsWithTimestamps = new LockupDynamic.Segment[](segmentCount);

        // Make the block timestamp the stream's start time.
        uint40 startTime = uint40(block.timestamp);

        // It is safe to use unchecked arithmetic because {SablierV2LockupDynamic-_create} will nonetheless check the
        // correctness of the calculated segment timestamps.
        unchecked {
            // The first segment is precomputed because it is needed in the for loop below.
            segmentsWithTimestamps[0] = LockupDynamic.Segment({
                amount: segments[0].amount,
                exponent: segments[0].exponent,
                timestamp: startTime + segments[0].duration
            });

            // Copy the segment amounts and exponents, and calculate the segment timestamps.
            for (uint256 i = 1; i < segmentCount; ++i) {
                segmentsWithTimestamps[i] = LockupDynamic.Segment({
                    amount: segments[i].amount,
                    exponent: segments[i].exponent,
                    timestamp: segmentsWithTimestamps[i - 1].timestamp + segments[i].duration
                });
            }
        }
    }

    /// @dev Calculate the timestamps and return the tranches.
    function calculateTrancheTimestamps(LockupTranched.TrancheWithDuration[] memory tranches)
        internal
        view
        returns (LockupTranched.Tranche[] memory tranchesWithTimestamps)
    {
        uint256 trancheCount = tranches.length;
        tranchesWithTimestamps = new LockupTranched.Tranche[](trancheCount);

        // Make the block timestamp the stream's start time.
        uint40 startTime = uint40(block.timestamp);

        // It is safe to use unchecked arithmetic because {SablierV2LockupTranched-_create} will nonetheless check the
        // correctness of the calculated tranche timestamps.
        unchecked {
            // The first tranche is precomputed because it is needed in the for loop below.
            tranchesWithTimestamps[0] =
                LockupTranched.Tranche({ amount: tranches[0].amount, timestamp: startTime + tranches[0].duration });

            // Copy the tranche amounts and calculate the tranche timestamps.
            for (uint256 i = 1; i < trancheCount; ++i) {
                tranchesWithTimestamps[i] = LockupTranched.Tranche({
                    amount: tranches[i].amount,
                    timestamp: tranchesWithTimestamps[i - 1].timestamp + tranches[i].duration
                });
            }
        }
    }

    /// @dev Checks the broker fee is not greater than `maxBrokerFee`, and then calculates the broker fee amount and the
    /// deposit amount from the total amount.
    function checkAndCalculateBrokerFee(
        uint128 totalAmount,
        UD60x18 brokerFee,
        UD60x18 maxBrokerFee
    )
        internal
        pure
        returns (Lockup.CreateAmounts memory amounts)
    {
        // When the total amount is zero, the broker fee is also zero.
        if (totalAmount == 0) {
            return Lockup.CreateAmounts(0, 0);
        }

        // Check: the broker fee is not greater than `maxBrokerFee`.
        if (brokerFee.gt(maxBrokerFee)) {
            revert Errors.SablierV2Lockup_BrokerFeeTooHigh(brokerFee, maxBrokerFee);
        }

        // Calculate the broker fee amount.
        // The cast to uint128 is safe because the maximum fee is hard coded.
        amounts.brokerFee = uint128(ud(totalAmount).mul(brokerFee).intoUint256());

        // Assert that the total amount is strictly greater than the broker fee amount.
        assert(totalAmount > amounts.brokerFee);

        // Calculate the deposit amount (the amount to stream, net of the broker fee).
        amounts.deposit = totalAmount - amounts.brokerFee;
    }

    /// @dev Checks the parameters of the {SablierV2LockupDynamic-_create} function.
    function checkCreateLockupDynamic(
        uint128 depositAmount,
        LockupDynamic.Segment[] memory segments,
        uint256 maxSegmentCount,
        uint40 startTime
    )
        internal
        view
    {
        // Check: the deposit amount is not zero.
        if (depositAmount == 0) {
            revert Errors.SablierV2Lockup_DepositAmountZero();
        }

        // Check: the start time is not zero.
        if (startTime == 0) {
            revert Errors.SablierV2Lockup_StartTimeZero();
        }

        // Check: the segment count is not zero.
        uint256 segmentCount = segments.length;
        if (segmentCount == 0) {
            revert Errors.SablierV2LockupDynamic_SegmentCountZero();
        }

        // Check: the segment count is not greater than the maximum allowed.
        if (segmentCount > maxSegmentCount) {
            revert Errors.SablierV2LockupDynamic_SegmentCountTooHigh(segmentCount);
        }

        // Check: requirements of segments.
        _checkSegments(segments, depositAmount, startTime);
    }

    /// @dev Checks the parameters of the {SablierV2LockupLinear-_create} function.
    function checkCreateLockupLinear(uint128 depositAmount, LockupLinear.Timestamps memory timestamps) internal view {
        // Check: the deposit amount is not zero.
        if (depositAmount == 0) {
            revert Errors.SablierV2Lockup_DepositAmountZero();
        }

        // Check: the start time is not zero.
        if (timestamps.start == 0) {
            revert Errors.SablierV2Lockup_StartTimeZero();
        }

        // Since a cliff time of zero means there is no cliff, the following checks are performed only if it's not zero.
        if (timestamps.cliff > 0) {
            // Check: the start time is strictly less than the cliff time.
            if (timestamps.start >= timestamps.cliff) {
                revert Errors.SablierV2LockupLinear_StartTimeNotLessThanCliffTime(timestamps.start, timestamps.cliff);
            }

            // Check: the cliff time is strictly less than the end time.
            if (timestamps.cliff >= timestamps.end) {
                revert Errors.SablierV2LockupLinear_CliffTimeNotLessThanEndTime(timestamps.cliff, timestamps.end);
            }
        }

        // Check: the start time is strictly less than the end time.
        if (timestamps.start >= timestamps.end) {
            revert Errors.SablierV2LockupLinear_StartTimeNotLessThanEndTime(timestamps.start, timestamps.end);
        }

        // Check: the end time is in the future.
        uint40 blockTimestamp = uint40(block.timestamp);
        if (blockTimestamp >= timestamps.end) {
            revert Errors.SablierV2Lockup_EndTimeNotInTheFuture(blockTimestamp, timestamps.end);
        }
    }

    /// @dev Checks the parameters of the {SablierV2LockupTranched-_create} function.
    function checkCreateLockupTranched(
        uint128 depositAmount,
        LockupTranched.Tranche[] memory tranches,
        uint256 maxTrancheCount,
        uint40 startTime
    )
        internal
        view
    {
        // Check: the deposit amount is not zero.
        if (depositAmount == 0) {
            revert Errors.SablierV2Lockup_DepositAmountZero();
        }

        // Check: the start time is not zero.
        if (startTime == 0) {
            revert Errors.SablierV2Lockup_StartTimeZero();
        }

        // Check: the tranche count is not zero.
        uint256 trancheCount = tranches.length;
        if (trancheCount == 0) {
            revert Errors.SablierV2LockupTranched_TrancheCountZero();
        }

        // Check: the tranche count is not greater than the maximum allowed.
        if (trancheCount > maxTrancheCount) {
            revert Errors.SablierV2LockupTranched_TrancheCountTooHigh(trancheCount);
        }

        // Check: requirements of tranches.
        _checkTranches(tranches, depositAmount, startTime);
    }

    /*//////////////////////////////////////////////////////////////////////////
                             PRIVATE CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Checks that:
    ///
    /// 1. The first timestamp is strictly greater than the start time.
    /// 2. The timestamps are ordered chronologically.
    /// 3. There are no duplicate timestamps.
    /// 4. The deposit amount is equal to the sum of all segment amounts.
    function _checkSegments(
        LockupDynamic.Segment[] memory segments,
        uint128 depositAmount,
        uint40 startTime
    )
        private
        view
    {
        // Check: the start time is strictly less than the first segment timestamp.
        if (startTime >= segments[0].timestamp) {
            revert Errors.SablierV2LockupDynamic_StartTimeNotLessThanFirstSegmentTimestamp(
                startTime, segments[0].timestamp
            );
        }

        // Pre-declare the variables needed in the for loop.
        uint128 segmentAmountsSum;
        uint40 currentSegmentTimestamp;
        uint40 previousSegmentTimestamp;

        // Iterate over the segments to:
        //
        // 1. Calculate the sum of all segment amounts.
        // 2. Check that the timestamps are ordered.
        uint256 count = segments.length;
        for (uint256 index = 0; index < count; ++index) {
            // Add the current segment amount to the sum.
            segmentAmountsSum += segments[index].amount;

            // Check: the current timestamp is strictly greater than the previous timestamp.
            currentSegmentTimestamp = segments[index].timestamp;
            if (currentSegmentTimestamp <= previousSegmentTimestamp) {
                revert Errors.SablierV2LockupDynamic_SegmentTimestampsNotOrdered(
                    index, previousSegmentTimestamp, currentSegmentTimestamp
                );
            }

            // Make the current timestamp the previous timestamp of the next loop iteration.
            previousSegmentTimestamp = currentSegmentTimestamp;
        }

        // Check: the last timestamp is in the future.
        // When the loop exits, the current segment's timestamp is the last segment's timestamp, i.e. the stream's end
        // time. The variable is not renamed for gas efficiency purposes.
        uint40 blockTimestamp = uint40(block.timestamp);
        if (blockTimestamp >= currentSegmentTimestamp) {
            revert Errors.SablierV2Lockup_EndTimeNotInTheFuture(blockTimestamp, currentSegmentTimestamp);
        }

        // Check: the deposit amount is equal to the segment amounts sum.
        if (depositAmount != segmentAmountsSum) {
            revert Errors.SablierV2LockupDynamic_DepositAmountNotEqualToSegmentAmountsSum(
                depositAmount, segmentAmountsSum
            );
        }
    }

    /// @dev Checks that:
    ///
    /// 1. The first timestamp is strictly greater than the start time.
    /// 2. The timestamps are ordered chronologically.
    /// 3. There are no duplicate timestamps.
    /// 4. The deposit amount is equal to the sum of all tranche amounts.
    function _checkTranches(
        LockupTranched.Tranche[] memory tranches,
        uint128 depositAmount,
        uint40 startTime
    )
        private
        view
    {
        // Check: the start time is strictly less than the first tranche timestamp.
        if (startTime >= tranches[0].timestamp) {
            revert Errors.SablierV2LockupTranched_StartTimeNotLessThanFirstTrancheTimestamp(
                startTime, tranches[0].timestamp
            );
        }

        // Pre-declare the variables needed in the for loop.
        uint128 trancheAmountsSum;
        uint40 currentTrancheTimestamp;
        uint40 previousTrancheTimestamp;

        // Iterate over the tranches to:
        //
        // 1. Calculate the sum of all tranche amounts.
        // 2. Check that the timestamps are ordered.
        uint256 count = tranches.length;
        for (uint256 index = 0; index < count; ++index) {
            // Add the current tranche amount to the sum.
            trancheAmountsSum += tranches[index].amount;

            // Check: the current timestamp is strictly greater than the previous timestamp.
            currentTrancheTimestamp = tranches[index].timestamp;
            if (currentTrancheTimestamp <= previousTrancheTimestamp) {
                revert Errors.SablierV2LockupTranched_TrancheTimestampsNotOrdered(
                    index, previousTrancheTimestamp, currentTrancheTimestamp
                );
            }

            // Make the current timestamp the previous timestamp of the next loop iteration.
            previousTrancheTimestamp = currentTrancheTimestamp;
        }

        // Check: the last timestamp is in the future.
        // When the loop exits, the current tranche's timestamp is the last tranche's timestamp, i.e. the stream's end
        // time. The variable is not renamed for gas efficiency purposes.
        uint40 blockTimestamp = uint40(block.timestamp);
        if (blockTimestamp >= currentTrancheTimestamp) {
            revert Errors.SablierV2Lockup_EndTimeNotInTheFuture(blockTimestamp, currentTrancheTimestamp);
        }

        // Check: the deposit amount is equal to the tranche amounts sum.
        if (depositAmount != trancheAmountsSum) {
            revert Errors.SablierV2LockupTranched_DepositAmountNotEqualToTrancheAmountsSum(
                depositAmount, trancheAmountsSum
            );
        }
    }
}
