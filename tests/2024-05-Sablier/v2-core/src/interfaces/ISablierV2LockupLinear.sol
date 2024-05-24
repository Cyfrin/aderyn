// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { Lockup, LockupLinear } from "../types/DataTypes.sol";
import { ISablierV2Lockup } from "./ISablierV2Lockup.sol";

/// @title ISablierV2LockupLinear
/// @notice Creates and manages Lockup streams with a linear distribution function.
interface ISablierV2LockupLinear is ISablierV2Lockup {
    /*//////////////////////////////////////////////////////////////////////////
                                       EVENTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a stream is created.
    /// @param streamId The ID of the newly created stream.
    /// @param funder The address which funded the stream.
    /// @param sender The address distributing the assets, which will have the ability to cancel the stream.
    /// @param recipient The address receiving the assets.
    /// @param amounts Struct containing (i) the deposit amount, and (ii) the broker fee amount, both denoted
    /// in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Boolean indicating whether the stream will be cancelable or not.
    /// @param transferable Boolean indicating whether the stream NFT is transferable or not.
    /// @param timestamps Struct containing (i) the stream's start time, (ii) cliff time, and (iii) end time, all as
    /// Unix timestamps.
    /// @param broker The address of the broker who has helped create the stream, e.g. a front-end website.
    event CreateLockupLinearStream(
        uint256 streamId,
        address funder,
        address indexed sender,
        address indexed recipient,
        Lockup.CreateAmounts amounts,
        IERC20 indexed asset,
        bool cancelable,
        bool transferable,
        LockupLinear.Timestamps timestamps,
        address broker
    );

    /*//////////////////////////////////////////////////////////////////////////
                                 CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Retrieves the stream's cliff time, which is a Unix timestamp.  A value of zero means there
    /// is no cliff.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getCliffTime(uint256 streamId) external view returns (uint40 cliffTime);

    /// @notice Retrieves the full stream details.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    /// @return stream See the documentation in {DataTypes}.
    function getStream(uint256 streamId) external view returns (LockupLinear.StreamLL memory stream);

    /// @notice Retrieves the stream's start, cliff and end timestamps.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    /// @return timestamps See the documentation in {DataTypes}.
    function getTimestamps(uint256 streamId) external view returns (LockupLinear.Timestamps memory timestamps);

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates a stream by setting the start time to `block.timestamp`, and the end time to
    /// the sum of `block.timestamp` and `params.durations.total`. The stream is funded by `msg.sender` and is wrapped
    /// in an ERC-721 NFT.
    ///
    /// @dev Emits a {Transfer} and {CreateLockupLinearStream} event.
    ///
    /// Requirements:
    /// - All requirements in {createWithTimestamps} must be met for the calculated parameters.
    ///
    /// @param params Struct encapsulating the function parameters, which are documented in {DataTypes}.
    /// @return streamId The ID of the newly created stream.
    function createWithDurations(LockupLinear.CreateWithDurations calldata params)
        external
        returns (uint256 streamId);

    /// @notice Creates a stream with the provided start time and end time. The stream is funded by `msg.sender` and is
    /// wrapped in an ERC-721 NFT.
    ///
    /// @dev Emits a {Transfer} and {CreateLockupLinearStream} event.
    ///
    /// Notes:
    /// - A cliff time of zero means there is no cliff.
    /// - As long as the times are ordered, it is not an error for the start or the cliff time to be in the past.
    ///
    /// Requirements:
    /// - Must not be delegate called.
    /// - `params.totalAmount` must be greater than zero.
    /// - If set, `params.broker.fee` must not be greater than `MAX_BROKER_FEE`.
    /// - `params.timestamps.start` must be greater than zero and less than `params.timestamps.end`.
    /// - If set, `params.timestamps.cliff` must be greater than `params.timestamps.start` and less than
    /// `params.timestamps.end`.
    /// - `params.timestamps.end` must be in the future.
    /// - `params.recipient` must not be the zero address.
    /// - `msg.sender` must have allowed this contract to spend at least `params.totalAmount` assets.
    ///
    /// @param params Struct encapsulating the function parameters, which are documented in {DataTypes}.
    /// @return streamId The ID of the newly created stream.
    function createWithTimestamps(LockupLinear.CreateWithTimestamps calldata params)
        external
        returns (uint256 streamId);
}
