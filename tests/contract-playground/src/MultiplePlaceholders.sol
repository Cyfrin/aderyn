// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract MultiplePlaceholders {
    address internal owner;

    constructor() {
        owner = msg.sender;
    }

    modifier checkOwner() {
        require(msg.sender == owner, "You are not the owner!");
        _;
        _;
    }

    // aderyn-ignore-next-line(empty-block)
    function restrictedFunction1() external checkOwner {}

    // aderyn-ignore-next-line(empty-block)
    function restrictedFunction2() external checkOwner {}
}
