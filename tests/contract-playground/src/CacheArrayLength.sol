// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract CacheArrayLength {
    uint[] array;
    uint[] anotherArray;

    function c1() external view {
        uint total = 0;

        // BAD (length is not cached)
        for (uint i = 0; i < array.length; i++) {
            total += array[i];
        }
    }

    function c2() external view {
        uint array_length = array.length;
        // GOOD (length is cached)
        for (uint i = 0; i < array_length; i++) {}
    }

    function c3() external {
        // GOOD
        for (uint i = 0; i < array.length; i++) {
            array[i] = 100;
            array.push(100);
            array.pop();
        }
    }

    function c4() external {
        // GOOD (even though anotherArray.length doesn't change, static analysis should assume that the condition as
        // a whole can change even if just one of the involved state variable changes)
        for (uint i = 0; i < array.length + anotherArray.length; i++) {
            array[i] = 100;
            array.push(100);
            array.pop();
        }
    }

    function c5() external view {
        // BAD (can cache)
        for (uint i = 0; i < array.length + anotherArray.length; i++) {}
    }

    function c6() external view {
        uint total = 0;
        // BAD (can cache)
        for (uint i = 0; i < array.length + anotherArray.length; i++) {
            // Only reading from storage (not changing them)
            total += array[i] * anotherArray[i];
        }
    }
}
