// SPDX-License-Identifier: MIT
pragma solidity 0.5.8;

contract FunctionPointerExample {

    // A simple add function
    function add(uint a, uint b) public pure returns (uint) {
        return a + b;
    }

    constructor() public {
        // Declare a function type that takes two uint arguments and returns a uint
        function(uint, uint) pure returns (uint) operation;
        
        // Assign the add function to the operation variable
        operation = add;
    }
  
}
