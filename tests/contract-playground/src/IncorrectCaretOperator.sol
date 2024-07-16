// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

contract IncorrectCaretOperator {

    uint256 private s_first;
    uint256 public constant s_second = 190;
    uint256 public constant s_third = 0x9;

    function calc1() public {
        // BAD
        uint256 x = 3^40 - 1;
        uint256 z = s_second^89 + 13;
        uint256 w = s_second^s_first + 13; 
        uint256 y = s_first ^ 100; // s_first is not a constant but, 100 is.
        uint256 p = s_third ^ 20;

        // GOOD
        uint256 r = 0x2 ^ s_first;
        uint256 k = 0x4 ^ 0x10;
        uint256 p = s_third ^ s_first;
    }
}