// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { ISablierV2Lockup } from "../../../src/interfaces/ISablierV2Lockup.sol";
import { ISablierV2Recipient } from "../../../src/interfaces/hooks/ISablierV2Recipient.sol";

contract ReentrantRecipient is ISablierV2Recipient {
    function onLockupStreamCanceled(
        uint256 streamId,
        address sender,
        uint128 senderAmount,
        uint128 recipientAmount
    )
        external
    {
        streamId;
        senderAmount;
        sender;
        recipientAmount;
        ISablierV2Lockup(msg.sender).cancel(streamId);
    }

    function onLockupStreamRenounced(uint256 streamId) external {
        ISablierV2Lockup(msg.sender).renounce(streamId);
    }

    function onLockupStreamWithdrawn(uint256 streamId, address caller, address to, uint128 amount) external {
        streamId;
        caller;
        to;
        amount;
        ISablierV2Lockup(msg.sender).withdraw(streamId, address(this), amount);
    }
}
