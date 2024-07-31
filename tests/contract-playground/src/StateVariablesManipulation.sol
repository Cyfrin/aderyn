// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/***** ASSIGNMENT OPERATIONS *****/

contract NoStateVarManipulationExample {
    // Simple state variables
    uint256 public simpleUint;
    int256 public simpleInt;
    bool public simpleBool;
    address public simpleAddress;
    address payable public simplePayableAddress;
    string public simpleString;
    bytes public simpleBytes;

    // There are NO state variables assigned in this function
    function dontManipulateStateVar() external view {
        // VariableDeclaration (initialize with values)
        uint256 a = simpleUint * 100;
        int256 b = simpleInt * int256(a);
        bool c = simpleBool;
        bytes memory g = simpleBytes;

        // VariableDeclarationStatements (initialize)
        address d;
        address payable e;
        string memory f;

        // Assignments
        d = simpleAddress;
        e = simplePayableAddress;
        f = simpleString;
    }
}

// This contract should catch a total of 7 state variables manipulated
contract SimpleStateVarManipulationExample {
    // Simple state variables (2 of them assigned to here)
    uint256 public simpleUint;
    int256 public simpleInt = 100;
    bool public simpleBool = true;
    address public simpleAddress;
    address payable public simplePayableAddress;
    string public simpleString;
    bytes public simpleBytes;

    // Remaining 5 state variables manipulated in this function
    function manipulateStateVarDirectly(
        uint256 a,
        address d,
        address payable e,
        string calldata f,
        bytes calldata g
    ) external {
        simpleUint = a;
        simpleAddress = d;
        simplePayableAddress = e;
        simpleString = f;
        simpleBytes = g;
    }
}

// This contract should catch 4 state variables being assigned
contract FixedSizeArraysAssignmentExample {
    uint256[5] public directlyAssignMe;
    uint256[5] public assignToMeNow = [1, 4, 5, 8, 9]; // 1 state var assigned here
    uint256[5] public indexMeToAssign;
    uint256[5] public indexMeTwiceToAssign;

    // This shouldn't be caught! (it's not being manipulated)
    uint256[5] public dummy;

    // 1 State variable is assigned here
    function manipulateDirectly(uint256[5] calldata _values) external {
        directlyAssignMe = _values;
    }

    // 2 more state variables assigned here
    function manipulateViaIndexAccess() external {
        // 1st - indexMeToAssign
        indexMeToAssign[0] = directlyAssignMe[0] + directlyAssignMe[1];
        indexMeToAssign[1] = dummy[0] + dummy[1];

        // 2nd - indexMeTwiceToAssign
        indexMeTwiceToAssign[dummy[dummy[0]]] = directlyAssignMe[3];
    }
}

contract StructPlusFixedArrayAssignmentExample {
    // Struct example
    struct Person {
        string name;
        uint256 age;
    }

    using SVManipulationLibrary for Person;

    // Simple Assignments
    Person public person;
    Person public person2;
    uint256[5][1] public allAges;

    // Complex assignments
    Person public person3;
    Person[5][1] public persons;

    // More Complex Assignment
    Person[5][1] public personsUltimate;

    // This is not manipulated
    Person public dummy;

    // 3 state vars manipulated here
    function manipulateStateVariables() external {
        person = Person("Spiderman", 21);
        allAges[person.age][0] = dummy.age;
        person2.age = 21;
    }

    function manipulateStateVariables2() external {
        Person storage ptr_person = person3;
        ptr_person.name = "Changed";

        ptr_person = persons[dummy.age][0];
        ptr_person.name = "Changed";
    }

    function manipulateStateVariables3() external {
        Person storage ptr_person = person3;
        ptr_person.name = "Changed";

        ptr_person = persons[dummy.age][0];
        ptr_person.name = "Changed";
    }

    function manipulateStateVariables4() external {
        (Person storage p1, ) = manipulateHelper(personsUltimate[0][0]);
        manipulateHelper(p1);
        p1.manipulateLib();
    }

    function manipulateHelper(
        Person storage h
    ) internal returns (Person storage, Person storage) {
        h.name = "H";
        Person storage p1 = person3;
        p1.age = 130;
        return (p1, p1);
    }
}

library SVManipulationLibrary {
    function manipulateLib(
        StructPlusFixedArrayAssignmentExample.Person storage p
    ) internal {
        p.age = 200;
    }
}

contract NoStructPlusFixedArrayAssignmentExample {
    // Struct example
    struct Person {
        string name;
        uint256 age;
    }

    Person public person;
    Person public person2;
    uint256[5][1] public allAges;

    Person public dummy;

    // NO state vars manipulated here
    function dontManipulateStateVariables() external {
        Person memory m_person;
        Person memory m_person2;
        uint256[5][1] memory m_allAges;

        Person memory m_dummy;

        m_person = Person("Spiderman", 21);
        m_allAges[person.age][0] = dummy.age;
        m_person2.age = 21;
    }

    // NO state vars manipulated here
    function dontManipulateStateVariablesPart2() external {
        Person memory m_dummy = Person("Spiderman", 21);
    }
}

/***** DELETE OPERATIONS *****/

// This contract should catch 3 state variables undergoing a delete operation
contract FixedSizeArraysDeletionExample {
    uint256[5] public directlyDeleteMe;
    uint256[5] public indexMeToDelete;
    uint256[5] public indexMeTwiceToDelete;

    // This shouldn't be caught! (it's not being manipulated)
    uint256[5] public dummy;

    // 1 State variable is deleted here
    function manipulateDirectly() external {
        delete directlyDeleteMe;
    }

    // 2 more state variables deleted here
    function manipulateViaIndexAccess() external {
        delete indexMeToDelete[0];
        delete indexMeTwiceToDelete[dummy[dummy[0]]];
    }
}
