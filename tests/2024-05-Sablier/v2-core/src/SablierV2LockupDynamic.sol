// SPDX-License-Identifier: BUSL-1.1
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import { PRBMathCastingUint128 as CastingUint128 } from "@prb/math/src/casting/Uint128.sol";
import { PRBMathCastingUint40 as CastingUint40 } from "@prb/math/src/casting/Uint40.sol";
import { SD59x18 } from "@prb/math/src/SD59x18.sol";

import { SablierV2Lockup } from "./abstracts/SablierV2Lockup.sol";
import { ISablierV2LockupDynamic } from "./interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2NFTDescriptor } from "./interfaces/ISablierV2NFTDescriptor.sol";
import { Helpers } from "./libraries/Helpers.sol";
import { Lockup, LockupDynamic } from "./types/DataTypes.sol";

/*

███████╗ █████╗ ██████╗ ██╗     ██╗███████╗██████╗     ██╗   ██╗██████╗
██╔════╝██╔══██╗██╔══██╗██║     ██║██╔════╝██╔══██╗    ██║   ██║╚════██╗
███████╗███████║██████╔╝██║     ██║█████╗  ██████╔╝    ██║   ██║ █████╔╝
╚════██║██╔══██║██╔══██╗██║     ██║██╔══╝  ██╔══██╗    ╚██╗ ██╔╝██╔═══╝
███████║██║  ██║██████╔╝███████╗██║███████╗██║  ██║     ╚████╔╝ ███████╗
╚══════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝╚══════╝╚═╝  ╚═╝      ╚═══╝  ╚══════╝

██╗      ██████╗  ██████╗██╗  ██╗██╗   ██╗██████╗     ██████╗ ██╗   ██╗███╗   ██╗ █████╗ ███╗   ███╗██╗ ██████╗
██║     ██╔═══██╗██╔════╝██║ ██╔╝██║   ██║██╔══██╗    ██╔══██╗╚██╗ ██╔╝████╗  ██║██╔══██╗████╗ ████║██║██╔════╝
██║     ██║   ██║██║     █████╔╝ ██║   ██║██████╔╝    ██║  ██║ ╚████╔╝ ██╔██╗ ██║███████║██╔████╔██║██║██║
██║     ██║   ██║██║     ██╔═██╗ ██║   ██║██╔═══╝     ██║  ██║  ╚██╔╝  ██║╚██╗██║██╔══██║██║╚██╔╝██║██║██║
███████╗╚██████╔╝╚██████╗██║  ██╗╚██████╔╝██║         ██████╔╝   ██║   ██║ ╚████║██║  ██║██║ ╚═╝ ██║██║╚██████╗
╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝         ╚═════╝    ╚═╝   ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝ ╚═════╝

*/

/// @title SablierV2LockupDynamic
/// @notice See the documentation in {ISablierV2LockupDynamic}.
contract SablierV2LockupDynamic is
    ISablierV2LockupDynamic, // 5 inherited components
    SablierV2Lockup // 14 inherited components
{
    using CastingUint128 for uint128;
    using CastingUint40 for uint40;
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupDynamic
    uint256 public immutable override MAX_SEGMENT_COUNT;

    /// @dev Stream segments mapped by stream IDs. This complements the `_streams` mapping in {SablierV2Lockup}.
    mapping(uint256 id => LockupDynamic.Segment[] segments) internal _segments;

    /*//////////////////////////////////////////////////////////////////////////
                                     CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Emits a {TransferAdmin} event.
    /// @param initialAdmin The address of the initial contract admin.
    /// @param initialNFTDescriptor The address of the NFT descriptor contract.
    /// @param maxSegmentCount The maximum number of segments allowed in a stream.
    constructor(
        address initialAdmin,
        ISablierV2NFTDescriptor initialNFTDescriptor,
        uint256 maxSegmentCount
    )
        ERC721("Sablier V2 Lockup Dynamic NFT", "SAB-V2-LOCKUP-DYN")
        SablierV2Lockup(initialAdmin, initialNFTDescriptor)
    {
        MAX_SEGMENT_COUNT = maxSegmentCount;
        nextStreamId = 1;
    }

    /*//////////////////////////////////////////////////////////////////////////
                           USER-FACING CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupDynamic
    function getSegments(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupDynamic.Segment[] memory segments)
    {
        segments = _segments[streamId];
    }

    /// @inheritdoc ISablierV2LockupDynamic
    function getStream(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupDynamic.StreamLD memory stream)
    {
        // Retrieve the Lockup stream from storage.
        Lockup.Stream memory lockupStream = _streams[streamId];

        // Settled streams cannot be canceled.
        if (_statusOf(streamId) == Lockup.Status.SETTLED) {
            lockupStream.isCancelable = false;
        }

        stream = LockupDynamic.StreamLD({
            amounts: lockupStream.amounts,
            asset: lockupStream.asset,
            endTime: lockupStream.endTime,
            isCancelable: lockupStream.isCancelable,
            isDepleted: lockupStream.isDepleted,
            isStream: lockupStream.isStream,
            isTransferable: lockupStream.isTransferable,
            recipient: _ownerOf(streamId),
            segments: _segments[streamId],
            sender: lockupStream.sender,
            startTime: lockupStream.startTime,
            wasCanceled: lockupStream.wasCanceled
        });
    }

    /// @inheritdoc ISablierV2LockupDynamic
    function getTimestamps(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupDynamic.Timestamps memory timestamps)
    {
        timestamps = LockupDynamic.Timestamps({ start: _streams[streamId].startTime, end: _streams[streamId].endTime });
    }

    /*//////////////////////////////////////////////////////////////////////////
                         USER-FACING NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupDynamic
    function createWithDurations(LockupDynamic.CreateWithDurations calldata params)
        external
        override
        noDelegateCall
        returns (uint256 streamId)
    {
        // Generate the canonical segments.
        LockupDynamic.Segment[] memory segments = Helpers.calculateSegmentTimestamps(params.segments);

        // Checks, Effects and Interactions: create the stream.
        streamId = _create(
            LockupDynamic.CreateWithTimestamps({
                sender: params.sender,
                recipient: params.recipient,
                totalAmount: params.totalAmount,
                asset: params.asset,
                cancelable: params.cancelable,
                transferable: params.transferable,
                startTime: uint40(block.timestamp),
                segments: segments,
                broker: params.broker
            })
        );
    }

    /// @inheritdoc ISablierV2LockupDynamic
    function createWithTimestamps(LockupDynamic.CreateWithTimestamps calldata params)
        external
        override
        noDelegateCall
        returns (uint256 streamId)
    {
        // Checks, Effects and Interactions: create the stream.
        streamId = _create(params);
    }

    /*//////////////////////////////////////////////////////////////////////////
                             INTERNAL CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc SablierV2Lockup
    /// @dev The distribution function is:
    ///
    /// $$
    /// f(x) = x^{exp} * csa + \Sigma(esa)
    /// $$
    ///
    /// Where:
    ///
    /// - $x$ is the elapsed time divided by the total duration of the current segment.
    /// - $exp$ is the current segment exponent.
    /// - $csa$ is the current segment amount.
    /// - $\Sigma(esa)$ is the sum of all vested segments' amounts.
    function _calculateStreamedAmount(uint256 streamId) internal view override returns (uint128) {
        // If the start time is in the future, return zero.
        uint40 blockTimestamp = uint40(block.timestamp);
        if (_streams[streamId].startTime >= blockTimestamp) {
            return 0;
        }

        // If the end time is not in the future, return the deposited amount.
        uint40 endTime = _streams[streamId].endTime;
        if (endTime <= blockTimestamp) {
            return _streams[streamId].amounts.deposited;
        }

        if (_segments[streamId].length > 1) {
            // If there is more than one segment, it may be required to iterate over all of them.
            return _calculateStreamedAmountForMultipleSegments(streamId);
        } else {
            // Otherwise, there is only one segment, and the calculation is simpler.
            return _calculateStreamedAmountForOneSegment(streamId);
        }
    }

    /// @dev Calculates the streamed amount for a stream with multiple segments.
    ///
    /// Notes:
    ///
    /// 1. Normalization to 18 decimals is not needed because there is no mix of amounts with different decimals.
    /// 2. The stream's start time must be in the past so that the calculations below do not overflow.
    /// 3. The stream's end time must be in the future so that the loop below does not panic with an "index out of
    /// bounds" error.
    function _calculateStreamedAmountForMultipleSegments(uint256 streamId) internal view returns (uint128) {
        unchecked {
            uint40 blockTimestamp = uint40(block.timestamp);
            Lockup.Stream memory stream = _streams[streamId];
            LockupDynamic.Segment[] memory segments = _segments[streamId];

            // Sum the amounts in all segments that precede the block timestamp.
            uint128 previousSegmentAmounts;
            uint40 currentSegmentTimestamp = segments[0].timestamp;
            uint256 index = 0;
            while (currentSegmentTimestamp < blockTimestamp) {
                previousSegmentAmounts += segments[index].amount;
                index += 1;
                currentSegmentTimestamp = segments[index].timestamp;
            }

            // After exiting the loop, the current segment is at `index`.
            SD59x18 currentSegmentAmount = segments[index].amount.intoSD59x18();
            SD59x18 currentSegmentExponent = segments[index].exponent.intoSD59x18();
            currentSegmentTimestamp = segments[index].timestamp;

            uint40 previousTimestamp;
            if (index == 0) {
                // When the current segment's index is equal to 0, the current segment is the first, so use the start
                // time as the previous timestamp.
                previousTimestamp = stream.startTime;
            } else {
                // Otherwise, when the current segment's index is greater than zero, it means that the segment is not
                // the first. In this case, use the previous segment's timestamp.
                previousTimestamp = segments[index - 1].timestamp;
            }

            // Calculate how much time has passed since the segment started, and the total duration of the segment.
            SD59x18 elapsedTime = (blockTimestamp - previousTimestamp).intoSD59x18();
            SD59x18 segmentDuration = (currentSegmentTimestamp - previousTimestamp).intoSD59x18();

            // Divide the elapsed time by the total duration of the segment.
            SD59x18 elapsedTimePercentage = elapsedTime.div(segmentDuration);

            // Calculate the streamed amount using the special formula.
            SD59x18 multiplier = elapsedTimePercentage.pow(currentSegmentExponent);
            SD59x18 segmentStreamedAmount = multiplier.mul(currentSegmentAmount);

            // Although the segment streamed amount should never exceed the total segment amount, this condition is
            // checked without asserting to avoid locking funds in case of a bug. If this situation occurs, the
            // amount streamed in the segment is considered zero (except for past withdrawals), and the segment is
            // effectively voided.
            if (segmentStreamedAmount.gt(currentSegmentAmount)) {
                return previousSegmentAmounts > stream.amounts.withdrawn
                    ? previousSegmentAmounts
                    : stream.amounts.withdrawn;
            }

            // Calculate the total streamed amount by adding the previous segment amounts and the amount streamed in
            // the current segment. Casting to uint128 is safe due to the if statement above.
            return previousSegmentAmounts + uint128(segmentStreamedAmount.intoUint256());
        }
    }

    /// @dev Calculates the streamed amount for a stream with one segment. Normalization to 18 decimals is not
    /// needed because there is no mix of amounts with different decimals.
    function _calculateStreamedAmountForOneSegment(uint256 streamId) internal view returns (uint128) {
        unchecked {
            // Calculate how much time has passed since the stream started, and the stream's total duration.
            SD59x18 elapsedTime = (uint40(block.timestamp) - _streams[streamId].startTime).intoSD59x18();
            SD59x18 totalDuration = (_streams[streamId].endTime - _streams[streamId].startTime).intoSD59x18();

            // Divide the elapsed time by the stream's total duration.
            SD59x18 elapsedTimePercentage = elapsedTime.div(totalDuration);

            // Cast the stream parameters to SD59x18.
            SD59x18 exponent = _segments[streamId][0].exponent.intoSD59x18();
            SD59x18 depositedAmount = _streams[streamId].amounts.deposited.intoSD59x18();

            // Calculate the streamed amount using the special formula.
            SD59x18 multiplier = elapsedTimePercentage.pow(exponent);
            SD59x18 streamedAmount = multiplier.mul(depositedAmount);

            // Although the streamed amount should never exceed the deposited amount, this condition is checked
            // without asserting to avoid locking funds in case of a bug. If this situation occurs, the withdrawn
            // amount is considered to be the streamed amount, and the stream is effectively frozen.
            if (streamedAmount.gt(depositedAmount)) {
                return _streams[streamId].amounts.withdrawn;
            }

            // Cast the streamed amount to uint128. This is safe due to the check above.
            return uint128(streamedAmount.intoUint256());
        }
    }

    /*//////////////////////////////////////////////////////////////////////////
                           INTERNAL NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev See the documentation for the user-facing functions that call this internal function.
    function _create(LockupDynamic.CreateWithTimestamps memory params) internal returns (uint256 streamId) {
        // Check: verify the broker fee and calculate the amounts.
        Lockup.CreateAmounts memory createAmounts =
            Helpers.checkAndCalculateBrokerFee(params.totalAmount, params.broker.fee, MAX_BROKER_FEE);

        // Check: validate the user-provided parameters.
        Helpers.checkCreateLockupDynamic(createAmounts.deposit, params.segments, MAX_SEGMENT_COUNT, params.startTime);

        // Load the stream ID in a variable.
        streamId = nextStreamId;

        // Effect: create the stream.
        Lockup.Stream storage stream = _streams[streamId];
        stream.amounts.deposited = createAmounts.deposit;
        stream.asset = params.asset;
        stream.isCancelable = params.cancelable;
        stream.isStream = true;
        stream.isTransferable = params.transferable;
        stream.sender = params.sender;
        stream.startTime = params.startTime;

        unchecked {
            // The segment count cannot be zero at this point.
            uint256 segmentCount = params.segments.length;
            stream.endTime = params.segments[segmentCount - 1].timestamp;

            // Effect: store the segments. Since Solidity lacks a syntax for copying arrays of structs directly from
            // memory to storage, a manual approach is necessary. See https://github.com/ethereum/solidity/issues/12783.
            for (uint256 i = 0; i < segmentCount; ++i) {
                _segments[streamId].push(params.segments[i]);
            }

            // Effect: bump the next stream ID.
            // Using unchecked arithmetic because these calculations cannot realistically overflow, ever.
            nextStreamId = streamId + 1;
        }

        // Effect: mint the NFT to the recipient.
        _mint({ to: params.recipient, tokenId: streamId });

        // Interaction: transfer the deposit amount.
        params.asset.safeTransferFrom({ from: msg.sender, to: address(this), value: createAmounts.deposit });

        // Interaction: pay the broker fee, if not zero.
        if (createAmounts.brokerFee > 0) {
            params.asset.safeTransferFrom({ from: msg.sender, to: params.broker.account, value: createAmounts.brokerFee });
        }

        // Log the newly created stream.
        emit ISablierV2LockupDynamic.CreateLockupDynamicStream({
            streamId: streamId,
            funder: msg.sender,
            sender: params.sender,
            recipient: params.recipient,
            amounts: createAmounts,
            asset: params.asset,
            cancelable: params.cancelable,
            transferable: params.transferable,
            segments: params.segments,
            timestamps: LockupDynamic.Timestamps({ start: stream.startTime, end: stream.endTime }),
            broker: params.broker.account
        });
    }
}
