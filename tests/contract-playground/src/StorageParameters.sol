// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

contract StorageParameters {
    uint[1] public storageArray;

    function editArrays() external {
        uint[1] memory memoryArray;

        editStorage(storageArray);
        editMemory(storageArray); // BAD LINE

        // editStorage(memoryArray); // compiler error
        editMemory(memoryArray);
    }

    function editStorage(uint[1] storage arr) internal {
        arr[0] = 1;
    }

    function editMemory(uint[1] memory arr) internal pure {
        arr[0] = 2;
    }
}
