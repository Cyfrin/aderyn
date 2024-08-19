// SPDX-License-Identifier: MIT
pragma solidity ^0.4.0;

contract ConstantFunctionChangeState {
    uint counter;

    // BAD (it is declared as view but changes state)
    function changeState() public view returns (uint) {
        counter = counter + 1;
        return counter;
    }

    // GOOD (because it's not declared as view)
    function changeState2() public returns (uint) {
        counter = counter + 1;
        return counter;
    }

    // GOOD (it's declared as view and it doesn't change state)
    function dontChangeState() public view returns (uint) {
        return counter + 1;
    }
}
