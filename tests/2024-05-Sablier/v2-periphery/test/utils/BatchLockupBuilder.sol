// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { LockupDynamic, LockupLinear, LockupTranched } from "@sablier/v2-core/src/types/DataTypes.sol";

import { BatchLockup } from "../../src/types/DataTypes.sol";

library BatchLockupBuilder {
    /// @notice Generates an array containing `batchSize` copies of `batchSingle`.
    function fillBatch(
        BatchLockup.CreateWithDurationsLD memory batchSingle,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithDurationsLD[] memory batch)
    {
        batch = new BatchLockup.CreateWithDurationsLD[](batchSize);
        for (uint256 i = 0; i < batchSize; ++i) {
            batch[i] = batchSingle;
        }
    }

    /// @notice Turns the `params` into an array of {BatchLockup.CreateWithDurationsLD} structs.
    function fillBatch(
        LockupDynamic.CreateWithDurations memory params,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithDurationsLD[] memory batch)
    {
        batch = new BatchLockup.CreateWithDurationsLD[](batchSize);
        BatchLockup.CreateWithDurationsLD memory batchSingle = BatchLockup.CreateWithDurationsLD({
            sender: params.sender,
            recipient: params.recipient,
            totalAmount: params.totalAmount,
            cancelable: params.cancelable,
            transferable: params.transferable,
            segments: params.segments,
            broker: params.broker
        });
        batch = fillBatch(batchSingle, batchSize);
    }

    /// @notice Generates an array containing `batchSize` copies of `batchSingle`.
    function fillBatch(
        BatchLockup.CreateWithDurationsLL memory batchSingle,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithDurationsLL[] memory batch)
    {
        batch = new BatchLockup.CreateWithDurationsLL[](batchSize);
        for (uint256 i = 0; i < batchSize; ++i) {
            batch[i] = batchSingle;
        }
    }

    /// @notice Turns the `params` into an array of {BatchLockup.CreateWithDurationsLL} structs.
    function fillBatch(
        LockupLinear.CreateWithDurations memory params,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithDurationsLL[] memory batch)
    {
        batch = new BatchLockup.CreateWithDurationsLL[](batchSize);
        BatchLockup.CreateWithDurationsLL memory batchSingle = BatchLockup.CreateWithDurationsLL({
            sender: params.sender,
            recipient: params.recipient,
            totalAmount: params.totalAmount,
            cancelable: params.cancelable,
            transferable: params.transferable,
            durations: params.durations,
            broker: params.broker
        });
        batch = fillBatch(batchSingle, batchSize);
    }

    /// @notice Generates an array containing `batchSize` copies of `batchSingle`.
    function fillBatch(
        BatchLockup.CreateWithDurationsLT memory batchSingle,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithDurationsLT[] memory batch)
    {
        batch = new BatchLockup.CreateWithDurationsLT[](batchSize);
        for (uint256 i = 0; i < batchSize; ++i) {
            batch[i] = batchSingle;
        }
    }

    /// @notice Turns the `params` into an array of {BatchLockup.CreateWithDurationsLT} structs.
    function fillBatch(
        LockupTranched.CreateWithDurations memory params,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithDurationsLT[] memory batch)
    {
        batch = new BatchLockup.CreateWithDurationsLT[](batchSize);
        BatchLockup.CreateWithDurationsLT memory batchSingle = BatchLockup.CreateWithDurationsLT({
            sender: params.sender,
            recipient: params.recipient,
            totalAmount: params.totalAmount,
            cancelable: params.cancelable,
            transferable: params.transferable,
            tranches: params.tranches,
            broker: params.broker
        });
        batch = fillBatch(batchSingle, batchSize);
    }

    /// @notice Generates an array containing `batchSize` copies of `batchSingle`.
    function fillBatch(
        BatchLockup.CreateWithTimestampsLD memory batchSingle,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithTimestampsLD[] memory batch)
    {
        batch = new BatchLockup.CreateWithTimestampsLD[](batchSize);
        for (uint256 i = 0; i < batchSize; ++i) {
            batch[i] = batchSingle;
        }
    }

    /// @notice Turns the `params` into an array of {BatchLockup.CreateWithTimestampsLDs} structs.
    function fillBatch(
        LockupDynamic.CreateWithTimestamps memory params,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithTimestampsLD[] memory batch)
    {
        batch = new BatchLockup.CreateWithTimestampsLD[](batchSize);
        BatchLockup.CreateWithTimestampsLD memory batchSingle = BatchLockup.CreateWithTimestampsLD({
            sender: params.sender,
            recipient: params.recipient,
            totalAmount: params.totalAmount,
            cancelable: params.cancelable,
            transferable: params.transferable,
            startTime: params.startTime,
            segments: params.segments,
            broker: params.broker
        });
        batch = fillBatch(batchSingle, batchSize);
    }

    /// @notice Generates an array containing `batchSize` copies of `batchSingle`.
    function fillBatch(
        BatchLockup.CreateWithTimestampsLL memory batchSingle,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithTimestampsLL[] memory batch)
    {
        batch = new BatchLockup.CreateWithTimestampsLL[](batchSize);
        for (uint256 i = 0; i < batchSize; ++i) {
            batch[i] = batchSingle;
        }
    }

    /// @notice Turns the `params` into an array of {BatchLockup.CreateWithTimestampsLL} structs.
    function fillBatch(
        LockupLinear.CreateWithTimestamps memory params,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithTimestampsLL[] memory batch)
    {
        batch = new BatchLockup.CreateWithTimestampsLL[](batchSize);
        BatchLockup.CreateWithTimestampsLL memory batchSingle = BatchLockup.CreateWithTimestampsLL({
            sender: params.sender,
            recipient: params.recipient,
            totalAmount: params.totalAmount,
            cancelable: params.cancelable,
            transferable: params.transferable,
            timestamps: params.timestamps,
            broker: params.broker
        });
        batch = fillBatch(batchSingle, batchSize);
    }

    /// @notice Generates an array containing `batchSize` copies of `batchSingle`.
    function fillBatch(
        BatchLockup.CreateWithTimestampsLT memory batchSingle,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithTimestampsLT[] memory batch)
    {
        batch = new BatchLockup.CreateWithTimestampsLT[](batchSize);
        for (uint256 i = 0; i < batchSize; ++i) {
            batch[i] = batchSingle;
        }
    }

    /// @notice Turns the `params` into an array of {BatchLockup.CreateWithTimestampsLT} structs.
    function fillBatch(
        LockupTranched.CreateWithTimestamps memory params,
        uint256 batchSize
    )
        internal
        pure
        returns (BatchLockup.CreateWithTimestampsLT[] memory batch)
    {
        batch = new BatchLockup.CreateWithTimestampsLT[](batchSize);
        BatchLockup.CreateWithTimestampsLT memory batchSingle = BatchLockup.CreateWithTimestampsLT({
            sender: params.sender,
            recipient: params.recipient,
            totalAmount: params.totalAmount,
            cancelable: params.cancelable,
            transferable: params.transferable,
            startTime: params.startTime,
            tranches: params.tranches,
            broker: params.broker
        });
        batch = fillBatch(batchSingle, batchSize);
    }
}
