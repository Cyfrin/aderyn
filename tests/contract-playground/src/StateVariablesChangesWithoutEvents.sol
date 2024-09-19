// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableEvents {
    struct MyStruct {
        uint256 a;
        uint256 b;
    }

    uint256 public result;
    address public result2;
    bool public result3;
    MyStruct public myStruct;

    uint256 constant NUMBER = 123;

    event variableChanged(uint256 indexed outcome);
    event variableChanged2(bool indexed outcome);
    event AddressChanged(address indexed outcome);
    event MyStructChanged(MyStruct indexed outcome);
    event MyStructMemberChanged(uint256 indexed a);

    function stateChangedNoEventEmitted(uint256 a) external {
        // bad
        result *= a;
    }

    function stateChangedNoEventEmittedPlusEquals(uint256 a) external {
        // bad
        result += a;
    }

    function stateChangedToBooleanNoEventEmitted() external {
        // bad
        result3 = true;
    }

    function stateChangedToBooleanEventEmitted() external {
        // good
        result3 = true;

        emit variableChanged2(true);
    }

    function stateChangedToConstantNoEventEmitted() external {
        // bad
        result = NUMBER;
    }

    function stateChangedNoEventEmitted() external {
        // bad
        result = block.timestamp;
    }

    function addressChangedNotEmitted(address a) external {
        // bad
        require(a != address(0), "Address cannot be 0");
        result2 = a;
    }

    function addressChangedEventEmitted(address a) external {
        // good
        require(a != address(0), "Address cannot be 0");
        result2 = a;
        emit AddressChanged(a);
    }

    function wholeStructChangedEventNotEmitted(uint256 a, uint256 b) external {
        // bad
        MyStruct memory temp = MyStruct(a, b);
        myStruct = temp;
    }

    function wholeStructChangedEventEmitted(uint256 a, uint256 b) external {
        // good
        MyStruct memory temp = MyStruct(a, b);
        myStruct = temp;
        emit MyStructChanged(temp);
    }

    function structMemberChangedEventNotEmitted(uint256 a, uint256 b) external {
        // bad
        myStruct.a = a;
        myStruct.b = b;
    }

    function structMemberChangedEventEmitted(uint256 a) external {
        // good
        myStruct.a = a;
        emit MyStructMemberChanged(a);
    }

    function stateChangedEventEmitted(uint256 a) external {
        // good
        result += a;
        emit variableChanged(result);
    }

    // GOOD because there is some event emitted even though the contents
    // of the event doesn't directly reprsent the state variable's value
    function stateChangedEventEmittedForLocal(uint256 a) external {
        // good
        result += a;
        emit variableChanged(a);
    }

    function stateChangedEventEmittedForLocalEquals(uint256 a) external {
        // good
        result = a;
        emit variableChanged(a);
    }

    function noStateChangedNoEventEmitted(
        uint256 a
    ) external pure returns (uint) {
        // good
        uint256 b = a;
        return b;
    }
}
