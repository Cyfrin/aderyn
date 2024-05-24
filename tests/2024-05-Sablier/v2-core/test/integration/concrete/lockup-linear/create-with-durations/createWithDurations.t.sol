// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2LockupLinear } from "src/interfaces/ISablierV2LockupLinear.sol";
import { Errors } from "src/libraries/Errors.sol";
import { Lockup, LockupLinear } from "src/types/DataTypes.sol";

import { CreateWithDurations_Integration_Shared_Test } from "../../../shared/lockup/createWithDurations.t.sol";
import { LockupLinear_Integration_Concrete_Test } from "../LockupLinear.t.sol";

contract CreateWithDurations_LockupLinear_Integration_Concrete_Test is
    LockupLinear_Integration_Concrete_Test,
    CreateWithDurations_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupLinear_Integration_Concrete_Test, CreateWithDurations_Integration_Shared_Test)
    {
        LockupLinear_Integration_Concrete_Test.setUp();
        CreateWithDurations_Integration_Shared_Test.setUp();
    }

    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData =
            abi.encodeCall(ISablierV2LockupLinear.createWithDurations, defaults.createWithDurationsLL());
        (bool success, bytes memory returnData) = address(lockupLinear).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    function test_RevertWhen_CliffDurationCalculationOverflows() external whenNotDelegateCalled {
        uint40 startTime = getBlockTimestamp();
        uint40 cliffDuration = MAX_UINT40 - startTime + 2 seconds;
        uint40 totalDuration = defaults.TOTAL_DURATION();

        // Calculate the end time. Needs to be "unchecked" to avoid an overflow.
        uint40 cliffTime;
        unchecked {
            cliffTime = startTime + cliffDuration;
        }

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupLinear_StartTimeNotLessThanCliffTime.selector, startTime, cliffTime
            )
        );

        // Create the stream.
        createDefaultStreamWithDurations(LockupLinear.Durations({ cliff: cliffDuration, total: totalDuration }));
    }

    function test_RevertWhen_TotalDurationCalculationOverflows()
        external
        whenNotDelegateCalled
        whenCliffDurationCalculationDoesNotOverflow
    {
        uint40 startTime = getBlockTimestamp();
        LockupLinear.Durations memory durations =
            LockupLinear.Durations({ cliff: 0, total: MAX_UINT40 - startTime + 1 seconds });

        // Calculate the cliff time and the end time. Needs to be "unchecked" to allow an overflow.
        uint40 cliffTime;
        uint40 endTime;
        unchecked {
            cliffTime = startTime + durations.cliff;
            endTime = startTime + durations.total;
        }

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupLinear_StartTimeNotLessThanEndTime.selector, startTime, endTime
            )
        );

        // Create the stream.
        createDefaultStreamWithDurations(durations);
    }

    function test_CreateWithDurations()
        external
        whenNotDelegateCalled
        whenCliffDurationCalculationDoesNotOverflow
        whenTotalDurationCalculationDoesNotOverflow
    {
        // Make the Sender the stream's funder
        address funder = users.sender;

        // Declare the timestamps.
        uint40 blockTimestamp = getBlockTimestamp();
        LockupLinear.Timestamps memory timestamps = LockupLinear.Timestamps({
            start: blockTimestamp,
            cliff: blockTimestamp + defaults.CLIFF_DURATION(),
            end: blockTimestamp + defaults.TOTAL_DURATION()
        });

        // Expect the assets to be transferred from the funder to {SablierV2LockupLinear}.
        expectCallToTransferFrom({ from: funder, to: address(lockupLinear), value: defaults.DEPOSIT_AMOUNT() });

        // Expect the broker fee to be paid to the broker.
        expectCallToTransferFrom({ from: funder, to: users.broker, value: defaults.BROKER_FEE_AMOUNT() });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockupLinear) });
        emit MetadataUpdate({ _tokenId: streamId });
        vm.expectEmit({ emitter: address(lockupLinear) });
        emit CreateLockupLinearStream({
            streamId: streamId,
            funder: funder,
            sender: users.sender,
            recipient: users.recipient,
            amounts: defaults.lockupCreateAmounts(),
            asset: dai,
            cancelable: true,
            transferable: true,
            timestamps: timestamps,
            broker: users.broker
        });

        // Create the stream.
        createDefaultStreamWithDurations();

        // Assert that the stream has been created.
        LockupLinear.StreamLL memory actualStream = lockupLinear.getStream(streamId);
        LockupLinear.StreamLL memory expectedStream = defaults.lockupLinearStream();
        expectedStream.startTime = timestamps.start;
        expectedStream.cliffTime = timestamps.cliff;
        expectedStream.endTime = timestamps.end;
        assertEq(actualStream, expectedStream);

        // Assert that the stream's status is "STREAMING".
        Lockup.Status actualStatus = lockupLinear.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.STREAMING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the next stream ID has been bumped.
        uint256 actualNextStreamId = lockupLinear.nextStreamId();
        uint256 expectedNextStreamId = streamId + 1;
        assertEq(actualNextStreamId, expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        address actualNFTOwner = lockupLinear.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }
}
