// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";
import { Lockup } from "src/types/DataTypes.sol";

import { LockupStore } from "../stores/LockupStore.sol";
import { TimestampStore } from "../stores/TimestampStore.sol";
import { BaseHandler } from "./BaseHandler.sol";

/// @dev Common handler logic between {LockupLinearHandler} and {LockupDynamicHandler}.
abstract contract LockupHandler is BaseHandler {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    ISablierV2Lockup public lockup;
    LockupStore public lockupStore;

    /*//////////////////////////////////////////////////////////////////////////
                                     VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    address internal currentRecipient;
    address internal currentSender;
    uint256 internal currentStreamId;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(
        IERC20 asset_,
        TimestampStore timestampStore_,
        LockupStore lockupStore_,
        ISablierV2Lockup lockup_
    )
        BaseHandler(asset_, timestampStore_)
    {
        lockupStore = lockupStore_;
        lockup = lockup_;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     MODIFIERS
    //////////////////////////////////////////////////////////////////////////*/

    modifier useAdmin() {
        address admin = lockup.admin();
        resetPrank(admin);
        _;
    }

    /// @dev Picks a random stream from the store.
    /// @param streamIndexSeed A fuzzed value needed for picking the random stream.
    modifier useFuzzedStream(uint256 streamIndexSeed) {
        uint256 lastStreamId = lockupStore.lastStreamId();
        if (lastStreamId == 0) {
            return;
        }
        uint256 fuzzedStreamId = _bound(streamIndexSeed, 0, lastStreamId - 1);
        currentStreamId = lockupStore.streamIds(fuzzedStreamId);
        _;
    }

    modifier useFuzzedStreamRecipient() {
        uint256 lastStreamId = lockupStore.lastStreamId();
        currentRecipient = lockupStore.recipients(currentStreamId);
        resetPrank(currentRecipient);
        _;
    }

    modifier useFuzzedStreamSender() {
        uint256 lastStreamId = lockupStore.lastStreamId();
        currentSender = lockupStore.senders(currentStreamId);
        resetPrank(currentSender);
        _;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                 SABLIER-V2-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    function burn(
        uint256 timeJumpSeed,
        uint256 streamIndexSeed
    )
        external
        instrument("burn")
        adjustTimestamp(timeJumpSeed)
        useFuzzedStream(streamIndexSeed)
        useFuzzedStreamRecipient
    {
        // Only depleted streams can be burned.
        if (lockup.statusOf(currentStreamId) != Lockup.Status.DEPLETED) {
            return;
        }

        // Only NFTs that still exist can be burned.
        if (currentRecipient == address(0)) {
            return;
        }

        // Burn the NFT.
        lockup.burn(currentStreamId);

        // Set the recipient associated with this stream to the zero address.
        lockupStore.updateRecipient(currentStreamId, address(0));
    }

    function cancel(
        uint256 timeJumpSeed,
        uint256 streamIndexSeed
    )
        external
        instrument("cancel")
        adjustTimestamp(timeJumpSeed)
        useFuzzedStream(streamIndexSeed)
        useFuzzedStreamSender
    {
        // Cold streams cannot be withdrawn from.
        if (lockup.isCold(currentStreamId)) {
            return;
        }

        // Not cancelable streams cannot be canceled.
        bool isCancelable = lockup.isCancelable(currentStreamId);
        if (!isCancelable) {
            return;
        }

        // Cancel the stream.
        lockup.cancel(currentStreamId);
    }

    function renounce(
        uint256 timeJumpSeed,
        uint256 streamIndexSeed
    )
        external
        instrument("renounce")
        adjustTimestamp(timeJumpSeed)
        useFuzzedStream(streamIndexSeed)
        useFuzzedStreamSender
    {
        // Cold streams cannot be renounced.
        if (lockup.isCold(currentStreamId)) {
            return;
        }

        // Not cancelable streams cannot be renounced.
        bool isCancelable = lockup.isCancelable(currentStreamId);
        if (!isCancelable) {
            return;
        }

        // Renounce the stream (make it not cancelable).
        lockup.renounce(currentStreamId);
    }

    function withdraw(
        uint256 timeJumpSeed,
        uint256 streamIndexSeed,
        address to,
        uint128 withdrawAmount
    )
        external
        instrument("withdraw")
        adjustTimestamp(timeJumpSeed)
        useFuzzedStream(streamIndexSeed)
        useFuzzedStreamRecipient
    {
        // Pending and depleted streams cannot be withdrawn from.
        Lockup.Status status = lockup.statusOf(currentStreamId);
        if (status == Lockup.Status.PENDING || status == Lockup.Status.DEPLETED) {
            return;
        }

        // The protocol doesn't allow the withdrawal address to be the zero address.
        if (to == address(0)) {
            return;
        }

        // The protocol doesn't allow zero withdrawal amounts.
        uint128 withdrawableAmount = lockup.withdrawableAmountOf(currentStreamId);
        if (withdrawableAmount == 0) {
            return;
        }

        // Bound the withdraw amount so that it is not zero.
        withdrawAmount = boundUint128(withdrawAmount, 1, withdrawableAmount);

        // There is an edge case when the sender is the same as the recipient. In this scenario, the withdrawal
        // address must be set to the recipient.
        address sender = lockupStore.senders(currentStreamId);
        if (sender == currentRecipient && to != currentRecipient) {
            to = currentRecipient;
        }

        // Withdraw from the stream.
        lockup.withdraw({ streamId: currentStreamId, to: to, amount: withdrawAmount });
    }

    function withdrawMax(
        uint256 timeJumpSeed,
        uint256 streamIndexSeed,
        address to
    )
        external
        instrument("withdrawMax")
        adjustTimestamp(timeJumpSeed)
        useFuzzedStream(streamIndexSeed)
        useFuzzedStreamRecipient
    {
        // Pending and depleted streams cannot be withdrawn from.
        Lockup.Status status = lockup.statusOf(currentStreamId);
        if (status == Lockup.Status.PENDING || status == Lockup.Status.DEPLETED) {
            return;
        }

        // The protocol doesn't allow the withdrawal address to be the zero address.
        if (to == address(0)) {
            return;
        }

        // The protocol doesn't allow a zero amount to be withdrawn.
        uint128 withdrawableAmount = lockup.withdrawableAmountOf(currentStreamId);
        if (withdrawableAmount == 0) {
            return;
        }

        // There is an edge case when the sender is the same as the recipient. In this scenario, the withdrawal
        // address must be set to the recipient.
        address sender = lockupStore.senders(currentStreamId);
        if (sender == currentRecipient && to != currentRecipient) {
            to = currentRecipient;
        }

        // Make the max withdrawal.
        lockup.withdrawMax({ streamId: currentStreamId, to: to });
    }

    function withdrawMaxAndTransfer(
        uint256 timeJumpSeed,
        uint256 streamIndexSeed,
        address newRecipient
    )
        external
        instrument("withdrawMaxAndTransfer")
        adjustTimestamp(timeJumpSeed)
        useFuzzedStream(streamIndexSeed)
        useFuzzedStreamRecipient
    {
        // Pending and depleted streams cannot be withdrawn from.
        Lockup.Status status = lockup.statusOf(currentStreamId);
        if (status == Lockup.Status.PENDING || status == Lockup.Status.DEPLETED) {
            return;
        }

        // OpenZeppelin's ERC-721 implementation doesn't allow the new recipient to be the zero address.
        if (newRecipient == address(0)) {
            return;
        }

        // Skip burned NFTs.
        if (currentRecipient == address(0)) {
            return;
        }

        // Skip if the stream is not transferable.
        if (!lockup.isTransferable(currentStreamId)) {
            return;
        }

        // The protocol doesn't allow a zero amount to be withdrawn.
        uint128 withdrawableAmount = lockup.withdrawableAmountOf(currentStreamId);
        if (withdrawableAmount == 0) {
            return;
        }

        // Make the max withdrawal and transfer the NFT.
        lockup.withdrawMaxAndTransfer({ streamId: currentStreamId, newRecipient: newRecipient });

        // Update the recipient associated with this stream ID.
        lockupStore.updateRecipient(currentStreamId, newRecipient);
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      ERC-721
    //////////////////////////////////////////////////////////////////////////*/

    function transferNFT(
        uint256 timeJumpSeed,
        uint256 streamIndexSeed,
        address newRecipient
    )
        external
        instrument("transferNFT")
        adjustTimestamp(timeJumpSeed)
        useFuzzedStream(streamIndexSeed)
        useFuzzedStreamRecipient
    {
        // OpenZeppelin's ERC-721 implementation doesn't allow the new recipient to be the zero address.
        if (newRecipient == address(0)) {
            return;
        }

        // Skip burned NFTs.
        if (currentRecipient == address(0)) {
            return;
        }

        // Skip if the stream is not transferable.
        if (!lockup.isTransferable(currentStreamId)) {
            return;
        }

        // Transfer the NFT to the new recipient.
        lockup.transferFrom({ from: currentRecipient, to: newRecipient, tokenId: currentStreamId });

        // Update the recipient associated with this stream ID.
        lockupStore.updateRecipient(currentStreamId, newRecipient);
    }
}
