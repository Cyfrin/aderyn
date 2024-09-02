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

    function readSimpleStateVars() external {
        uint256 _a = simpleUint;
        int256 _b;
        _b = simpleInt;
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
        // This is VariableDeclarationStatement, not an Assigment
        Person storage ptr_person3 = person3;
    }

    function manipulateStateVariables4() external {
        // This is VariableDeclarationStatement, not an Assigment
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

    function manipulateStateVariables5() external view {
        Person storage p1;
        p1 = person3;
    }

    function manipulateStateVariables6() external {
        // here, p1 and p2 are manipulated but not p3 and p4
        Person storage p1;
        Person storage p2;
        Person storage p3;
        Person storage p4;
        // These are assigments with tuple expression on lhs and rhs
        (p3, p4, p1, p2, (p1.age, p2.age)) = (
            person3,
            person3,
            person3,
            person3,
            (1398, 1399)
        );
    }

    function manipulateStateVariables7() external {
        person.age += 1;
        person2.age -= 10;
        person3.age *= 100;
    }

    function manipulateStateVariables8() external {
        person.age++;
        person2.age--;
    }
}

library SVManipulationLibrary {
    function manipulateLib(
        StructPlusFixedArrayAssignmentExample.Person storage p
    ) internal {
        p.age = 200;
    }

    function manipulateLib2()
        internal
        returns (StructPlusFixedArrayAssignmentExample.Person storage p2)
    {
        assembly {
            p2.slot := 0x00
        }
        p2.age = 200;
    }

    function manipulateLib3() internal {
        StructPlusFixedArrayAssignmentExample.Person storage p1;
        StructPlusFixedArrayAssignmentExample.Person storage p2;
        assembly {
            p1.slot := 0x00
            p2.slot := 0x01
        }
        (p1.name, p2.age) = ("Hello", 400);
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

    // NO state vars manipulated here
    function dontManipulateStateVariablesPart3() external {
        uint256 theAge = person.age;
        uint256 theAge2;
        theAge2 = person.age;
    }

    // Only person2 state variable is read here. Also it's "directly read" (aka not through storage pointers)
    // Other initializations are merely just references.
    function dontManipulateStateVariablesPart4() external {
        (uint256 theAge3, uint256 theAge4) = (340, bytes(person2.name).length);
        uint256[5] storage ageRef1 = allAges[0];
        Person storage dr = dummy;
    }

    // Only `v` is the one that's causing a read. Also, this is "indirectly" read (through a storage pointer)
    function dontManipulateStateVariablesPart5() external {
        uint256[5] storage ageRef2 = allAges[0];
        uint256 v = ageRef2[3];
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

/***** PUSH OPERATIONS *****/

// This contract should catch 3 state variables undergoing a delete operation
contract DynamicArraysPushExample {
    // Struct example
    struct Person {
        string name;
        uint256 age;
        uint256[] some;
    }

    Person[] public directlyPushMe;
    Person[][5] public indexMeToPush;
    Person[][] public indexMeTwiceToPush;

    uint256[] public indexMeToPushDArray;
    Person public p;

    // This shouldn't be caught! (it's not being manipulated)
    uint256[5] public dummy;

    // 1 State variable is pushed to here
    function manipulateDirectly() external {
        directlyPushMe.push(
            Person({name: "dfsf", age: 610, some: indexMeToPushDArray})
        );
    }

    // 3 more state variables pushed to here
    function manipulateViaIndexAccess() external {
        indexMeToPush[dummy.length].push(
            Person({name: "dfsf", age: 710, some: indexMeToPushDArray})
        );
        indexMeTwiceToPush[dummy.length].push(
            Person({name: "dfsf", age: 810, some: indexMeToPushDArray})
        );
        indexMeToPushDArray.push(12309);
    }

    // 1 state variable pushed to here
    function manipulateViaMemberAccess() external {
        p.some.push(102);
    }

    // 1 storage pointer pushed to here
    function manipulateViaMemberAccess2() external {
        uint256[] storage that = p.some;
        that.push(102);
    }
}

contract DynamicMappingsArrayPushExample {
    // Struct example
    struct Person {
        string name;
        uint256 age;
        uint256[] some;
    }

    mapping(uint256 => Person) myMap;

    function add(uint256 i) external {
        myMap[i].some.push(304);
    }
}
