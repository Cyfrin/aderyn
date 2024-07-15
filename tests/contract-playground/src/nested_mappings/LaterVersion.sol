// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

contract TestNestedStructInMappingLaterVersion {

    // The struct that is nested.
    struct structNested {
        uint dummy;
    }

    // The struct that holds the nested struct.
    struct structMain {
        structNested gamePaymentsSummary;
    }

    // The map that maps a game ID to a specific game.
    mapping(uint256 => structMain) public s_mapOfNestedStructs;
}