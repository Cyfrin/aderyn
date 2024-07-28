// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableEvents {
    uint public result;

    event variableChanged(uint outcome);

    function stateChangedNoEventEmitted(uint a) public {
        result *= a;
    }

    function stateChangedEventEmitted(uint a) public {
        result += a;
        emit variableChanged(result);
    }

    
    function stateChangedEventEmittedForLocal(uint a) public {
        result = a;
        emit variableChanged(a);
    }
    

    function noStateChangedNoEventEmitted(uint a) public pure returns(uint){
        uint b = a;
        return b;
    }
}