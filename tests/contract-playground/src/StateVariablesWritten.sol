// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract StorageManipulationExamples {
    // Simple state variables
    uint256 public simpleUint;
    int256 public simpleInt;
    bool public simpleBool;
    address public simpleAddress;
    string public simpleString;

    // Array of uint256
    uint256[] public numbersArray;

    // Mapping from address to uint256
    mapping(address => uint256) public balanceOf;

    // Define a mapping from an address to another mapping, which maps an uint256 to an array of int64
    mapping(address => mapping(uint256 => int64[])) private userNestedValues;

    // Struct example
    struct Person {
        string name;
        uint256 age;
    }
    Person public person;

    // Nested mapping
    mapping(address => mapping(uint256 => bool)) public nestedMapping;

    // Function to add a value to the array associated with a specific address and identifier
    function addValue(address _user, uint256 _id, int64 _value) public {
        userNestedValues[_user][_id].push(_value);
    }

    // Simple function to update uint256
    function setSimpleUint(uint256 _value) external {
        simpleUint = _value;
    }

    // Function to update int256
    function setSimpleInt(int256 _value) external {
        simpleInt = _value;
    }

    // Function to delete the array for a specific address and identifier
    function deleteValues(address _user, uint256 _id) public {
        delete userNestedValues[_user][_id];
    }

    // Function to update bool
    function setSimpleBool(bool _value) external {
        simpleBool = _value;
    }

    // Function to update address
    function setSimpleAddress(address _value) external {
        simpleAddress = _value;
    }

    // Function to update string
    function setSimpleString(string calldata _value) external {
        simpleString = _value;
    }

    // Function to add an element to the array
    function addNumber(uint256 _number) external {
        numbersArray.push(_number);
    }

    // Function to remove an element from the array by index
    function removeNumber(uint256 _index) external {
        require(_index < numbersArray.length, "Index out of bounds");
        for (uint256 i = _index; i < numbersArray.length - 1; i++) {
            numbersArray[i] = numbersArray[i + 1];
        }
        numbersArray.pop(); // Remove the last element which is now duplicated
    }

    // Function to set balance for an address
    function setBalance(address _account, uint256 _amount) external {
        balanceOf[_account] = _amount;
    }

    // Function to delete balance for an address
    function deleteBalance(address _account) external {
        delete balanceOf[_account];
    }

    // Function to update Person struct
    function setPerson(string calldata _name, uint256 _age) external {
        person = Person(_name, _age);
    }

    // Function to delete Person struct
    function deletePerson() external {
        delete person;
    }

    // Function to set value in nested mapping
    function setNestedMapping(
        address _account,
        uint256 _key,
        bool _value
    ) external {
        nestedMapping[_account][_key] = _value;
    }

    // Function to delete value from nested mapping
    function deleteNestedMapping(address _account, uint256 _key) external {
        delete nestedMapping[_account][_key];
    }

    // Function to retrieve a value from the nested mapping
    function getNestedMapping(
        address _account,
        uint256 _key
    ) external view returns (bool) {
        return nestedMapping[_account][_key];
    }
}

contract FixedSizeArrayExamples {
    // Constant-sized array of uint256 with 5 elements
    uint256[5] public fixedSizeArray;

    // Function to set values in the fixed-size array
    function setFixedSizeArray(uint256[5] calldata _values) external {
        fixedSizeArray = _values;
    }

    // Function to update a specific element in the fixed-size array
    function updateElement(uint256 _index, uint256 _value) external {
        require(_index < fixedSizeArray.length, "Index out of bounds");
        fixedSizeArray[_index] = _value;
    }

    // Function to get a specific element from the fixed-size array
    function getElement(uint256 _index) external view returns (uint256) {
        require(_index < fixedSizeArray.length, "Index out of bounds");
        return fixedSizeArray[_index];
    }

    // Function to reset the fixed-size array to default values
    function resetArray() external {
        delete fixedSizeArray; // Resets all elements to 0
    }
}
