// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

// Copied Heuristic from Slither:
// look for binary expressions with ^ operator where at least one of the operands is a constant, and
// # the constant is not in hex, because hex typically is used with bitwise xor and not exponentiation

contract IncorrectCaretOperator {

    uint256 private s_first;
    uint256 public constant s_second = 190;
    uint256 public constant s_third = 0x9;

    function calc1() external {
        // BAD
        uint256 x = 3^40 - 1;
        uint256 z = s_second^89 + 13;
        uint256 w = s_second^s_first + 13; 
        uint256 y = s_first ^ 100; // s_first is not a constant but, 100 is.
        uint256 p = s_third ^ 20;

        // GOOD
        uint256 r = 0x2 ^ s_first;
        uint256 k = 0x4 ^ 0x10;
        uint256 l = s_third ^ s_first;
    }
}