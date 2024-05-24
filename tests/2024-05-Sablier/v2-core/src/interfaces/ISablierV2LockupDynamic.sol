// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { Lockup, LockupDynamic } from "../types/DataTypes.sol";
import { ISablierV2Lockup } from "./ISablierV2Lockup.sol";

/// @title ISablierV2LockupDynamic
/// @notice Creates and manages Lockup streams with a dynamic distribution function.
interface ISablierV2LockupDynamic is ISablierV2Lockup {
    /*//////////////////////////////////////////////////////////////////////////
                                       EVENTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a stream is created.
    /// @param streamId The ID of the newly created stream.
    /// @param funder The address which has funded the stream.
    /// @param sender The address distributing the assets, which will have the ability to cancel the stream.
    /// @param recipient The address toward which to stream the assets.
    /// @param amounts Struct containing (i) the deposit amount, and (ii) the broker fee amount, both denoted
    /// in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Boolean indicating whether the stream will be cancelable or not.
    /// @param transferable Boolean indicating whether the stream NFT is transferable or not.
    /// @param segments The segments the protocol uses to compose the dynamic distribution function.
    /// @param timestamps Struct containing (i) the stream's start time and (ii) end time, both as Unix timestamps.
    /// @param broker The address of the broker who has helped create the stream, e.g. a front-end website.
    event CreateLockupDynamicStream(
        uint256 streamId,
        address funder,
        address indexed sender,
        address indexed recipient,
        Lockup.CreateAmounts amounts,
        IERC20 indexed asset,
        bool cancelable,
        bool transferable,
        LockupDynamic.Segment[] segments,
        LockupDynamic.Timestamps timestamps,
        address broker
    );

    /*//////////////////////////////////////////////////////////////////////////
                                 CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Retrieves the segments used to compose the dynamic distribution function.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getSegments(uint256 streamId) external view returns (LockupDynamic.Segment[] memory segments);

    /// @notice Retrieves the full stream details.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    /// @return stream See the documentation in {DataTypes}.
    function getStream(uint256 streamId) external view returns (LockupDynamic.StreamLD memory stream);

    /// @notice Retrieves the stream's start and end timestamps.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    /// @return timestamps See the documentation in {DataTypes}.
    function getTimestamps(uint256 streamId) external view returns (LockupDynamic.Timestamps memory timestamps);

    /// @notice The maximum number of segments allowed in a stream.
    /// @dev This is initialized at construction time and cannot be changed later.
    function MAX_SEGMENT_COUNT() external view returns (uint256);

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates a stream by setting the start time to `block.timestamp`, and the end time to the sum of
    /// `block.timestamp` and all specified time durations. The segment timestamps are derived from these
    /// durations. The stream is funded by `msg.sender` and is wrapped in an ERC-721 NFT.
    ///
    /// @dev Emits a {Transfer} and {CreateLockupDynamicStream} event.
    ///
    /// Requirements:
    /// - All requirements in {createWithTimestamps} must be met for the calculated parameters.
    ///
    /// @param params Struct encapsulating the function parameters, which are documented in {DataTypes}.
    /// @return streamId The ID of the newly created stream.
    function createWithDurations(LockupDynamic.CreateWithDurations calldata params)
        external
        returns (uint256 streamId);

    /// @notice Creates a stream with the provided segment timestamps, implying the end time from the last timestamp.
    /// The stream is funded by `msg.sender` and is wrapped in an ERC-721 NFT.
    ///
    /// @dev Emits a {Transfer} and {CreateLockupDynamicStream} event.
    ///
    /// Notes:
    /// - As long as the segment timestamps are arranged in ascending order, it is not an error for some
    /// of them to be in the past.
    ///
    /// Requirements:
    /// - Must not be delegate called.
    /// - `params.totalAmount` must be greater than zero.
    /// - If set, `params.broker.fee` must not be greater than `MAX_BROKER_FEE`.
    /// - `params.startTime` must be greater than zero and less than the first segment's timestamp.
    /// - `params.segments` must have at least one segment, but not more than `MAX_SEGMENT_COUNT`.
    /// - The segment timestamps must be arranged in ascending order.
    /// - The last segment timestamp (i.e. the stream's end time) must be in the future.
    /// - The sum of the segment amounts must equal the deposit amount.
    /// - `params.recipient` must not be the zero address.
    /// - `msg.sender` must have allowed this contract to spend at least `params.totalAmount` assets.
    ///
    /// @param params Struct encapsulating the function parameters, which are documented in {DataTypes}.
    /// @return streamId The ID of the newly created stream.
    function createWithTimestamps(LockupDynamic.CreateWithTimestamps calldata params)
        external
        returns (uint256 streamId);
}
