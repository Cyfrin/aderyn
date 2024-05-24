// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { ISablierV2Lockup } from "../../../src/interfaces/ISablierV2Lockup.sol";
import { ISablierV2Sender } from "../../../src/interfaces/hooks/ISablierV2Sender.sol";

contract ReentrantSender is ISablierV2Sender {
    function onLockupStreamWithdrawn(uint256 streamId, address caller, address to, uint128 amount) external {
        streamId;
        caller;
        to;
        amount;
        ISablierV2Lockup(msg.sender).withdraw(streamId, address(this), amount);
    }
}
