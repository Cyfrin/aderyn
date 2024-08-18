// SPDX-License-Identifier: MIT
pragma solidity ^0.4.0;

contract BuiltinSymbolShadow {
    uint now; // BAD

    // BAD
    function assert(bool condition) public {}

    function get_next_expiration(
        uint earlier_time
    ) private blockhash returns (uint) {
        return now + 259200; // References overshadowed timestamp.
    }

    // BAD
    modifier blockhash() {
        _;
    }

    // BAD
    event sha256();
}
