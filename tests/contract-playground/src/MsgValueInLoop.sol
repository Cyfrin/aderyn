// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// COPIED from Slither Wiki
contract MsgValueInLoop1 {
    mapping(address => uint256) balances;

    function bad(address[] memory receivers) public payable {
        // BAD for loop
        for (uint256 i = 0; i < receivers.length; i++) {
            balances[receivers[i]] += msg.value;
        }
    }
}

contract MsgValueInLoop2 {
    mapping(address => uint256) balances;

    function bad(address[] memory receivers) public payable {
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

    function bad(address[] memory receivers) public payable {
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

    function bad(address[] memory receivers) public payable {
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
