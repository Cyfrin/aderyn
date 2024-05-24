// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2LockupTranched } from "src/interfaces/ISablierV2LockupTranched.sol";
import { Errors } from "src/libraries/Errors.sol";
import { Lockup, LockupTranched } from "src/types/DataTypes.sol";

import { CreateWithDurations_Integration_Shared_Test } from "../../../shared/lockup/createWithDurations.t.sol";
import { LockupTranched_Integration_Concrete_Test } from "../LockupTranched.t.sol";

contract CreateWithDurations_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    CreateWithDurations_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, CreateWithDurations_Integration_Shared_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        CreateWithDurations_Integration_Shared_Test.setUp();
        streamId = lockupTranched.nextStreamId();
    }

    /// @dev it should revert.
    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData =
            abi.encodeCall(ISablierV2LockupTranched.createWithDurations, defaults.createWithDurationsLT());
        (bool success, bytes memory returnData) = address(lockupTranched).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    /// @dev it should revert.
    function test_RevertWhen_LoopCalculationOverflowsBlockGasLimit() external whenNotDelegateCalled {
        LockupTranched.TrancheWithDuration[] memory tranches = new LockupTranched.TrancheWithDuration[](25_000);
        vm.expectRevert();
        createDefaultStreamWithDurations(tranches);
    }

    function test_RevertWhen_DurationsZero()
        external
        whenNotDelegateCalled
        whenLoopCalculationsDoNotOverflowBlockGasLimit
    {
        uint40 startTime = getBlockTimestamp();
        LockupTranched.TrancheWithDuration[] memory tranches = defaults.createWithDurationsLT().tranches;
        tranches[2].duration = 0;
        uint256 index = 2;
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupTranched_TrancheTimestampsNotOrdered.selector,
                index,
                startTime + tranches[0].duration + tranches[1].duration,
                startTime + tranches[0].duration + tranches[1].duration
            )
        );
        createDefaultStreamWithDurations(tranches);
    }

    function test_RevertWhen_TimestampsCalculationsOverflows_StartTimeNotLessThanFirstTrancheTimestamp()
        external
        whenNotDelegateCalled
        whenLoopCalculationsDoNotOverflowBlockGasLimit
        whenDurationsNotZero
    {
        unchecked {
            uint40 startTime = getBlockTimestamp();
            LockupTranched.TrancheWithDuration[] memory tranches = defaults.tranchesWithDurations();
            tranches[0].duration = MAX_UINT40;
            vm.expectRevert(
                abi.encodeWithSelector(
                    Errors.SablierV2LockupTranched_StartTimeNotLessThanFirstTrancheTimestamp.selector,
                    startTime,
                    startTime + tranches[0].duration
                )
            );
            createDefaultStreamWithDurations(tranches);
        }
    }

    function test_RevertWhen_TimestampsCalculationsOverflows_TrancheTimestampsNotOrdered()
        external
        whenNotDelegateCalled
        whenLoopCalculationsDoNotOverflowBlockGasLimit
        whenDurationsNotZero
    {
        unchecked {
            uint40 startTime = getBlockTimestamp();

            // Create new tranches that overflow when the timestamps are eventually calculated.
            LockupTranched.TrancheWithDuration[] memory tranches = new LockupTranched.TrancheWithDuration[](2);
            tranches[0] = LockupTranched.TrancheWithDuration({ amount: 0, duration: startTime + 1 seconds });
            tranches[1] = defaults.tranchesWithDurations()[0];
            tranches[1].duration = MAX_UINT40;

            // Expect the relevant error to be thrown.
            uint256 index = 1;
            vm.expectRevert(
                abi.encodeWithSelector(
                    Errors.SablierV2LockupTranched_TrancheTimestampsNotOrdered.selector,
                    index,
                    startTime + tranches[0].duration,
                    startTime + tranches[0].duration + tranches[1].duration
                )
            );

            // Create the stream.
            createDefaultStreamWithDurations(tranches);
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
        LockupTranched.Timestamps memory timestamps =
            LockupTranched.Timestamps({ start: blockTimestamp, end: blockTimestamp + defaults.TOTAL_DURATION() });

        LockupTranched.TrancheWithDuration[] memory tranchesWithDurations = defaults.tranchesWithDurations();
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        tranches[0].timestamp = timestamps.start + tranchesWithDurations[0].duration;
        tranches[1].timestamp = tranches[0].timestamp + tranchesWithDurations[1].duration;
        tranches[2].timestamp = tranches[1].timestamp + tranchesWithDurations[2].duration;

        // Expect the assets to be transferred from the funder to {SablierV2LockupTranched}.
        expectCallToTransferFrom({ from: funder, to: address(lockupTranched), value: defaults.DEPOSIT_AMOUNT() });

        // Expect the broker fee to be paid to the broker.
        expectCallToTransferFrom({ from: funder, to: users.broker, value: defaults.BROKER_FEE_AMOUNT() });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockupTranched) });
        emit MetadataUpdate({ _tokenId: streamId });
        vm.expectEmit({ emitter: address(lockupTranched) });
        emit CreateLockupTranchedStream({
            streamId: streamId,
            funder: funder,
            sender: users.sender,
            recipient: users.recipient,
            amounts: defaults.lockupCreateAmounts(),
            asset: dai,
            cancelable: true,
            transferable: true,
            tranches: tranches,
            timestamps: timestamps,
            broker: users.broker
        });

        // Create the stream.
        createDefaultStreamWithDurations();

        // Assert that the stream has been created.
        LockupTranched.StreamLT memory actualStream = lockupTranched.getStream(streamId);
        LockupTranched.StreamLT memory expectedStream = defaults.lockupTranchedStream();
        expectedStream.endTime = timestamps.end;
        expectedStream.startTime = timestamps.start;
        expectedStream.tranches = tranches;
        assertEq(actualStream, expectedStream);

        // Assert that the stream's status is "STREAMING".
        Lockup.Status actualStatus = lockupTranched.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the next stream ID has been bumped.
        uint256 actualNextStreamId = lockupTranched.nextStreamId();
        uint256 expectedNextStreamId = streamId + 1;
        assertEq(actualNextStreamId, expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        address actualNFTOwner = lockupTranched.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }
}
