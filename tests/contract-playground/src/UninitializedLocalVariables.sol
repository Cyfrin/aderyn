// SPDX-License-Identifier: MITc
pragma solidity ^0.8.0;

contract AllDataTypes {

    uint256 stateVarUint; 

    function testAllDataTypes() public pure {
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
        

        // Initialized variables (GOOD)
        uint initializedUint = 1;
        bool initializedBool = true;
        address initializedAddress = 0x0000000000000000000000000000000000000000;
        int initializedInt = -1;
        bytes32 initializedBytes32 = "hello";
        string memory initializedString = "world";
        uint[1] memory initializedUintArray = [uint(2)];
        bool[1] memory initializedBoolArray = [false];
        address[1] memory initializedAddressArray = [0x0000000000000000000000000000000000000001];
        int[1] memory initializedIntArray = [int(-2)];
        bytes32[1] memory initializedBytes32Array = [bytes32("bye")];
        string[1] memory initializedStringArray = ["Solidity"];

        {
            // Example usage of initialized and uninitialized variables
            uint sum = initializedUint + uninitializedUint;
            bool conjunction = initializedBool && uninitializedBool;
            address comparison = initializedAddress == uninitializedAddress ? initializedAddress : uninitializedAddress;
            int difference = initializedInt - uninitializedInt;
            bytes32 combinedBytes32 = keccak256(abi.encodePacked(initializedBytes32, uninitializedBytes32));
            string memory combinedString = string(abi.encodePacked(initializedString, uninitializedString));

            // Example usage of arrays
            uint arraySum = initializedUintArray[0] + uninitializedUintArray[0];
            bool arrayConjunction = initializedBoolArray[0] && uninitializedBoolArray[0];
            address arrayComparison = initializedAddressArray[0] == uninitializedAddressArray[0] ? initializedAddressArray[0] : uninitializedAddressArray[0];
            int arrayDifference = initializedIntArray[0] - uninitializedIntArray[0];
            bytes32 arrayCombinedBytes32 = keccak256(abi.encodePacked(initializedBytes32Array[0], uninitializedBytes32Array[0]));
            string memory arrayCombinedString = string(abi.encodePacked(initializedStringArray[0], uninitializedStringArray[0]));
            
            // These statements are to prevent warnings about unused variables
            sum; conjunction; comparison; difference; combinedBytes32; combinedString;
            arraySum; arrayConjunction; arrayComparison; arrayDifference; arrayCombinedBytes32; arrayCombinedString;
        }
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

        // Example usage of initialized variables
        uint sum = delayedUint + 1;
        bool conjunction = delayedBool && false;
        address comparison = delayedAddress == 0x0000000000000000000000000000000000000001 ? delayedAddress : address(0);
        int difference = delayedInt - 1;
        bytes32 combinedBytes32 = keccak256(abi.encodePacked(delayedBytes32, "concat"));
        string memory combinedString = string(abi.encodePacked(delayedString, " now"));

        // Example usage of arrays
        uint arraySum = delayedUintArray[0] + 1;
        bool arrayConjunction = delayedBoolArray[0] && true;
        address arrayComparison = delayedAddressArray[0] == 0x0000000000000000000000000000000000000002 ? delayedAddressArray[0] : address(0);
        int arrayDifference = delayedIntArray[0] - 1;
        bytes32 arrayCombinedBytes32 = keccak256(abi.encodePacked(delayedBytes32Array[0], " more"));
        string memory arrayCombinedString = string(abi.encodePacked(delayedStringArray[0], " elements"));

        // These statements are to prevent warnings about unused variables
        sum; conjunction; comparison; difference; combinedBytes32; combinedString;
        arraySum; arrayConjunction; arrayComparison; arrayDifference; arrayCombinedBytes32; arrayCombinedString;
    }

}
