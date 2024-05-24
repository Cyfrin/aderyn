// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { MAX_UD60x18, UD60x18, ud, ZERO } from "@prb/math/src/UD60x18.sol";
import { stdError } from "forge-std/src/StdError.sol";

import { Errors } from "src/libraries/Errors.sol";
import { Broker, Lockup, LockupTranched } from "src/types/DataTypes.sol";

import { CreateWithTimestamps_Integration_Shared_Test } from "../../shared/lockup/createWithTimestamps.t.sol";
import { LockupTranched_Integration_Fuzz_Test } from "./LockupTranched.t.sol";

contract CreateWithTimestamps_LockupTranched_Integration_Fuzz_Test is
    LockupTranched_Integration_Fuzz_Test,
    CreateWithTimestamps_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Fuzz_Test, CreateWithTimestamps_Integration_Shared_Test)
    {
        LockupTranched_Integration_Fuzz_Test.setUp();
        CreateWithTimestamps_Integration_Shared_Test.setUp();
    }

    function testFuzz_RevertWhen_TrancheCountTooHigh(uint256 trancheCount)
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenTrancheCountNotZero
    {
        uint256 defaultMax = defaults.MAX_TRANCHE_COUNT();
        trancheCount = _bound(trancheCount, defaultMax + 1, defaultMax * 10);
        LockupTranched.Tranche[] memory tranches = new LockupTranched.Tranche[](trancheCount);
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2LockupTranched_TrancheCountTooHigh.selector, trancheCount)
        );
        createDefaultStreamWithTranches(tranches);
    }

    function testFuzz_RevertWhen_TrancheAmountsSumOverflows(
        uint128 amount0,
        uint128 amount1
    )
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
    {
        amount0 = boundUint128(amount0, MAX_UINT128 / 2 + 1, MAX_UINT128);
        amount1 = boundUint128(amount0, MAX_UINT128 / 2 + 1, MAX_UINT128);
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        tranches[0].amount = amount0;
        tranches[1].amount = amount1;
        vm.expectRevert(stdError.arithmeticError);
        createDefaultStreamWithTranches(tranches);
    }

    function testFuzz_RevertWhen_StartTimeNotLessThanFirstTrancheTimestamp(uint40 firstTimestamp)
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
    {
        firstTimestamp = boundUint40(firstTimestamp, 0, defaults.START_TIME());

        // Change the timestamp of the first tranche.
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        tranches[0].timestamp = firstTimestamp;

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupTranched_StartTimeNotLessThanFirstTrancheTimestamp.selector,
                defaults.START_TIME(),
                tranches[0].timestamp
            )
        );

        // Create the stream.
        createDefaultStreamWithTranches(tranches);
    }

    function testFuzz_RevertWhen_DepositAmountNotEqualToTrancheAmountsSum(uint128 depositDiff)
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
    {
        depositDiff = boundUint128(depositDiff, 100, defaults.TOTAL_AMOUNT());

        UD60x18 brokerFee = ZERO;
        resetPrank({ msgSender: users.sender });

        // Adjust the default deposit amount.
        uint128 defaultDepositAmount = defaults.DEPOSIT_AMOUNT();
        uint128 depositAmount = defaultDepositAmount + depositDiff;

        // Prepare the params.
        LockupTranched.CreateWithTimestamps memory params = defaults.createWithTimestampsLT();
        params.broker = Broker({ account: address(0), fee: brokerFee });
        params.totalAmount = depositAmount;

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupTranched_DepositAmountNotEqualToTrancheAmountsSum.selector,
                depositAmount,
                defaultDepositAmount
            )
        );

        // Create the stream.
        lockupTranched.createWithTimestamps(params);
    }

    function testFuzz_RevertWhen_BrokerFeeTooHigh(Broker memory broker)
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToTrancheAmountsSum
    {
        vm.assume(broker.account != address(0));
        broker.fee = _bound(broker.fee, MAX_BROKER_FEE + ud(1), MAX_UD60x18);
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2Lockup_BrokerFeeTooHigh.selector, broker.fee, MAX_BROKER_FEE)
        );
        createDefaultStreamWithBroker(broker);
    }

    struct Vars {
        uint256 actualNextStreamId;
        address actualNFTOwner;
        Lockup.Status actualStatus;
        Lockup.CreateAmounts createAmounts;
        uint256 expectedNextStreamId;
        address expectedNFTOwner;
        Lockup.Status expectedStatus;
        bool isCancelable;
        bool isSettled;
        uint128 totalAmount;
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - All possible permutations for the funder, sender, recipient, and broker
    /// - Multiple values for the tranche amounts, exponents, and timestamps
    /// - Cancelable and not cancelable
    /// - Start time in the past
    /// - Start time in the present
    /// - Start time in the future
    /// - Start time equal and not equal to the first tranche timestamp
    /// - Multiple values for the broker fee, including zero
    function testFuzz_CreateWithTimestamps(
        address funder,
        LockupTranched.CreateWithTimestamps memory params
    )
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToTrancheAmountsSum
        whenBrokerFeeNotTooHigh
        whenAssetContract
        whenAssetERC20
    {
        vm.assume(funder != address(0) && params.recipient != address(0) && params.broker.account != address(0));
        vm.assume(params.tranches.length != 0);
        params.broker.fee = _bound(params.broker.fee, 0, MAX_BROKER_FEE);

        params.startTime = boundUint40(params.startTime, 1, defaults.START_TIME());
        params.transferable = true;

        // Fuzz the tranche timestamps.
        fuzzTrancheTimestamps(params.tranches, params.startTime);

        // Fuzz the tranche amounts and calculate the total and create amounts (deposit and broker fee).
        Vars memory vars;
        (vars.totalAmount, vars.createAmounts) = fuzzTranchedStreamAmounts({
            upperBound: MAX_UINT128,
            tranches: params.tranches,
            brokerFee: params.broker.fee
        });

        // Make the fuzzed funder the caller in the rest of this test.
        resetPrank(funder);

        // Mint enough assets to the fuzzed funder.
        deal({ token: address(dai), to: funder, give: vars.totalAmount });

        // Approve {SablierV2LockupTranched} to transfer the assets from the fuzzed funder.
        dai.approve({ spender: address(lockupTranched), value: MAX_UINT256 });

        // Expect the assets to be transferred from the funder to {SablierV2LockupTranched}.
        expectCallToTransferFrom({ from: funder, to: address(lockupTranched), value: vars.createAmounts.deposit });

        // Expect the broker fee to be paid to the broker, if not zero.
        if (vars.createAmounts.brokerFee > 0) {
            expectCallToTransferFrom({ from: funder, to: params.broker.account, value: vars.createAmounts.brokerFee });
        }

        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockupTranched) });
        LockupTranched.Timestamps memory timestamps = LockupTranched.Timestamps({
            start: params.startTime,
            end: params.tranches[params.tranches.length - 1].timestamp
        });
        emit CreateLockupTranchedStream({
            streamId: streamId,
            funder: funder,
            sender: params.sender,
            recipient: params.recipient,
            amounts: vars.createAmounts,
            asset: dai,
            cancelable: params.cancelable,
            transferable: params.transferable,
            tranches: params.tranches,
            timestamps: timestamps,
            broker: params.broker.account
        });

        // Create the stream.
        lockupTranched.createWithTimestamps(
            LockupTranched.CreateWithTimestamps({
                sender: params.sender,
                recipient: params.recipient,
                totalAmount: vars.totalAmount,
                asset: dai,
                cancelable: params.cancelable,
                transferable: params.transferable,
                startTime: params.startTime,
                tranches: params.tranches,
                broker: params.broker
            })
        );

        // Check if the stream is settled. It is possible for a Lockup Tranched stream to settle at the time of creation
        // because some tranche amounts can be zero.
        vars.isSettled = (lockupTranched.getDepositedAmount(streamId) - lockupTranched.streamedAmountOf(streamId)) == 0;
        vars.isCancelable = vars.isSettled ? false : params.cancelable;

        // Assert that the stream has been created.
        LockupTranched.StreamLT memory actualStream = lockupTranched.getStream(streamId);
        assertEq(actualStream.amounts, Lockup.Amounts(vars.createAmounts.deposit, 0, 0));
        assertEq(actualStream.asset, dai, "asset");
        assertEq(actualStream.endTime, timestamps.end, "endTime");
        assertEq(actualStream.isCancelable, vars.isCancelable, "isCancelable");
        assertEq(actualStream.isDepleted, false, "isStream");
        assertEq(actualStream.isStream, true, "isStream");
        assertEq(actualStream.isTransferable, true, "isTransferable");
        assertEq(actualStream.recipient, params.recipient, "recipient");
        assertEq(actualStream.sender, params.sender, "sender");
        assertEq(actualStream.tranches, params.tranches, "tranches");
        assertEq(actualStream.startTime, timestamps.start, "startTime");
        assertEq(actualStream.wasCanceled, false, "wasCanceled");

        // Assert that the stream's status is correct.
        vars.actualStatus = lockupTranched.statusOf(streamId);
        if (params.startTime > getBlockTimestamp()) {
            vars.expectedStatus = Lockup.Status.PENDING;
        } else if (vars.isSettled) {
            vars.expectedStatus = Lockup.Status.SETTLED;
        } else {
            vars.expectedStatus = Lockup.Status.STREAMING;
        }
        assertEq(vars.actualStatus, vars.expectedStatus);

        // Assert that the next stream ID has been bumped.
        vars.actualNextStreamId = lockupTranched.nextStreamId();
        vars.expectedNextStreamId = streamId + 1;
        assertEq(vars.actualNextStreamId, vars.expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        vars.actualNFTOwner = lockupTranched.ownerOf({ tokenId: streamId });
        vars.expectedNFTOwner = params.recipient;
        assertEq(vars.actualNFTOwner, vars.expectedNFTOwner, "NFT owner");
    }
}
