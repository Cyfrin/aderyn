// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { Broker, LockupLinear } from "src/types/DataTypes.sol";

import { Lockup_Integration_Shared_Test } from "../lockup/Lockup.t.sol";

/// @notice Common testing logic needed by all {SablierV2LockupLinear} integration tests.
abstract contract LockupLinear_Integration_Shared_Test is Lockup_Integration_Shared_Test {
    struct Params {
        LockupLinear.CreateWithDurations createWithDurations;
        LockupLinear.CreateWithTimestamps createWithTimestamps;
    }

    /// @dev These have to be pre-declared so that `vm.expectRevert` does not expect a revert in `defaults`.
    /// See https://github.com/foundry-rs/foundry/issues/4762.
    Params private _params;

    function setUp() public virtual override {
        Lockup_Integration_Shared_Test.setUp();
        _params.createWithDurations = defaults.createWithDurationsLL();
        _params.createWithTimestamps = defaults.createWithTimestampsLL();
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStream() internal override returns (uint256 streamId) {
        streamId = lockupLinear.createWithTimestamps(_params.createWithTimestamps);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithAsset(IERC20 asset) internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.asset = asset;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithBroker(Broker memory broker) internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.broker = broker;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with durations.
    function createDefaultStreamWithDurations() internal returns (uint256 streamId) {
        streamId = lockupLinear.createWithDurations(_params.createWithDurations);
    }

    /// @dev Creates the default stream with the provided durations.
    function createDefaultStreamWithDurations(LockupLinear.Durations memory durations)
        internal
        returns (uint256 streamId)
    {
        LockupLinear.CreateWithDurations memory params = _params.createWithDurations;
        params.durations = durations;
        streamId = lockupLinear.createWithDurations(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamNotCancelable() internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.cancelable = false;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamNotTransferable() internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.transferable = false;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithEndTime(uint40 endTime) internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.timestamps.end = endTime;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithRecipient(address recipient) internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.recipient = recipient;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithSender(address sender) internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.sender = sender;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithStartTime(uint40 startTime) internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.timestamps.start = startTime;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with the provided timestamps.
    function createDefaultStreamWithTimestamps(LockupLinear.Timestamps memory timestamps)
        internal
        returns (uint256 streamId)
    {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.timestamps = timestamps;
        streamId = lockupLinear.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithTotalAmount(uint128 totalAmount) internal override returns (uint256 streamId) {
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.totalAmount = totalAmount;
        streamId = lockupLinear.createWithTimestamps(params);
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
        LockupLinear.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.recipient = recipient;
        params.sender = sender;
        streamId = lockupLinear.createWithTimestamps(params);
    }
}
