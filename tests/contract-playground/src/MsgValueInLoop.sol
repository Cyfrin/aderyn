// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// COPIED from Slither Wiki

// BAD 
contract MsgValueInLoop1 {
    mapping(address => uint256) balances;

    function bad(address[] memory receivers) external payable {
        // BAD for loop (uses msg.value inside loop)
        for (uint256 i = 0; i < receivers.length; i++) {
            balances[receivers[i]] += msg.value;
        }
    }
}

// GOOD
contract MsgValueOutsideLoop {
    mapping(address => uint256) balances;

    function good(address[] memory receivers) external payable {
        // GOOD for loop (does not use msg.value inside loop)
        uint256 total = msg.value;
        for (uint256 i = 0; i < receivers.length; i++) {
            balances[receivers[i]] += total / receivers.length;
        }
    }
}

///// MORE BAD EXAMPLES //////

contract MsgValueInLoop2 {
    mapping(address => uint256) balances;

    function bad(address[] memory receivers) external payable {
        // BAD for loop
        for (uint256 i = 0; i < receivers.length; i++) {
            addToBal(receivers, i);
        }
    }

    function addToBal(address[] memory receivers, uint256 index) internal {
        balances[receivers[index]] += msg.value;
    }
}

contract MsgValueInLoop3 {
    mapping(address => uint256) balances;

    function bad(address[] memory receivers) external payable {
        // BAD while loop
        uint256 i = 0;
        while (i < receivers.length) {
            addToBal(receivers, i);
            i++;
        }
    }

    function addToBal(address[] memory receivers, uint256 index) internal {
        balances[receivers[index]] += msg.value;
    }
}

contract MsgValueInLoop4 {
    mapping(address => uint256) balances;

    function bad(address[] memory receivers) external payable {
        // BAD do while loop
        uint256 i = 0;
        do {
            addToBal(receivers, i);
            i++;
        } while (i < receivers.length);
    }

    function addToBal(address[] memory receivers, uint256 index) internal {
        balances[receivers[index]] += msg.value;
    }
}
