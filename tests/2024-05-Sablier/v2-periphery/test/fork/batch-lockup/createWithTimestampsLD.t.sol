// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { LockupDynamic } from "@sablier/v2-core/src/types/DataTypes.sol";

import { BatchLockup } from "src/types/DataTypes.sol";

import { ArrayBuilder } from "../../utils/ArrayBuilder.sol";
import { BatchLockupBuilder } from "../../utils/BatchLockupBuilder.sol";
import { Fork_Test } from "../Fork.t.sol";

/// @dev Runs against multiple fork assets.
abstract contract CreateWithTimestamps_LockupDynamic_BatchLockup_Fork_Test is Fork_Test {
    constructor(IERC20 asset_) Fork_Test(asset_) { }

    function setUp() public virtual override {
        Fork_Test.setUp();
    }

    struct CreateWithTimestampsParams {
        uint128 batchSize;
        address sender;
        address recipient;
        uint128 perStreamAmount;
        uint40 startTime;
        LockupDynamic.Segment[] segments;
    }

    function testForkFuzz_CreateWithTimestampsLD(CreateWithTimestampsParams memory params) external {
        vm.assume(params.segments.length != 0);
        params.batchSize = boundUint128(params.batchSize, 1, 20);
        params.startTime = boundUint40(params.startTime, getBlockTimestamp(), getBlockTimestamp() + 24 hours);
        fuzzSegmentTimestamps(params.segments, params.startTime);
        (params.perStreamAmount,) = fuzzDynamicStreamAmounts({
            upperBound: MAX_UINT128 / params.batchSize,
            segments: params.segments,
            brokerFee: defaults.BROKER_FEE()
        });

        checkUsers(params.sender, params.recipient);

        uint256 firstStreamId = lockupDynamic.nextStreamId();
        uint128 totalTransferAmount = params.perStreamAmount * params.batchSize;

        deal({ token: address(FORK_ASSET), to: params.sender, give: uint256(totalTransferAmount) });
        approveContract({ asset_: FORK_ASSET, from: params.sender, spender: address(batchLockup) });

        LockupDynamic.CreateWithTimestamps memory createWithTimestamps = LockupDynamic.CreateWithTimestamps({
            sender: params.sender,
            recipient: params.recipient,
            totalAmount: params.perStreamAmount,
            asset: FORK_ASSET,
            cancelable: true,
            transferable: true,
            startTime: params.startTime,
            segments: params.segments,
            broker: defaults.broker()
        });
        BatchLockup.CreateWithTimestampsLD[] memory batchParams =
            BatchLockupBuilder.fillBatch(createWithTimestamps, params.batchSize);

        expectCallToTransferFrom({
            asset_: address(FORK_ASSET),
            from: params.sender,
            to: address(batchLockup),
            amount: totalTransferAmount
        });
        expectMultipleCallsToCreateWithTimestampsLD({ count: uint64(params.batchSize), params: createWithTimestamps });
        expectMultipleCallsToTransferFrom({
            asset_: address(FORK_ASSET),
            count: uint64(params.batchSize),
            from: address(batchLockup),
            to: address(lockupDynamic),
            amount: params.perStreamAmount
        });

        uint256[] memory actualStreamIds = batchLockup.createWithTimestampsLD(lockupDynamic, FORK_ASSET, batchParams);
        uint256[] memory expectedStreamIds = ArrayBuilder.fillStreamIds(firstStreamId, params.batchSize);
        assertEq(actualStreamIds, expectedStreamIds);
    }
}
