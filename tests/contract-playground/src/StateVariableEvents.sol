// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableEvents {
    uint public result;

    event variableChanged(uint outcome);

    // functions that should trigger StateVariableNotLoggedInEventDetector detector
    function stateChangedNoEventEmitted(uint a) public {
        result = a;
    }

    function stateChangedNoEventEmittedDifferentOperator(uint a) public {
        result /= a;
    }

    function stateChangedDifferentEventEmitted(uint a, uint b) public {
        result *= a;
        emit variableChanged(b);
    }

     // functions that should not trigger StateVariableNotLoggedInEventDetector detector

    function stateChangedEventEmittedRecordLeft(uint a) public {
        result += a;
        emit variableChanged(result);
    }

    function stateChangedEventEmittedRecordRight(uint a) public {
        result = a;
        emit variableChanged(a);
    }

    function noStateChangedNoEventEmitted(uint a) public pure returns(uint){
        uint b = a;
        return b;
    }
}