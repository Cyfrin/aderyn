// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract AllDataTypes {
    uint256 stateVarUint;

    function testAllDataTypesBAD() public pure {
        // Uninitialized variables (BAD)
        uint uninitializedUint;
        bool uninitializedBool;
        address uninitializedAddress;
        int uninitializedInt;
        bytes32 uninitializedBytes32;
        string memory uninitializedString;
        uint[1] memory uninitializedUintArray;
        bool[1] memory uninitializedBoolArray;
        address[1] memory uninitializedAddressArray;
        int[1] memory uninitializedIntArray;
        bytes32[1] memory uninitializedBytes32Array;
        string[1] memory uninitializedStringArray;
    }

    function testAllDataTypesGOOD() public pure {
        // Initialized variables (GOOD)
        uint initializedUint = 1;
        bool initializedBool = true;
        address initializedAddress = 0x0000000000000000000000000000000000000000;
        int initializedInt = -1;
        bytes32 initializedBytes32 = "hello";
        string memory initializedString = "world";
        uint[1] memory initializedUintArray = [uint(2)];
        bool[1] memory initializedBoolArray = [false];
        address[1] memory initializedAddressArray = [
            0x0000000000000000000000000000000000000001
        ];
        int[1] memory initializedIntArray = [int(-2)];
        bytes32[1] memory initializedBytes32Array = [bytes32("bye")];
        string[1] memory initializedStringArray = ["Solidity"];
    }

    function testAllDataTypes2() public pure {
        // Declaration of variables (but initialized later) GOOD
        uint delayedUint;
        bool delayedBool;
        address delayedAddress;
        int delayedInt;
        bytes32 delayedBytes32;
        string memory delayedString;
        uint[1] memory delayedUintArray;
        bool[1] memory delayedBoolArray;
        address[1] memory delayedAddressArray;
        int[1] memory delayedIntArray;
        bytes32[1] memory delayedBytes32Array;
        string[1] memory delayedStringArray;

        // Initialize delayedUint via assembly
        assembly ("memory-safe") {
            delayedUint := 6
        }

        // Initialization of variables
        delayedBool = true;
        delayedAddress = 0x0000000000000000000000000000000000000001;
        delayedInt = -42;
        delayedBytes32 = "example";
        delayedString = "initialized later";
        delayedUintArray[0] = 21;
        delayedBoolArray[0] = false;
        delayedAddressArray[0] = 0x0000000000000000000000000000000000000002;
        delayedIntArray[0] = -21;
        delayedBytes32Array[0] = "test";
        delayedStringArray[0] = "array";
    }
}
