// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { ISablierV2LockupLinear } from "src/interfaces/ISablierV2LockupLinear.sol";
import { LockupLinear } from "src/types/DataTypes.sol";

import { TimestampStore } from "../stores/TimestampStore.sol";
import { LockupStore } from "../stores/LockupStore.sol";
import { BaseHandler } from "./BaseHandler.sol";

/// @dev This contract is a complement of {LockupLinearHandler}. The goal is to bias the invariant calls
/// toward the lockup functions (especially the create stream functions) by creating multiple handlers for
/// the lockup contracts.
contract LockupLinearCreateHandler is BaseHandler {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    ISablierV2LockupLinear public lockupLinear;
    LockupStore public lockupStore;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(
        IERC20 asset_,
        TimestampStore timestampStore_,
        LockupStore lockupStore_,
        ISablierV2LockupLinear lockupLinear_
    )
        BaseHandler(asset_, timestampStore_)
    {
        lockupStore = lockupStore_;
        lockupLinear = lockupLinear_;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                 HANDLER FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    function createWithDurations(
        uint256 timeJumpSeed,
        LockupLinear.CreateWithDurations memory params
    )
        public
        instrument("createWithDurations")
        adjustTimestamp(timeJumpSeed)
        checkUsers(params.sender, params.recipient, params.broker.account)
        useNewSender(params.sender)
    {
        // We don't want to create more than a certain number of streams.
        if (lockupStore.lastStreamId() >= MAX_STREAM_COUNT) {
            return;
        }

        // Bound the stream parameters.
        params.broker.fee = _bound(params.broker.fee, 0, MAX_BROKER_FEE);
        params.durations.cliff = boundUint40(params.durations.cliff, 1 seconds, 2500 seconds);
        params.durations.total =
            boundUint40(params.durations.total, params.durations.cliff + 1 seconds, MAX_UNIX_TIMESTAMP);
        params.totalAmount = boundUint128(params.totalAmount, 1, 1_000_000_000e18);

        // Mint enough assets to the Sender.
        deal({ token: address(asset), to: params.sender, give: asset.balanceOf(params.sender) + params.totalAmount });

        // Approve {SablierV2LockupLinear} to spend the assets.
        asset.approve({ spender: address(lockupLinear), value: params.totalAmount });

        // Create the stream.
        params.asset = asset;
        uint256 streamId = lockupLinear.createWithDurations(params);

        // Store the stream ID.
        lockupStore.pushStreamId(streamId, params.sender, params.recipient);
    }

    function createWithTimestamps(
        uint256 timeJumpSeed,
        LockupLinear.CreateWithTimestamps memory params
    )
        public
        instrument("createWithTimestamps")
        adjustTimestamp(timeJumpSeed)
        checkUsers(params.sender, params.recipient, params.broker.account)
        useNewSender(params.sender)
    {
        // We don't want to create more than a certain number of streams.
        if (lockupStore.lastStreamId() >= MAX_STREAM_COUNT) {
            return;
        }

        uint40 blockTimestamp = getBlockTimestamp();
        params.broker.fee = _bound(params.broker.fee, 0, MAX_BROKER_FEE);
        params.timestamps.start = boundUint40(params.timestamps.start, 1 seconds, blockTimestamp);
        params.totalAmount = boundUint128(params.totalAmount, 1, 1_000_000_000e18);

        // The cliff time must be either zero or greater than the start time.
        if (params.timestamps.cliff > 0) {
            params.timestamps.cliff = boundUint40(
                params.timestamps.cliff, params.timestamps.start + 1 seconds, params.timestamps.start + 52 weeks
            );
        }

        // Bound the end time so that it is always greater than the start time, the cliff time, and the block timestamp.
        uint40 endTimeLowerBound = maxOfThree(params.timestamps.start, params.timestamps.cliff, blockTimestamp);
        params.timestamps.end = boundUint40(params.timestamps.end, endTimeLowerBound + 1 seconds, MAX_UNIX_TIMESTAMP);

        // Mint enough assets to the Sender.
        deal({ token: address(asset), to: params.sender, give: asset.balanceOf(params.sender) + params.totalAmount });

        // Approve {SablierV2LockupLinear} to spend the assets.
        asset.approve({ spender: address(lockupLinear), value: params.totalAmount });

        // Create the stream.
        params.asset = asset;
        uint256 streamId = lockupLinear.createWithTimestamps(params);

        // Store the stream ID.
        lockupStore.pushStreamId(streamId, params.sender, params.recipient);
    }
}
