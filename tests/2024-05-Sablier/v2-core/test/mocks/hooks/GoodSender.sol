// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { ISablierV2Sender } from "../../../src/interfaces/hooks/ISablierV2Sender.sol";

contract GoodSender is ISablierV2Sender {
    function onLockupStreamWithdrawn(uint256 streamId, address caller, address to, uint128 amount) external pure {
        streamId;
        caller;
        to;
        amount;
    }
}
