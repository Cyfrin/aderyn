// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ud2x18 } from "@prb/math/src/UD2x18.sol";

import { ISablierV2LockupDynamic } from "src/interfaces/ISablierV2LockupDynamic.sol";
import { Errors } from "src/libraries/Errors.sol";
import { Lockup, LockupDynamic } from "src/types/DataTypes.sol";

import { CreateWithDurations_Integration_Shared_Test } from "../../../shared/lockup/createWithDurations.t.sol";
import { LockupDynamic_Integration_Concrete_Test } from "../LockupDynamic.t.sol";

contract CreateWithDurations_LockupDynamic_Integration_Concrete_Test is
    LockupDynamic_Integration_Concrete_Test,
    CreateWithDurations_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Concrete_Test, CreateWithDurations_Integration_Shared_Test)
    {
        LockupDynamic_Integration_Concrete_Test.setUp();
        CreateWithDurations_Integration_Shared_Test.setUp();
        streamId = lockupDynamic.nextStreamId();
    }

    /// @dev it should revert.
    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData =
            abi.encodeCall(ISablierV2LockupDynamic.createWithDurations, defaults.createWithDurationsLD());
        (bool success, bytes memory returnData) = address(lockupDynamic).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    /// @dev it should revert.
    function test_RevertWhen_LoopCalculationOverflowsBlockGasLimit() external whenNotDelegateCalled {
        LockupDynamic.SegmentWithDuration[] memory segments = new LockupDynamic.SegmentWithDuration[](250_000);
        vm.expectRevert(bytes(""));
        createDefaultStreamWithDurations(segments);
    }

    function test_RevertWhen_DurationsZero()
        external
        whenNotDelegateCalled
        whenLoopCalculationsDoNotOverflowBlockGasLimit
    {
        uint40 startTime = getBlockTimestamp();
        LockupDynamic.SegmentWithDuration[] memory segments = defaults.createWithDurationsLD().segments;
        segments[1].duration = 0;
        uint256 index = 1;
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupDynamic_SegmentTimestampsNotOrdered.selector,
                index,
                startTime + segments[0].duration,
                startTime + segments[0].duration
            )
        );
        createDefaultStreamWithDurations(segments);
    }

    function test_RevertWhen_TimestampsCalculationsOverflows_StartTimeNotLessThanFirstSegmentTimestamp()
        external
        whenNotDelegateCalled
        whenLoopCalculationsDoNotOverflowBlockGasLimit
        whenDurationsNotZero
    {
        unchecked {
            uint40 startTime = getBlockTimestamp();
            LockupDynamic.SegmentWithDuration[] memory segments = defaults.segmentsWithDurations();
            segments[0].duration = MAX_UINT40;
            vm.expectRevert(
                abi.encodeWithSelector(
                    Errors.SablierV2LockupDynamic_StartTimeNotLessThanFirstSegmentTimestamp.selector,
                    startTime,
                    startTime + segments[0].duration
                )
            );
            createDefaultStreamWithDurations(segments);
        }
    }

    function test_RevertWhen_TimestampsCalculationsOverflows_SegmentTimestampsNotOrdered()
        external
        whenNotDelegateCalled
        whenLoopCalculationsDoNotOverflowBlockGasLimit
        whenDurationsNotZero
    {
        unchecked {
            uint40 startTime = getBlockTimestamp();

            // Create new segments that overflow when the timestamps are eventually calculated.
            LockupDynamic.SegmentWithDuration[] memory segments = new LockupDynamic.SegmentWithDuration[](2);
            segments[0] = LockupDynamic.SegmentWithDuration({
                amount: 0,
                exponent: ud2x18(1e18),
                duration: startTime + 1 seconds
            });
            segments[1] = defaults.segmentsWithDurations()[0];
            segments[1].duration = MAX_UINT40;

            // Expect the relevant error to be thrown.
            uint256 index = 1;
            vm.expectRevert(
                abi.encodeWithSelector(
                    Errors.SablierV2LockupDynamic_SegmentTimestampsNotOrdered.selector,
                    index,
                    startTime + segments[0].duration,
                    startTime + segments[0].duration + segments[1].duration
                )
            );

            // Create the stream.
            createDefaultStreamWithDurations(segments);
        }
    }

    function test_CreateWithDurations()
        external
        whenNotDelegateCalled
        whenLoopCalculationsDoNotOverflowBlockGasLimit
        whenDurationsNotZero
        whenTimestampsCalculationsDoNotOverflow
    {
        // Make the Sender the stream's funder
        address funder = users.sender;

        // Declare the timestamps.
        uint40 blockTimestamp = getBlockTimestamp();
        LockupDynamic.Timestamps memory timestamps =
            LockupDynamic.Timestamps({ start: blockTimestamp, end: blockTimestamp + defaults.TOTAL_DURATION() });

        // Adjust the segments.
        LockupDynamic.SegmentWithDuration[] memory segmentsWithDurations = defaults.segmentsWithDurations();
        LockupDynamic.Segment[] memory segments = defaults.segments();
        segments[0].timestamp = timestamps.start + segmentsWithDurations[0].duration;
        segments[1].timestamp = segments[0].timestamp + segmentsWithDurations[1].duration;

        // Expect the assets to be transferred from the funder to {SablierV2LockupDynamic}.
        expectCallToTransferFrom({ from: funder, to: address(lockupDynamic), value: defaults.DEPOSIT_AMOUNT() });

        // Expect the broker fee to be paid to the broker.
        expectCallToTransferFrom({ from: funder, to: users.broker, value: defaults.BROKER_FEE_AMOUNT() });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockupDynamic) });
        emit MetadataUpdate({ _tokenId: streamId });
        vm.expectEmit({ emitter: address(lockupDynamic) });
        emit CreateLockupDynamicStream({
            streamId: streamId,
            funder: funder,
            sender: users.sender,
            recipient: users.recipient,
            amounts: defaults.lockupCreateAmounts(),
            asset: dai,
            cancelable: true,
            transferable: true,
            segments: segments,
            timestamps: timestamps,
            broker: users.broker
        });

        // Create the stream.
        createDefaultStreamWithDurations();

        // Assert that the stream has been created.
        LockupDynamic.StreamLD memory actualStream = lockupDynamic.getStream(streamId);
        LockupDynamic.StreamLD memory expectedStream = defaults.lockupDynamicStream();
        expectedStream.endTime = timestamps.end;
        expectedStream.segments = segments;
        expectedStream.startTime = timestamps.start;
        assertEq(actualStream, expectedStream);

        // Assert that the stream's status is "STREAMING".
        Lockup.Status actualStatus = lockupDynamic.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the next stream ID has been bumped.
        uint256 actualNextStreamId = lockupDynamic.nextStreamId();
        uint256 expectedNextStreamId = streamId + 1;
        assertEq(actualNextStreamId, expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        address actualNFTOwner = lockupDynamic.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }
}
