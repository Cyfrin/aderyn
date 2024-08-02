// SPDX-License-Identifier: MIT
pragma solidity ^0.5.0;

contract TautologyOrContradiction {
    mapping(uint256 => uint72) map;
    uint x;
    uint256 y;

    function makeUselessComparisons() external view {
        uint8 a = 103;

        // BAD because max value of a is 2^8 - 1
        if (a > 258) {}

        // BAD because min value of uint72 is 0
        if (map[67] < 0) {}
    }
}
