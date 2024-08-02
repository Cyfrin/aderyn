// SPDX-License-Identifier: MIT
pragma solidity ^0.4.0;

contract CompilerBugStorageSignedIntegerArray {
    int256[3] affectedArray;
    int256[4] unaffectedArray;

    function assignBadValue() private {
        affectedArray = [-1, 5, 2];
    }

    function assignGoodValue() private {
        unaffectedArray[0] = -1;
        unaffectedArray[1] = 5;
        unaffectedArray[2] = 2;
    }
}
