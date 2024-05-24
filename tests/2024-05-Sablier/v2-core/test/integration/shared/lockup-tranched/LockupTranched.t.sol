// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { Broker, LockupTranched } from "src/types/DataTypes.sol";

import { Lockup_Integration_Shared_Test } from "../lockup/Lockup.t.sol";

/// @notice Common testing logic needed across {SablierV2LockupTranched} integration tests.
abstract contract LockupTranched_Integration_Shared_Test is Lockup_Integration_Shared_Test {
    struct CreateParams {
        LockupTranched.CreateWithDurations createWithDurations;
        LockupTranched.CreateWithTimestamps createWithTimestamps;
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
        LockupTranched.TrancheWithDuration[] memory tranchesWithDurations = defaults.tranchesWithDurations();
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        for (uint256 i = 0; i < defaults.TRANCHE_COUNT(); ++i) {
            _params.createWithDurations.tranches.push(tranchesWithDurations[i]);
            _params.createWithTimestamps.tranches.push(tranches[i]);
        }
    }

    /// @dev Creates the default stream.
    function createDefaultStream() internal override returns (uint256 streamId) {
        streamId = lockupTranched.createWithTimestamps(_params.createWithTimestamps);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithAsset(IERC20 asset) internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.asset = asset;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithBroker(Broker memory broker) internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.broker = broker;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with durations.
    function createDefaultStreamWithDurations() internal returns (uint256 streamId) {
        streamId = lockupTranched.createWithDurations(_params.createWithDurations);
    }

    /// @dev Creates the default stream with the provided durations.
    function createDefaultStreamWithDurations(LockupTranched.TrancheWithDuration[] memory tranches)
        internal
        returns (uint256 streamId)
    {
        LockupTranched.CreateWithDurations memory params = _params.createWithDurations;
        params.tranches = tranches;
        streamId = lockupTranched.createWithDurations(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithEndTime(uint40 endTime) internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.tranches[2].timestamp = endTime;

        // Ensure the timestamps are arranged in ascending order.
        if (params.tranches[2].timestamp <= params.tranches[1].timestamp) {
            params.tranches[1].timestamp = params.tranches[2].timestamp - 1;
        }
        if (params.tranches[1].timestamp <= params.tranches[0].timestamp) {
            params.tranches[0].timestamp = params.tranches[1].timestamp - 1;
        }

        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamNotCancelable() internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.cancelable = false;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamNotTransferable() internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.transferable = false;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with the provided timestamps.
    function createDefaultStreamWithTimestamps(LockupTranched.Timestamps memory timestamps)
        internal
        returns (uint256 streamId)
    {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.startTime = timestamps.start;
        params.tranches[1].timestamp = timestamps.end;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithRecipient(address recipient) internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.recipient = recipient;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @dev Creates the default stream with the provided tranches.
    function createDefaultStreamWithTranches(LockupTranched.Tranche[] memory tranches)
        internal
        returns (uint256 streamId)
    {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.tranches = tranches;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithSender(address sender) internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.sender = sender;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithStartTime(uint40 startTime) internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.startTime = startTime;
        streamId = lockupTranched.createWithTimestamps(params);
    }

    /// @inheritdoc Lockup_Integration_Shared_Test
    function createDefaultStreamWithTotalAmount(uint128 totalAmount) internal override returns (uint256 streamId) {
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.totalAmount = totalAmount;
        streamId = lockupTranched.createWithTimestamps(params);
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
        LockupTranched.CreateWithTimestamps memory params = _params.createWithTimestamps;
        params.recipient = recipient;
        params.sender = sender;
        streamId = lockupTranched.createWithTimestamps(params);
    }
}
