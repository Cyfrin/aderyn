// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

contract IncorrectShift {
    function shiftBad() internal pure returns (uint shifted) {
        assembly {
            shifted := shr(shifted, 4) // BAD
            shifted := shl(shifted, 4) // BAD
        }

    }

    function shiftGood() internal pure returns (uint shifted) {
        assembly {
            shifted := shr(4, shifted) // GOOD
            shifted := shl(4, shifted) // GOOD
            shifted := shr(8, shifted) // GOOD
            shifted := shl(8, shifted) // GOOD
        }
    }
}