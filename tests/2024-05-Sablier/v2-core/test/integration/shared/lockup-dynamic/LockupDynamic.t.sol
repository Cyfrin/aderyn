// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { Broker, LockupDynamic } from "src/types/DataTypes.sol";

import { Lockup_Integration_Shared_Test } from "../lockup/Lockup.t.sol";

/// @notice Common testing logic needed across {SablierV2LockupDynamic} integration tests.
abstract contract LockupDynamic_Integration_Shared_Test is Lockup_Integration_Shared_Test {
    struct CreateParams {
        LockupDynamic.CreateWithDurations createWithDurations;
        LockupDynamic.CreateWithTimestamps createWithTimestamps;
    }

    /// @dev These have to be pre-declared so that `vm.expectRevert` does not expect a revert in `defaults`.
    /// See https://github.com/foundry-rs/foundry/issues/4762.
    CreateParams private _params;

    function setUp() public virtual override {
        Lockup_Integration_Shared_Test.setUp();

        _params.createWithDurations.sender = users.sender;
        _params.createWithDurations.recipient = users.recipient;
        _params.createWithDurations.totalAmount = defaults.TOTAL_AMOUNT();
        _params.createWithDurations.asset = dai;
        _params.createWithDurations.cancelable = true;
        _params.createWithDurations.transferable = true;
        _params.createWithDurations.broker = defaults.broker();

        _params.createWithTimestamps.sender = users.sender;
        _params.createWithTimestamps.recipient = users.recipient;
        _params.createWithTimestamps.totalAmount = defaults.TOTAL_AMOUNT();
        _params.createWithTimestamps.asset = dai;
        _params.createWithTimestamps.cancelable = true;
        _params.createWithTimestamps.transferable = true;
        _params.createWithTimestamps.startTime = defaults.START_TIME();
        _params.createWithTimestamps.broker = defaults.broker();

        // See https://github.com/ethereum/solidity/issues/12783
        LockupDynamic.SegmentWithDuration[] memory segmentsWithDurations = defaults.segmentsWithDurations();
        LockupDynamic.Segment[] memory segments = defaults.segments();
        for (uint256 i = 0; i < defaults.SEGMENT_COUNT(); ++i) {
            _params.createWithDurations.segments.push(segmentsWithDurations[i]);
            _params.createWithTimestamps.segments.push(segments[i]);
        }
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStream() internal override returns (uint256 streamId) {
        streamId = lockupDynamic.createWithTimestamps(_params.createWithTimestamps);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithAsset(IERC20 asset) internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.asset = asset;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithBroker(Broker memory broker) internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.broker = broker;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with durations.
    function createDefaultStreamWithDurations() internal returns (uint256 streamId) {
        streamId = lockupDynamic.createWithDurations(_params.createWithDurations);
    }

    /// @dev Creates the default stream with the provided durations.
    function createDefaultStreamWithDurations(LockupDynamic.SegmentWithDuration[] memory segments)
        internal
        returns (uint256 streamId)
    {
        LockupDynamic.CreateWithDurations memory params = _params.createWithDurations;
        params.segments = segments;
        streamId = lockupDynamic.createWithDurations(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithEndTime(uint40 endTime) internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.segments[1].timestamp = endTime;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamNotCancelable() internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.cancelable = false;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamNotTransferable() internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.transferable = false;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithRecipient(address recipient) internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.recipient = recipient;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with the provided segments.
    function createDefaultStreamWithSegments(LockupDynamic.Segment[] memory segments)
        internal
        returns (uint256 streamId)
    {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.segments = segments;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithSender(address sender) internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.sender = sender;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithStartTime(uint40 startTime) internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.startTime = startTime;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with the provided timestamps.
    function createDefaultStreamWithTimestamps(LockupDynamic.Timestamps memory timestamps)
        internal
        returns (uint256 streamId)
    {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.startTime = timestamps.start;
        params.segments[1].timestamp = timestamps.end;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithTotalAmount(uint128 totalAmount) internal override returns (uint256 streamId) {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.totalAmount = totalAmount;
        streamId = lockupDynamic.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithUsers(
        address recipient,
        address sender
    )
        internal
        override
        returns (uint256 streamId)
    {
        LockupDynamic.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.recipient = recipient;
        params.sender = sender;
        streamId = lockupDynamic.createWithTimestamps(params);
    }
}
