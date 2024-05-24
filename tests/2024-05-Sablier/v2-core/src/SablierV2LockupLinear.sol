// SPDX-License-Identifier: BUSL-1.1
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import { UD60x18, ud } from "@prb/math/src/UD60x18.sol";

import { SablierV2Lockup } from "./abstracts/SablierV2Lockup.sol";
import { SablierV2Lockup } from "./abstracts/SablierV2Lockup.sol";
import { ISablierV2LockupLinear } from "./interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2NFTDescriptor } from "./interfaces/ISablierV2NFTDescriptor.sol";
import { Helpers } from "./libraries/Helpers.sol";
import { Lockup, LockupLinear } from "./types/DataTypes.sol";

/*

███████╗ █████╗ ██████╗ ██╗     ██╗███████╗██████╗     ██╗   ██╗██████╗
██╔════╝██╔══██╗██╔══██╗██║     ██║██╔════╝██╔══██╗    ██║   ██║╚════██╗
███████╗███████║██████╔╝██║     ██║█████╗  ██████╔╝    ██║   ██║ █████╔╝
╚════██║██╔══██║██╔══██╗██║     ██║██╔══╝  ██╔══██╗    ╚██╗ ██╔╝██╔═══╝
███████║██║  ██║██████╔╝███████╗██║███████╗██║  ██║     ╚████╔╝ ███████╗
╚══════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝╚══════╝╚═╝  ╚═╝      ╚═══╝  ╚══════╝

██╗      ██████╗  ██████╗██╗  ██╗██╗   ██╗██████╗     ██╗     ██╗███╗   ██╗███████╗ █████╗ ██████╗
██║     ██╔═══██╗██╔════╝██║ ██╔╝██║   ██║██╔══██╗    ██║     ██║████╗  ██║██╔════╝██╔══██╗██╔══██╗
██║     ██║   ██║██║     █████╔╝ ██║   ██║██████╔╝    ██║     ██║██╔██╗ ██║█████╗  ███████║██████╔╝
██║     ██║   ██║██║     ██╔═██╗ ██║   ██║██╔═══╝     ██║     ██║██║╚██╗██║██╔══╝  ██╔══██║██╔══██╗
███████╗╚██████╔╝╚██████╗██║  ██╗╚██████╔╝██║         ███████╗██║██║ ╚████║███████╗██║  ██║██║  ██║
╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝         ╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝

*/

/// @title SablierV2LockupLinear
/// @notice See the documentation in {ISablierV2LockupLinear}.
contract SablierV2LockupLinear is
    ISablierV2LockupLinear, // 5 inherited components
    SablierV2Lockup // 14 inherited components
{
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Cliff times mapped by stream IDs. This complements the `_streams` mapping in {SablierV2Lockup}.
    mapping(uint256 id => uint40 cliff) internal _cliffs;

    /*//////////////////////////////////////////////////////////////////////////
                                     CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Emits a {TransferAdmin} event.
    /// @param initialAdmin The address of the initial contract admin.
    /// @param initialNFTDescriptor The address of the initial NFT descriptor.
    constructor(
        address initialAdmin,
        ISablierV2NFTDescriptor initialNFTDescriptor
    )
        ERC721("Sablier V2 Lockup Linear NFT", "SAB-V2-LOCKUP-LIN")
        SablierV2Lockup(initialAdmin, initialNFTDescriptor)
    {
        nextStreamId = 1;
    }

    /*//////////////////////////////////////////////////////////////////////////
                           USER-FACING CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupLinear
    function getCliffTime(uint256 streamId) external view override notNull(streamId) returns (uint40 cliffTime) {
        cliffTime = _cliffs[streamId];
    }

    /// @inheritdoc ISablierV2LockupLinear
    function getStream(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupLinear.StreamLL memory stream)
    {
        // Retrieve the Lockup stream from storage.
        Lockup.Stream memory lockupStream = _streams[streamId];

        // Settled streams cannot be canceled.
        if (_statusOf(streamId) == Lockup.Status.SETTLED) {
            lockupStream.isCancelable = false;
        }

        stream = LockupLinear.StreamLL({
            amounts: lockupStream.amounts,
            asset: lockupStream.asset,
            cliffTime: _cliffs[streamId],
            endTime: lockupStream.endTime,
            isCancelable: lockupStream.isCancelable,
            isTransferable: lockupStream.isTransferable,
            isDepleted: lockupStream.isDepleted,
            isStream: lockupStream.isStream,
            recipient: _ownerOf(streamId),
            sender: lockupStream.sender,
            startTime: lockupStream.startTime,
            wasCanceled: lockupStream.wasCanceled
        });
    }

    /// @inheritdoc ISablierV2LockupLinear
    function getTimestamps(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupLinear.Timestamps memory timestamps)
    {
        timestamps = LockupLinear.Timestamps({
            start: _streams[streamId].startTime,
            cliff: _cliffs[streamId],
            end: _streams[streamId].endTime
        });
    }

    /*//////////////////////////////////////////////////////////////////////////
                         USER-FACING NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupLinear
    function createWithDurations(LockupLinear.CreateWithDurations calldata params)
        external
        override
        noDelegateCall
        returns (uint256 streamId)
    {
        // Set the current block timestamp as the stream's start time.
        LockupLinear.Timestamps memory timestamps;
        timestamps.start = uint40(block.timestamp);

        // Calculate the cliff time and the end time. It is safe to use unchecked arithmetic because {_create} will
        // nonetheless check that the end time is greater than the cliff time, and also that the cliff time, if set,
        // is greater than or equal to the start time.
        unchecked {
            if (params.durations.cliff > 0) {
                timestamps.cliff = timestamps.start + params.durations.cliff;
            }
            timestamps.end = timestamps.start + params.durations.total;
        }

        // Checks, Effects and Interactions: create the stream.
        streamId = _create(
            LockupLinear.CreateWithTimestamps({
                sender: params.sender,
                recipient: params.recipient,
                totalAmount: params.totalAmount,
                asset: params.asset,
                cancelable: params.cancelable,
                transferable: params.transferable,
                timestamps: timestamps,
                broker: params.broker
            })
        );
    }

    /// @inheritdoc ISablierV2LockupLinear
    function createWithTimestamps(LockupLinear.CreateWithTimestamps calldata params)
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
    /// f(x) = x * d + c
    /// $$
    ///
    /// Where:
    ///
    /// - $x$ is the elapsed time divided by the stream's total duration.
    /// - $d$ is the deposited amount.
    /// - $c$ is the cliff amount.
    function _calculateStreamedAmount(uint256 streamId) internal view override returns (uint128) {
        // If the cliff time is in the future, return zero.
        uint256 cliffTime = uint256(_cliffs[streamId]);
        uint256 blockTimestamp = block.timestamp;
        if (cliffTime > blockTimestamp) {
            return 0;
        }

        // If the end time is not in the future, return the deposited amount.
        uint256 endTime = uint256(_streams[streamId].endTime);
        if (blockTimestamp >= endTime) {
            return _streams[streamId].amounts.deposited;
        }

        // In all other cases, calculate the amount streamed so far. Normalization to 18 decimals is not needed
        // because there is no mix of amounts with different decimals.
        unchecked {
            // Calculate how much time has passed since the stream started, and the stream's total duration.
            uint256 startTime = uint256(_streams[streamId].startTime);
            UD60x18 elapsedTime = ud(blockTimestamp - startTime);
            UD60x18 totalDuration = ud(endTime - startTime);

            // Divide the elapsed time by the stream's total duration.
            UD60x18 elapsedTimePercentage = elapsedTime.div(totalDuration);

            // Cast the deposited amount to UD60x18.
            UD60x18 depositedAmount = ud(_streams[streamId].amounts.deposited);

            // Calculate the streamed amount by multiplying the elapsed time percentage by the deposited amount.
            UD60x18 streamedAmount = elapsedTimePercentage.mul(depositedAmount);

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
    function _create(LockupLinear.CreateWithTimestamps memory params) internal returns (uint256 streamId) {
        // Check: verify the broker fee and calculate the amounts.
        Lockup.CreateAmounts memory createAmounts =
            Helpers.checkAndCalculateBrokerFee(params.totalAmount, params.broker.fee, MAX_BROKER_FEE);

        // Check: validate the user-provided parameters.
        Helpers.checkCreateLockupLinear(createAmounts.deposit, params.timestamps);

        // Load the stream ID.
        streamId = nextStreamId;

        // Effect: create the stream.
        _streams[streamId] = Lockup.Stream({
            amounts: Lockup.Amounts({ deposited: createAmounts.deposit, refunded: 0, withdrawn: 0 }),
            asset: params.asset,
            endTime: params.timestamps.end,
            isCancelable: params.cancelable,
            isDepleted: false,
            isStream: true,
            isTransferable: params.transferable,
            sender: params.sender,
            startTime: params.timestamps.start,
            wasCanceled: false
        });

        // Effect: set the cliff time if it is greater than zero.
        if (params.timestamps.cliff > 0) {
            _cliffs[streamId] = params.timestamps.cliff;
        }

        // Effect: bump the next stream ID.
        // Using unchecked arithmetic because these calculations cannot realistically overflow, ever.
        unchecked {
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
        emit ISablierV2LockupLinear.CreateLockupLinearStream({
            streamId: streamId,
            funder: msg.sender,
            sender: params.sender,
            recipient: params.recipient,
            amounts: createAmounts,
            asset: params.asset,
            cancelable: params.cancelable,
            transferable: params.transferable,
            timestamps: params.timestamps,
            broker: params.broker.account
        });
    }
}
