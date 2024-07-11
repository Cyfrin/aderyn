// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {EnumerableSet} from "../lib/openzeppelin-contracts/contracts/utils/structs/EnumerableSet.sol";

contract EnumerableSetIteration {
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.UintSet;

    EnumerableSet.Bytes32Set private bytes32Set;
    EnumerableSet.AddressSet private addressSet;
    EnumerableSet.UintSet private uintSet;

    constructor(bytes32[] memory bytes32s, address[] memory addresses, uint256[] memory uints) {
        for (uint256 i = 0; i < bytes32s.length; i++) {
            bytes32Set.add(bytes32s[i]);
        }
        for (uint256 i = 0; i < addresses.length; i++) {
            addressSet.add(addresses[i]);
        }
        for (uint256 i = 0; i < uints.length; i++) {
            uintSet.add(uints[i]);
        }
    }

    // Bad
    function badBytes32IterationInForLoop() external {
        for (uint256 i = 0; i < bytes32Set.length(); i++) {
            bytes32 thisBytes32 = bytes32Set.at(i);
            bytes32Set.remove(thisBytes32);
        }
    }

    // Bad
    function badBytes32IterationInWhileLoop() external {
        uint256 i = 0;
        while(i < bytes32Set.length()) {
            bytes32 thisBytes32 = bytes32Set.at(i);
            bytes32Set.remove(thisBytes32);
            i++;
        }
    }

    // Bad
    function badBytes32IterationInDoWhileLoop() external {
        uint256 i = 0;
        do {
            bytes32 thisBytes32 = bytes32Set.at(i);
            bytes32Set.remove(thisBytes32);
            i++;
        } while(i < bytes32Set.length());
    }

    // Bad
    function badAddressIteration() external {
        for (uint256 i = 0; i < addressSet.length(); i++) {
            address thisAddress = addressSet.at(i);
            addressSet.remove(thisAddress);
        }
    }

    // Bad
    function badUintIteration() external {
        for (uint256 i = 0; i < uintSet.length(); i++) {
            uint256 thisUint = uintSet.at(i);
            uintSet.remove(thisUint);
        }
    }
}