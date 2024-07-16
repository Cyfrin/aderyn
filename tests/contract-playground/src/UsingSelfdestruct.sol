// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract SelfdestructContract {

    function dangerous(address sink) external {
        selfdestruct(payable(sink));
    }
}