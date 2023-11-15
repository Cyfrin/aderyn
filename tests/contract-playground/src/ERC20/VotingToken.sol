// SPDX-License-Identifier: MIT

pragma solidity 0.8.20;

import {ERC20Votes, ERC20} from "../../lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Votes.sol";

abstract contract VotingToken is ERC20Votes  {
    constructor() ERC20("Voting Token", "VOTE") {
        _mint(msg.sender, 1000000000000000000000000000);
    }
}