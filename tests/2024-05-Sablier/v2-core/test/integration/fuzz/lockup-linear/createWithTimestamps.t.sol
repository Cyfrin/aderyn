// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { MAX_UD60x18, ud } from "@prb/math/src/UD60x18.sol";

import { Errors } from "src/libraries/Errors.sol";
import { Broker, Lockup, LockupLinear } from "src/types/DataTypes.sol";

import { CreateWithTimestamps_Integration_Shared_Test } from "../../shared/lockup/createWithTimestamps.t.sol";
import { LockupLinear_Integration_Fuzz_Test } from "./LockupLinear.t.sol";

contract CreateWithTimestamps_LockupLinear_Integration_Fuzz_Test is
    LockupLinear_Integration_Fuzz_Test,
    CreateWithTimestamps_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupLinear_Integration_Fuzz_Test, CreateWithTimestamps_Integration_Shared_Test)
    {
        LockupLinear_Integration_Fuzz_Test.setUp();
        CreateWithTimestamps_Integration_Shared_Test.setUp();
    }

    function testFuzz_RevertWhen_BrokerFeeTooHigh(Broker memory broker)
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
    {
        vm.assume(broker.account != address(0));
        broker.fee = _bound(broker.fee, MAX_BROKER_FEE + ud(1), MAX_UD60x18);
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2Lockup_BrokerFeeTooHigh.selector, broker.fee, MAX_BROKER_FEE)
        );
        createDefaultStreamWithBroker(broker);
    }

    function testFuzz_RevertWhen_StartTimeNotLessThanCliffTime(uint40 startTime)
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
    {
        startTime = boundUint40(startTime, defaults.CLIFF_TIME() + 1 seconds, defaults.END_TIME() - 1 seconds);
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupLinear_StartTimeNotLessThanCliffTime.selector, startTime, defaults.CLIFF_TIME()
            )
        );
        createDefaultStreamWithStartTime(startTime);
    }

    function testFuzz_RevertWhen_CliffTimeNotLessThanEndTime(
        uint40 cliffTime,
        uint40 endTime
    )
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
    {
        uint40 startTime = defaults.START_TIME();
        endTime = boundUint40(endTime, startTime + 1 seconds, startTime + 2 weeks);
        cliffTime = boundUint40(cliffTime, endTime, MAX_UNIX_TIMESTAMP);

        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupLinear_CliffTimeNotLessThanEndTime.selector, cliffTime, endTime
            )
        );
        createDefaultStreamWithTimestamps(LockupLinear.Timestamps({ start: startTime, cliff: cliffTime, end: endTime }));
    }

    struct Vars {
        uint256 actualNextStreamId;
        address actualNFTOwner;
        Lockup.Status actualStatus;
        Lockup.CreateAmounts createAmounts;
        uint256 expectedNextStreamId;
        address expectedNFTOwner;
        Lockup.Status expectedStatus;
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - All possible permutations for the funder, sender, recipient, and broker
    /// - Multiple values for the total amount
    /// - Cancelable and not cancelable
    /// - Start time in the past
    /// - Start time in the present
    /// - Start time in the future
    /// - Start time lower than and equal to cliff time
    /// - Cliff time zero and not zero
    /// - Multiple values for the cliff time and the end time
    /// - Multiple values for the broker fee, including zero
    function testFuzz_CreateWithTimestamps(
        address funder,
        LockupLinear.CreateWithTimestamps memory params
    )
        external
        whenNotDelegateCalled
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeLessThanEndTime
        whenEndTimeInTheFuture
        whenBrokerFeeNotTooHigh
        whenAssetContract
        whenAssetERC20
    {
        vm.assume(funder != address(0) && params.recipient != address(0) && params.broker.account != address(0));
        vm.assume(params.totalAmount != 0);
        params.timestamps.start =
            boundUint40(params.timestamps.start, defaults.START_TIME(), defaults.START_TIME() + 10_000 seconds);
        params.broker.fee = _bound(params.broker.fee, 0, MAX_BROKER_FEE);
        params.transferable = true;

        // The cliff time must be either zero or greater than the start time.
        if (params.timestamps.cliff > 0) {
            params.timestamps.cliff = boundUint40(
                params.timestamps.cliff, params.timestamps.start + 1 seconds, params.timestamps.start + 52 weeks
            );
            params.timestamps.end =
                boundUint40(params.timestamps.end, params.timestamps.cliff + 1 seconds, MAX_UNIX_TIMESTAMP);
        } else {
            params.timestamps.end =
                boundUint40(params.timestamps.end, params.timestamps.start + 1 seconds, MAX_UNIX_TIMESTAMP);
        }

        // Calculate the fee amounts and the deposit amount.
        Vars memory vars;

        vars.createAmounts.brokerFee = ud(params.totalAmount).mul(params.broker.fee).intoUint128();
        vars.createAmounts.deposit = params.totalAmount - vars.createAmounts.brokerFee;

        // Make the fuzzed funder the caller in this test.
        resetPrank(funder);

        // Mint enough assets to the funder.
        deal({ token: address(dai), to: funder, give: params.totalAmount });

        // Approve {SablierV2LockupLinear} to transfer the assets from the fuzzed funder.
        dai.approve({ spender: address(lockupLinear), value: MAX_UINT256 });

        // Expect the assets to be transferred from the funder to {SablierV2LockupLinear}.
        expectCallToTransferFrom({ from: funder, to: address(lockupLinear), value: vars.createAmounts.deposit });

        // Expect the broker fee to be paid to the broker, if not zero.
        if (vars.createAmounts.brokerFee > 0) {
            expectCallToTransferFrom({ from: funder, to: params.broker.account, value: vars.createAmounts.brokerFee });
        }

        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockupLinear) });
        emit CreateLockupLinearStream({
            streamId: streamId,
            funder: funder,
            sender: params.sender,
            recipient: params.recipient,
            amounts: vars.createAmounts,
            asset: dai,
            cancelable: params.cancelable,
            transferable: params.transferable,
            timestamps: params.timestamps,
            broker: params.broker.account
        });

        // Create the stream.
        lockupLinear.createWithTimestamps(
            LockupLinear.CreateWithTimestamps({
                sender: params.sender,
                recipient: params.recipient,
                totalAmount: params.totalAmount,
                asset: dai,
                cancelable: params.cancelable,
                transferable: params.transferable,
                timestamps: params.timestamps,
                broker: params.broker
            })
        );

        // Assert that the stream has been created.
        LockupLinear.StreamLL memory actualStream = lockupLinear.getStream(streamId);
        assertEq(actualStream.amounts, Lockup.Amounts(vars.createAmounts.deposit, 0, 0));
        assertEq(actualStream.asset, dai, "asset");
        assertEq(actualStream.cliffTime, params.timestamps.cliff, "cliffTime");
        assertEq(actualStream.endTime, params.timestamps.end, "endTime");
        assertEq(actualStream.isCancelable, params.cancelable, "isCancelable");
        assertEq(actualStream.isDepleted, false, "isStream");
        assertEq(actualStream.isStream, true, "isStream");
        assertEq(actualStream.isTransferable, true, "isTransferable");
        assertEq(actualStream.recipient, params.recipient, "recipient");
        assertEq(actualStream.sender, params.sender, "sender");
        assertEq(actualStream.startTime, params.timestamps.start, "startTime");
        assertEq(actualStream.wasCanceled, false, "wasCanceled");

        // Assert that the stream's status is correct.
        vars.actualStatus = lockupLinear.statusOf(streamId);
        vars.expectedStatus =
            params.timestamps.start > getBlockTimestamp() ? Lockup.Status.PENDING : Lockup.Status.STREAMING;
        assertEq(vars.actualStatus, vars.expectedStatus);

        // Assert that the next stream ID has been bumped.
        vars.actualNextStreamId = lockupLinear.nextStreamId();
        vars.expectedNextStreamId = streamId + 1;
        assertEq(vars.actualNextStreamId, vars.expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        vars.actualNFTOwner = lockupLinear.ownerOf({ tokenId: streamId });
        vars.expectedNFTOwner = params.recipient;
        assertEq(vars.actualNFTOwner, vars.expectedNFTOwner, "NFT owner");
    }
}
