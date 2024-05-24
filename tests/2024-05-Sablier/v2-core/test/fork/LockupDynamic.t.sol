// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Solarray } from "solarray/src/Solarray.sol";

import { Broker, Lockup, LockupDynamic } from "src/types/DataTypes.sol";

import { Fork_Test } from "./Fork.t.sol";

abstract contract LockupDynamic_Fork_Test is Fork_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(IERC20 asset, address holder) Fork_Test(asset, holder) { }

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Fork_Test.setUp();

        // Approve {SablierV2LockupDynamic} to transfer the holder's assets.
        // We use a low-level call to ignore reverts because the asset can have the missing return value bug.
        (bool success,) = address(ASSET).call(abi.encodeCall(IERC20.approve, (address(lockupDynamic), MAX_UINT256)));
        success;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                   TEST FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    struct Params {
        address sender;
        address recipient;
        uint128 withdrawAmount;
        uint40 startTime;
        uint40 warpTimestamp;
        LockupDynamic.Segment[] segments;
        Broker broker;
    }

    struct Vars {
        // Generic vars
        address actualNFTOwner;
        uint256 actualLockupDynamicBalance;
        uint256 actualRecipientBalance;
        Lockup.Status actualStatus;
        uint256[] balances;
        address expectedNFTOwner;
        uint256 expectedLockupDynamicBalance;
        uint256 expectedRecipientBalance;
        Lockup.Status expectedStatus;
        uint256 initialLockupDynamicBalance;
        uint256 initialRecipientBalance;
        bool isCancelable;
        bool isDepleted;
        bool isSettled;
        uint256 streamId;
        LockupDynamic.Timestamps timestamps;
        // Create vars
        uint256 actualBrokerBalance;
        uint256 actualHolderBalance;
        uint256 actualNextStreamId;
        Lockup.CreateAmounts createAmounts;
        uint256 expectedBrokerBalance;
        uint256 expectedHolderBalance;
        uint256 expectedNextStreamId;
        uint256 initialBrokerBalance;
        uint128 totalAmount;
        // Withdraw vars
        uint128 actualWithdrawnAmount;
        uint128 expectedWithdrawnAmount;
        uint128 withdrawableAmount;
        // Cancel vars
        uint256 actualSenderBalance;
        uint256 expectedSenderBalance;
        uint256 initialSenderBalance;
        uint128 recipientAmount;
        uint128 senderAmount;
    }

    /// @dev Checklist:
    ///
    /// - It should perform all expected ERC-20 transfers.
    /// - It should create the stream.
    /// - It should bump the next stream ID.
    /// - It should mint the NFT.
    /// - It should emit a {CreateLockupDynamicStream} event.
    /// - It may make a withdrawal.
    /// - It may update the withdrawn amounts.
    /// - It may emit a {WithdrawFromLockupStream} event.
    /// - It may cancel the stream
    /// - It may emit a {CancelLockupStream} event
    ///
    /// Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - Multiple values for the funder, recipient, sender, and broker
    /// - Multiple values for the total amount
    /// - Start time in the past
    /// - Start time in the present
    /// - Start time in the future
    /// - Start time equal and not equal to the first segment timestamp
    /// - Multiple values for the broker fee, including zero
    /// - Multiple values for the withdraw amount, including zero
    /// - The whole gamut of stream statuses
    function testForkFuzz_LockupDynamic_CreateWithdrawCancel(Params memory params) external {
        checkUsers(params.sender, params.recipient, params.broker.account, address(lockupDynamic));
        vm.assume(params.segments.length != 0);
        params.broker.fee = _bound(params.broker.fee, 0, MAX_BROKER_FEE);
        params.startTime = boundUint40(params.startTime, 1, defaults.START_TIME());

        // Fuzz the segment timestamps.
        fuzzSegmentTimestamps(params.segments, params.startTime);

        // Fuzz the segment amounts and calculate the total and create amounts (deposit and broker fee).
        Vars memory vars;
        (vars.totalAmount, vars.createAmounts) = fuzzDynamicStreamAmounts({
            upperBound: uint128(initialHolderBalance),
            segments: params.segments,
            brokerFee: params.broker.fee
        });

        // Make the holder the caller.
        resetPrank(HOLDER);

        /*//////////////////////////////////////////////////////////////////////////
                                            CREATE
        //////////////////////////////////////////////////////////////////////////*/

        // Load the pre-create asset balances.
        vars.balances =
            getTokenBalances(address(ASSET), Solarray.addresses(address(lockupDynamic), params.broker.account));
        vars.initialLockupDynamicBalance = vars.balances[0];
        vars.initialBrokerBalance = vars.balances[1];

        vars.streamId = lockupDynamic.nextStreamId();
        vars.timestamps = LockupDynamic.Timestamps({
            start: params.startTime,
            end: params.segments[params.segments.length - 1].timestamp
        });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockupDynamic) });
        emit MetadataUpdate({ _tokenId: vars.streamId });
        vm.expectEmit({ emitter: address(lockupDynamic) });
        emit CreateLockupDynamicStream({
            streamId: vars.streamId,
            funder: HOLDER,
            sender: params.sender,
            recipient: params.recipient,
            amounts: vars.createAmounts,
            asset: ASSET,
            cancelable: true,
            transferable: true,
            segments: params.segments,
            timestamps: vars.timestamps,
            broker: params.broker.account
        });

        // Create the stream.
        lockupDynamic.createWithTimestamps(
            LockupDynamic.CreateWithTimestamps({
                sender: params.sender,
                recipient: params.recipient,
                totalAmount: vars.totalAmount,
                asset: ASSET,
                cancelable: true,
                transferable: true,
                startTime: params.startTime,
                segments: params.segments,
                broker: params.broker
            })
        );

        // Check if the stream is settled. It is possible for a Lockup Dynamic stream to settle at the time of creation
        // because some segment amounts can be zero.
        vars.isSettled = lockupDynamic.refundableAmountOf(vars.streamId) == 0;
        vars.isCancelable = vars.isSettled ? false : true;

        // Assert that the stream has been created.
        LockupDynamic.StreamLD memory actualStream = lockupDynamic.getStream(vars.streamId);
        assertEq(actualStream.amounts, Lockup.Amounts(vars.createAmounts.deposit, 0, 0));
        assertEq(actualStream.asset, ASSET, "asset");
        assertEq(actualStream.endTime, vars.timestamps.end, "endTime");
        assertEq(actualStream.isCancelable, vars.isCancelable, "isCancelable");
        assertEq(actualStream.isDepleted, false, "isDepleted");
        assertEq(actualStream.isStream, true, "isStream");
        assertEq(actualStream.isTransferable, true, "isTransferable");
        assertEq(actualStream.recipient, params.recipient, "recipient");
        assertEq(actualStream.segments, params.segments, "segments");
        assertEq(actualStream.sender, params.sender, "sender");
        assertEq(actualStream.startTime, params.startTime, "startTime");
        assertEq(actualStream.wasCanceled, false, "wasCanceled");

        // Assert that the stream's status is correct.
        vars.actualStatus = lockupDynamic.statusOf(vars.streamId);
        if (params.startTime > getBlockTimestamp()) {
            vars.expectedStatus = Lockup.Status.PENDING;
        } else if (vars.isSettled) {
            vars.expectedStatus = Lockup.Status.SETTLED;
        } else {
            vars.expectedStatus = Lockup.Status.STREAMING;
        }
        assertEq(vars.actualStatus, vars.expectedStatus, "post-create stream status");

        // Assert that the next stream ID has been bumped.
        vars.actualNextStreamId = lockupDynamic.nextStreamId();
        vars.expectedNextStreamId = vars.streamId + 1;
        assertEq(vars.actualNextStreamId, vars.expectedNextStreamId, "post-create nextStreamId");

        // Assert that the NFT has been minted.
        vars.actualNFTOwner = lockupDynamic.ownerOf({ tokenId: vars.streamId });
        vars.expectedNFTOwner = params.recipient;
        assertEq(vars.actualNFTOwner, vars.expectedNFTOwner, "post-create NFT owner");

        // Load the post-create asset balances.
        vars.balances =
            getTokenBalances(address(ASSET), Solarray.addresses(address(lockupDynamic), HOLDER, params.broker.account));
        vars.actualLockupDynamicBalance = vars.balances[0];
        vars.actualHolderBalance = vars.balances[1];
        vars.actualBrokerBalance = vars.balances[2];

        // Assert that the contract's balance has been updated.
        vars.expectedLockupDynamicBalance = vars.initialLockupDynamicBalance + vars.createAmounts.deposit;
        assertEq(
            vars.actualLockupDynamicBalance,
            vars.expectedLockupDynamicBalance,
            "post-create LockupDynamic contract balance"
        );

        // Assert that the holder's balance has been updated.
        vars.expectedHolderBalance = initialHolderBalance - vars.totalAmount;
        assertEq(vars.actualHolderBalance, vars.expectedHolderBalance, "post-create Holder balance");

        // Assert that the broker's balance has been updated.
        vars.expectedBrokerBalance = vars.initialBrokerBalance + vars.createAmounts.brokerFee;
        assertEq(vars.actualBrokerBalance, vars.expectedBrokerBalance, "post-create Broker balance");

        /*//////////////////////////////////////////////////////////////////////////
                                          WITHDRAW
        //////////////////////////////////////////////////////////////////////////*/

        // Simulate the passage of time.
        params.warpTimestamp =
            boundUint40(params.warpTimestamp, vars.timestamps.start, vars.timestamps.end + 100 seconds);
        vm.warp({ newTimestamp: params.warpTimestamp });

        // Bound the withdraw amount.
        vars.withdrawableAmount = lockupDynamic.withdrawableAmountOf(vars.streamId);
        params.withdrawAmount = boundUint128(params.withdrawAmount, 0, vars.withdrawableAmount);

        // Check if the stream has settled or will get depleted. It is possible for the stream to be just settled
        // and not depleted because the withdraw amount is fuzzed.
        vars.isDepleted = params.withdrawAmount == vars.createAmounts.deposit;
        vars.isSettled = lockupDynamic.refundableAmountOf(vars.streamId) == 0;

        // Only run the withdraw tests if the withdraw amount is not zero.
        if (params.withdrawAmount > 0) {
            // Load the pre-withdraw asset balances.
            vars.initialLockupDynamicBalance = vars.actualLockupDynamicBalance;
            vars.initialRecipientBalance = ASSET.balanceOf(params.recipient);

            // Expect the relevant events to be emitted.
            vm.expectEmit({ emitter: address(lockupDynamic) });
            emit WithdrawFromLockupStream({
                streamId: vars.streamId,
                to: params.recipient,
                asset: ASSET,
                amount: params.withdrawAmount
            });
            vm.expectEmit({ emitter: address(lockupDynamic) });
            emit MetadataUpdate({ _tokenId: vars.streamId });

            // Make the withdrawal.
            resetPrank({ msgSender: params.recipient });
            lockupDynamic.withdraw({ streamId: vars.streamId, to: params.recipient, amount: params.withdrawAmount });

            // Assert that the stream's status is correct.
            vars.actualStatus = lockupDynamic.statusOf(vars.streamId);
            if (vars.isDepleted) {
                vars.expectedStatus = Lockup.Status.DEPLETED;
            } else if (vars.isSettled) {
                vars.expectedStatus = Lockup.Status.SETTLED;
            } else {
                vars.expectedStatus = Lockup.Status.STREAMING;
            }
            assertEq(vars.actualStatus, vars.expectedStatus, "post-withdraw stream status");

            // Assert that the withdrawn amount has been updated.
            vars.actualWithdrawnAmount = lockupDynamic.getWithdrawnAmount(vars.streamId);
            vars.expectedWithdrawnAmount = params.withdrawAmount;
            assertEq(vars.actualWithdrawnAmount, vars.expectedWithdrawnAmount, "post-withdraw withdrawnAmount");

            // Load the post-withdraw asset balances.
            vars.balances =
                getTokenBalances(address(ASSET), Solarray.addresses(address(lockupDynamic), params.recipient));
            vars.actualLockupDynamicBalance = vars.balances[0];
            vars.actualRecipientBalance = vars.balances[1];

            // Assert that the contract's balance has been updated.
            vars.expectedLockupDynamicBalance = vars.initialLockupDynamicBalance - uint256(params.withdrawAmount);
            assertEq(
                vars.actualLockupDynamicBalance,
                vars.expectedLockupDynamicBalance,
                "post-withdraw lockupDynamic contract balance"
            );

            // Assert that the Recipient's balance has been updated.
            vars.expectedRecipientBalance = vars.initialRecipientBalance + uint256(params.withdrawAmount);
            assertEq(vars.actualRecipientBalance, vars.expectedRecipientBalance, "post-withdraw Recipient balance");
        }

        /*//////////////////////////////////////////////////////////////////////////
                                          CANCEL
        //////////////////////////////////////////////////////////////////////////*/

        // Only run the cancel tests if the stream is neither depleted nor settled.
        if (!vars.isDepleted && !vars.isSettled) {
            // Load the pre-cancel asset balances.
            vars.balances = getTokenBalances(
                address(ASSET), Solarray.addresses(address(lockupDynamic), params.sender, params.recipient)
            );
            vars.initialLockupDynamicBalance = vars.balances[0];
            vars.initialSenderBalance = vars.balances[1];
            vars.initialRecipientBalance = vars.balances[2];

            // Expect the relevant events to be emitted.
            vm.expectEmit({ emitter: address(lockupDynamic) });
            vars.senderAmount = lockupDynamic.refundableAmountOf(vars.streamId);
            vars.recipientAmount = lockupDynamic.withdrawableAmountOf(vars.streamId);
            emit CancelLockupStream(
                vars.streamId, params.sender, params.recipient, ASSET, vars.senderAmount, vars.recipientAmount
            );
            vm.expectEmit({ emitter: address(lockupDynamic) });
            emit MetadataUpdate({ _tokenId: vars.streamId });

            // Cancel the stream.
            resetPrank({ msgSender: params.sender });
            lockupDynamic.cancel(vars.streamId);

            // Assert that the stream's status is correct.
            vars.actualStatus = lockupDynamic.statusOf(vars.streamId);
            vars.expectedStatus = vars.recipientAmount > 0 ? Lockup.Status.CANCELED : Lockup.Status.DEPLETED;
            assertEq(vars.actualStatus, vars.expectedStatus, "post-cancel stream status");

            // Load the post-cancel asset balances.
            vars.balances = getTokenBalances(
                address(ASSET), Solarray.addresses(address(lockupDynamic), params.sender, params.recipient)
            );
            vars.actualLockupDynamicBalance = vars.balances[0];
            vars.actualSenderBalance = vars.balances[1];
            vars.actualRecipientBalance = vars.balances[2];

            // Assert that the contract's balance has been updated.
            vars.expectedLockupDynamicBalance = vars.initialLockupDynamicBalance - uint256(vars.senderAmount);
            assertEq(
                vars.actualLockupDynamicBalance,
                vars.expectedLockupDynamicBalance,
                "post-cancel lockupDynamic contract balance"
            );

            // Assert that the Sender's balance has been updated.
            vars.expectedSenderBalance = vars.initialSenderBalance + uint256(vars.senderAmount);
            assertEq(vars.actualSenderBalance, vars.expectedSenderBalance, "post-cancel Sender balance");

            // Assert that the Recipient's balance has not changed.
            vars.expectedRecipientBalance = vars.initialRecipientBalance;
            assertEq(vars.actualRecipientBalance, vars.expectedRecipientBalance, "post-cancel Recipient balance");
        }

        // Assert that the NFT has not been burned.
        vars.actualNFTOwner = lockupDynamic.ownerOf({ tokenId: vars.streamId });
        vars.expectedNFTOwner = params.recipient;
        assertEq(vars.actualNFTOwner, vars.expectedNFTOwner, "post-cancel NFT owner");
    }
}
