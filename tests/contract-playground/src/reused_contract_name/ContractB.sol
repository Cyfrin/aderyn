// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

contract ReusedName {
    address public x;

    constructor() {
        x = msg.sender;
    }

}
