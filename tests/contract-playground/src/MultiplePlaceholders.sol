// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract MultiplePlaceholders {
    address internal owner;

    constructor() {
        owner = msg.sender;
    }

    // BAD
    modifier checkOwner() {
        require(msg.sender == owner, "You are not the owner!");
        _;
        _;
    }

    // BAD
    modifier test2(uint256 a, uint256 b) {
        if (a == b) {
            _;
        } else if (a + b < 100) {
            revert();
        }
        _;
    }

    // GOOD
    modifier test3(uint256 a, uint256 b) {
        if (a == b) {
            _;
            return;
        } else if (a + b < 100) {
            revert();
        }
        _;
    }

    // BAD
    modifier test4(uint256 a, uint256 b) {
        _;
        if (a == b) {
            _;
            return;
        } else if (a + b < 100) {
            revert();
        }
    }

    // aderyn-ignore-next-line(empty-block)
    function restrictedFunction1() external checkOwner {}

    // aderyn-ignore-next-line(empty-block)
    function restrictedFunction2() external checkOwner {}
}
