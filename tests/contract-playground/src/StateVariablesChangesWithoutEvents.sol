// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableEvents {

    struct MyStruct {
        uint a;
        uint b;
    }

    uint public result;
    address public result2;
    bool public result3;
    MyStruct public myStruct;

    uint256 constant NUMBER = 123;

    event variableChanged(uint outcome);
    event variableChanged2(bool outcome);
    event AddressChanged(address outcome);
    event MyStructChanged(MyStruct outcome);
    event MyStructMemberChanged(uint a);

    function stateChangedNoEventEmitted(uint a) external { // bad
        result *= a;
    }

    function stateChangedNoEventEmittedPlusEquals(uint a) external { // bad
        result += a;
    }

    function stateChangedToBooleanNoEventEmitted() external { // bad
        result3 = true;
    }

    function stateChangedToBooleanEventEmitted() external { // good
        result3 = true;

        emit variableChanged2(true);
    }

    function stateChangedToConstantNoEventEmitted() external { // bad
        result = NUMBER;
    }

    function stateChangedNoEventEmitted() external { // bad
        result = block.timestamp;
    }

    function addressChangedNotEmitted(address a) external { // bad
        require(a != address(0), "Address cannot be 0");
        result2 = a;
    }

    function addressChangedEventEmitted(address a) external { // good
        require(a != address(0), "Address cannot be 0");
        result2 = a;
        emit AddressChanged(a);
    }

    function wholeStructChangedEventNotEmitted(uint a, uint b) external { // bad
        MyStruct memory temp = MyStruct(a, b);
        myStruct = temp;
    }


    function wholeStructChangedEventEmitted(uint a, uint b) external { // good
        MyStruct memory temp = MyStruct(a, b);
        myStruct = temp;
        emit MyStructChanged(temp);
    }

    function structMemberChangedEventNotEmitted(uint a, uint b) external { // bad
        myStruct.a = a;
        myStruct.b = b;
    }

    function structMemberChangedEventEmitted(uint a) external { // good
        myStruct.a = a;
        emit MyStructMemberChanged(a); 
    }

    function stateChangedEventEmitted(uint a) external { // good
        result += a;
        emit variableChanged(result);
    }

    // GOOD because there is some event emitted even though the contents
    // of the event doesn't directly reprsent the state variable's value
    function stateChangedEventEmittedForLocal(uint a) external { // good
        result += a;
        emit variableChanged(a);
    }


    function stateChangedEventEmittedForLocalEquals(uint a) external { // good
        result = a;
        emit variableChanged(a);
    }


    function noStateChangedNoEventEmitted(uint a) external pure returns(uint){ // good
        uint b = a;
        return b;
    }
}
