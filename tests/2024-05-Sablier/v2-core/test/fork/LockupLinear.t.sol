// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { ud } from "@prb/math/src/UD60x18.sol";
import { Solarray } from "solarray/src/Solarray.sol";

import { Broker, Lockup, LockupLinear } from "src/types/DataTypes.sol";

import { Fork_Test } from "./Fork.t.sol";

abstract contract LockupLinear_Fork_Test is Fork_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(IERC20 asset, address holder) Fork_Test(asset, holder) { }

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Fork_Test.setUp();

        // Approve {SablierV2LockupLinear} to transfer the asset holder's assets.
        // We use a low-level call to ignore reverts because the asset can have the missing return value bug.
        (bool success,) = address(ASSET).call(abi.encodeCall(IERC20.approve, (address(lockupLinear), MAX_UINT256)));
        success;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                   TEST FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    struct Params {
        address sender;
        address recipient;
        uint128 totalAmount;
        uint128 withdrawAmount;
        uint40 warpTimestamp;
        LockupLinear.Timestamps timestamps;
        Broker broker;
    }

    struct Vars {
        // Generic vars
        uint256 actualLockupLinearBalance;
        uint256 actualHolderBalance;
        address actualNFTOwner;
        uint256 actualRecipientBalance;
        Lockup.Status actualStatus;
        uint256[] balances;
        uint40 blockTimestamp;
        uint40 endTimeLowerBound;
        uint256 expectedLockupLinearBalance;
        uint256 expectedHolderBalance;
        address expectedNFTOwner;
        uint256 expectedRecipientBalance;
        Lockup.Status expectedStatus;
        bool hasCliff;
        uint256 initialLockupLinearBalance;
        uint256 initialRecipientBalance;
        bool isDepleted;
        bool isSettled;
        uint256 streamId;
        // Create vars
        uint256 actualBrokerBalance;
        uint256 actualNextStreamId;
        Lockup.CreateAmounts createAmounts;
        uint256 expectedBrokerBalance;
        uint256 expectedNextStreamId;
        uint256 initialBrokerBalance;
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
    /// - It should emit a {MetadataUpdate} event
    /// - It should emit a {CreateLockupLinearStream} event.
    /// - It may make a withdrawal.
    /// - It may update the withdrawn amounts.
    /// - It may emit a {WithdrawFromLockupStream} event.
    /// - It may cancel the stream
    /// - It may emit a {CancelLockupStream} event
    ///
    /// Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - Multiple values for the sender, recipient, and broker
    /// - Multiple values for the total amount
    /// - Multiple values for the withdraw amount, including zero
    /// - Start time in the past
    /// - Start time in the present
    /// - Start time in the future
    /// - Multiple values for the cliff time and the end time
    /// - Cliff time zero and not zero
    /// - Multiple values for the broker fee, including zero
    /// - The whole gamut of stream statuses
    function testForkFuzz_LockupLinear_CreateWithdrawCancel(Params memory params) external {
        checkUsers(params.sender, params.recipient, params.broker.account, address(lockupLinear));

        // Bound the parameters.
        Vars memory vars;
        vars.blockTimestamp = getBlockTimestamp();
        params.broker.fee = _bound(params.broker.fee, 0, MAX_BROKER_FEE);
        params.timestamps.start = boundUint40(
            params.timestamps.start, vars.blockTimestamp - 1000 seconds, vars.blockTimestamp + 10_000 seconds
        );
        params.totalAmount = boundUint128(params.totalAmount, 1, uint128(initialHolderBalance));

        // The cliff time must be either zero or greater than the start time.
        vars.hasCliff = params.timestamps.cliff > 0;
        if (vars.hasCliff) {
            params.timestamps.cliff = boundUint40(
                params.timestamps.cliff, params.timestamps.start + 1 seconds, params.timestamps.start + 52 weeks
            );
        }
        // Bound the end time so that it is always greater than the block timestamp, the start time, and the cliff time.
        vars.endTimeLowerBound = maxOfThree(params.timestamps.start, params.timestamps.cliff, vars.blockTimestamp);
        params.timestamps.end =
            boundUint40(params.timestamps.end, vars.endTimeLowerBound + 1 seconds, MAX_UNIX_TIMESTAMP);

        // Make the holder the caller.
        resetPrank(HOLDER);

        /*//////////////////////////////////////////////////////////////////////////
                                            CREATE
        //////////////////////////////////////////////////////////////////////////*/

        // Load the pre-create asset balances.
        vars.balances =
            getTokenBalances(address(ASSET), Solarray.addresses(address(lockupLinear), params.broker.account));
        vars.initialLockupLinearBalance = vars.balances[0];
        vars.initialBrokerBalance = vars.balances[1];

        // Calculate the broker fee amount and the deposit amount.
        vars.createAmounts.brokerFee = ud(params.totalAmount).mul(params.broker.fee).intoUint128();
        vars.createAmounts.deposit = params.totalAmount - vars.createAmounts.brokerFee;

        vars.streamId = lockupLinear.nextStreamId();

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockupLinear) });
        emit MetadataUpdate({ _tokenId: vars.streamId });
        vm.expectEmit({ emitter: address(lockupLinear) });
        emit CreateLockupLinearStream({
            streamId: vars.streamId,
            funder: HOLDER,
            sender: params.sender,
            recipient: params.recipient,
            amounts: vars.createAmounts,
            asset: ASSET,
            cancelable: true,
            transferable: true,
            timestamps: params.timestamps,
            broker: params.broker.account
        });

        // Create the stream.
        lockupLinear.createWithTimestamps(
            LockupLinear.CreateWithTimestamps({
                sender: params.sender,
                recipient: params.recipient,
                totalAmount: params.totalAmount,
                asset: ASSET,
                cancelable: true,
                transferable: true,
                timestamps: params.timestamps,
                broker: params.broker
            })
        );

        // Assert that the stream has been created.
        LockupLinear.StreamLL memory actualStream = lockupLinear.getStream(vars.streamId);
        assertEq(actualStream.amounts, Lockup.Amounts(vars.createAmounts.deposit, 0, 0));
        assertEq(actualStream.asset, ASSET, "asset");
        assertEq(actualStream.cliffTime, params.timestamps.cliff, "cliffTime");
        assertEq(actualStream.endTime, params.timestamps.end, "endTime");
        assertEq(actualStream.isCancelable, true, "isCancelable");
        assertEq(actualStream.isDepleted, false, "isDepleted");
        assertEq(actualStream.isStream, true, "isStream");
        assertEq(actualStream.isTransferable, true, "isTransferable");
        assertEq(actualStream.recipient, params.recipient, "recipient");
        assertEq(actualStream.sender, params.sender, "sender");
        assertEq(actualStream.startTime, params.timestamps.start, "startTime");
        assertEq(actualStream.wasCanceled, false, "wasCanceled");

        // Assert that the stream's status is correct.
        vars.actualStatus = lockupLinear.statusOf(vars.streamId);
        vars.expectedStatus =
            params.timestamps.start > vars.blockTimestamp ? Lockup.Status.PENDING : Lockup.Status.STREAMING;
        assertEq(vars.actualStatus, vars.expectedStatus, "post-create stream status");

        // Assert that the next stream ID has been bumped.
        vars.actualNextStreamId = lockupLinear.nextStreamId();
        vars.expectedNextStreamId = vars.streamId + 1;
        assertEq(vars.actualNextStreamId, vars.expectedNextStreamId, "post-create nextStreamId");

        // Assert that the NFT has been minted.
        vars.actualNFTOwner = lockupLinear.ownerOf({ tokenId: vars.streamId });
        vars.expectedNFTOwner = params.recipient;
        assertEq(vars.actualNFTOwner, vars.expectedNFTOwner, "post-create NFT owner");

        // Load the post-create asset balances.
        vars.balances =
            getTokenBalances(address(ASSET), Solarray.addresses(address(lockupLinear), HOLDER, params.broker.account));
        vars.actualLockupLinearBalance = vars.balances[0];
        vars.actualHolderBalance = vars.balances[1];
        vars.actualBrokerBalance = vars.balances[2];

        // Assert that the LockupLinear contract's balance has been updated.
        vars.expectedLockupLinearBalance = vars.initialLockupLinearBalance + vars.createAmounts.deposit;
        assertEq(vars.actualLockupLinearBalance, vars.expectedLockupLinearBalance, "post-create LockupLinear balance");

        // Assert that the holder's balance has been updated.
        vars.expectedHolderBalance = initialHolderBalance - params.totalAmount;
        assertEq(vars.actualHolderBalance, vars.expectedHolderBalance, "post-create Holder balance");

        // Assert that the broker's balance has been updated.
        vars.expectedBrokerBalance = vars.initialBrokerBalance + vars.createAmounts.brokerFee;
        assertEq(vars.actualBrokerBalance, vars.expectedBrokerBalance, "post-create Broker balance");

        /*//////////////////////////////////////////////////////////////////////////
                                          WITHDRAW
        //////////////////////////////////////////////////////////////////////////*/

        // Simulate the passage of time.
        params.warpTimestamp = boundUint40(
            params.warpTimestamp,
            vars.hasCliff ? params.timestamps.cliff : params.timestamps.start + 1 seconds,
            params.timestamps.end + 100 seconds
        );
        vm.warp({ newTimestamp: params.warpTimestamp });

        // Bound the withdraw amount.
        vars.withdrawableAmount = lockupLinear.withdrawableAmountOf(vars.streamId);
        params.withdrawAmount = boundUint128(params.withdrawAmount, 0, vars.withdrawableAmount);

        // Check if the stream has settled or will get depleted. It is possible for the stream to be just settled
        // and not depleted because the withdraw amount is fuzzed.
        vars.isSettled = lockupLinear.refundableAmountOf(vars.streamId) == 0;
        vars.isDepleted = params.withdrawAmount == vars.createAmounts.deposit;

        // Only run the withdraw tests if the withdraw amount is not zero.
        if (params.withdrawAmount > 0) {
            // Load the pre-withdraw asset balances.
            vars.initialLockupLinearBalance = vars.actualLockupLinearBalance;
            vars.initialRecipientBalance = ASSET.balanceOf(params.recipient);

            // Expect the relevant events to be emitted.
            vm.expectEmit({ emitter: address(lockupLinear) });
            emit WithdrawFromLockupStream({
                streamId: vars.streamId,
                to: params.recipient,
                asset: ASSET,
                amount: params.withdrawAmount
            });
            vm.expectEmit({ emitter: address(lockupLinear) });
            emit MetadataUpdate({ _tokenId: vars.streamId });

            // Make the withdrawal.
            resetPrank({ msgSender: params.recipient });
            lockupLinear.withdraw({ streamId: vars.streamId, to: params.recipient, amount: params.withdrawAmount });

            // Assert that the stream's status is correct.
            vars.actualStatus = lockupLinear.statusOf(vars.streamId);
            if (vars.isDepleted) {
                vars.expectedStatus = Lockup.Status.DEPLETED;
            } else if (vars.isSettled) {
                vars.expectedStatus = Lockup.Status.SETTLED;
            } else {
                vars.expectedStatus = Lockup.Status.STREAMING;
            }
            assertEq(vars.actualStatus, vars.expectedStatus, "post-withdraw stream status");

            // Assert that the withdrawn amount has been updated.
            vars.actualWithdrawnAmount = lockupLinear.getWithdrawnAmount(vars.streamId);
            vars.expectedWithdrawnAmount = params.withdrawAmount;
            assertEq(vars.actualWithdrawnAmount, vars.expectedWithdrawnAmount, "post-withdraw withdrawnAmount");

            // Load the post-withdraw asset balances.
            vars.balances =
                getTokenBalances(address(ASSET), Solarray.addresses(address(lockupLinear), params.recipient));
            vars.actualLockupLinearBalance = vars.balances[0];
            vars.actualRecipientBalance = vars.balances[1];

            // Assert that the contract's balance has been updated.
            vars.expectedLockupLinearBalance = vars.initialLockupLinearBalance - uint256(params.withdrawAmount);
            assertEq(
                vars.actualLockupLinearBalance, vars.expectedLockupLinearBalance, "post-withdraw LockupLinear balance"
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
                address(ASSET), Solarray.addresses(address(lockupLinear), params.sender, params.recipient)
            );
            vars.initialLockupLinearBalance = vars.balances[0];
            vars.initialSenderBalance = vars.balances[1];
            vars.initialRecipientBalance = vars.balances[2];

            // Expect the relevant events to be emitted.
            vm.expectEmit({ emitter: address(lockupLinear) });
            vars.senderAmount = lockupLinear.refundableAmountOf(vars.streamId);
            vars.recipientAmount = lockupLinear.withdrawableAmountOf(vars.streamId);
            emit CancelLockupStream(
                vars.streamId, params.sender, params.recipient, ASSET, vars.senderAmount, vars.recipientAmount
            );
            vm.expectEmit({ emitter: address(lockupLinear) });
            emit MetadataUpdate({ _tokenId: vars.streamId });

            // Cancel the stream.
            resetPrank({ msgSender: params.sender });
            lockupLinear.cancel(vars.streamId);

            // Assert that the stream's status is correct.
            vars.actualStatus = lockupLinear.statusOf(vars.streamId);
            vars.expectedStatus = vars.recipientAmount > 0 ? Lockup.Status.CANCELED : Lockup.Status.DEPLETED;
            assertEq(vars.actualStatus, vars.expectedStatus, "post-cancel stream status");

            // Load the post-cancel asset balances.
            vars.balances = getTokenBalances(
                address(ASSET), Solarray.addresses(address(lockupLinear), params.sender, params.recipient)
            );
            vars.actualLockupLinearBalance = vars.balances[0];
            vars.actualSenderBalance = vars.balances[1];
            vars.actualRecipientBalance = vars.balances[2];

            // Assert that the contract's balance has been updated.
            vars.expectedLockupLinearBalance = vars.initialLockupLinearBalance - uint256(vars.senderAmount);
            assertEq(
                vars.actualLockupLinearBalance, vars.expectedLockupLinearBalance, "post-cancel LockupLinear balance"
            );

            // Assert that the Sender's balance has been updated.
            vars.expectedSenderBalance = vars.initialSenderBalance + uint256(vars.senderAmount);
            assertEq(vars.actualSenderBalance, vars.expectedSenderBalance, "post-cancel Sender balance");

            // Assert that the Recipient's balance has not changed.
            vars.expectedRecipientBalance = vars.initialRecipientBalance;
            assertEq(vars.actualRecipientBalance, vars.expectedRecipientBalance, "post-cancel Recipient balance");
        }

        // Assert that the NFT has not been burned.
        vars.actualNFTOwner = lockupLinear.ownerOf({ tokenId: vars.streamId });
        vars.expectedNFTOwner = params.recipient;
        assertEq(vars.actualNFTOwner, vars.expectedNFTOwner, "post-cancel NFT owner");
    }
}
