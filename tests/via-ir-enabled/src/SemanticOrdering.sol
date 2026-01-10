// SPDX-License-Identifier: GPL-3.0

/*
    https://docs.soliditylang.org/en/latest/ir-breaking-changes.html#semantic-only-changes
*/

pragma solidity ^0.8.0;

abstract contract A {
    uint x;
    constructor() {
        x = 42;
    }
    function f() public view returns(uint256) {
        return x;
    }
}

contract B is A {
    uint public y = f();
}
