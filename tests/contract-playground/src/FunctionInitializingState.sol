// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// WRONG WAY
contract StateVarInitFromFunction {
    uint public v = set(); // Initialize from function (sets to 77)
    uint public w = 5;
    uint public x = set(); // Initialize from function (sets to 88)
    uint public f = tes();
    uint public h = okay();
    uint public m = okay2();
    address public constant shouldntBeReported = address(8);

    constructor() {
        // The constructor is run after all state variables are initialized.
    }

    // BAD (not marked as view)
    function set() public returns (uint) {
        // If this function is being used to initialize a state variable declared
        // before w, w will be zero. If it is declared after w, w will be set.
        if (w == 0) {
            return 77;
        }

        return 88;
    }

    // BAD (marked as view)
    function tes() public view returns (uint) {
        // If this function is being used to initialize a state variable declared
        // before w, w will be zero. If it is declared after w, w will be set.
        if (w == 0) {
            return 77;
        }

        return 88;
    }

    // GOOD (doesn't reference non constant state variable)
    function okay() public returns (uint f) {
        f = 1e10 * 1e12;
    }

    // GOOD (doesn't reference non constant state variable)
    function okay2() public returns (uint f) {
        if (msg.sender == shouldntBeReported) {
            f = 1e10 * 1e12;
        } else {
            f = 10;
        }
    }
}

// CORRECT WAY
contract StateVarInitFromFunction2 {
    uint public v;
    uint public w = 5;
    uint public x;

    constructor() {
        v = set();
        x = set();
    }

    // GOOD here, although it refrences non constant variable, it is called from inside the constructor!
    function set() public returns (uint) {
        // If this function is being used to initialize a state variable declared
        // before w, w will be zero. If it is declared after w, w will be set.
        if (w == 0) {
            return 77;
        }

        return 88;
    }
}
