// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableEvents {

    struct MyStruct {
        uint a;
        uint b;
    }

    uint public result;
    address public result2;
    MyStruct public myStruct;

    event variableChanged(uint outcome);
    event AddressChanged(address outcome);
    event MyStructChanged(MyStruct outcome);
    event MyStructMemberChanged(uint a);

    function stateChangedNoEventEmitted(uint a) public { // bad
        result *= a;
    }

    function addressChangedNotEmitted(address a) public { // bad
        require(a != address(0), "Address cannot be 0");
        result2 = a;
    }

    function addressChangedEventEmitted(address a) public { // good
        require(a != address(0), "Address cannot be 0");
        result2 = a;
        emit AddressChanged(a);
    }

    function wholeStructChangedEventNotEmitted(uint a, uint b) public { // bad
        MyStruct memory temp = MyStruct(a, b);
        myStruct = temp;
    }


    function wholeStructChangedEventEmitted(uint a, uint b) public { // good
        MyStruct memory temp = MyStruct(a, b);
        myStruct = temp;
        emit MyStructChanged(temp);
    }
    
    function structMemberChangedEventNotEmitted(uint a, uint b) public { // bad
        myStruct.a = a;
        myStruct.b = b;
    }

    function structMemberChangedEventEmitted(uint a) public { // good
        myStruct.a = a;
        emit MyStructMemberChanged(a); 
    }

    function stateChangedEventEmitted(uint a) public { // good
        result += a;
        emit variableChanged(result);
    }

    
    function stateChangedEventEmittedForLocal(uint a) public { // good
        result = a;
        emit variableChanged(a);
    }
    

    function noStateChangedNoEventEmitted(uint a) public pure returns(uint){ // good
        uint b = a;
        return b;
    }
}