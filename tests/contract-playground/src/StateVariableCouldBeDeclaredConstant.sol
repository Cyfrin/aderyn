// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract StateVariableCouldBeDeclaredConstant {
    // This state variable could be declared constant but isn't
    uint256 public constantValue = 100;
    
    // Other state variables
    uint256 public variableValue;
    
    constructor() {
        variableValue = 50;
    }
    
    function getConstantValue() public view returns (uint256) {
        return constantValue;
    }
}


contract StateVariableCouldBeDeclaredConstant2 {
    // This state variable could NOT be declared constant
    uint256 public cannotBeconstantValue = 100;
    
    // Other state variables
    uint256 public variableValue;
    
    constructor() {
        variableValue = 50;
    }
    
}

contract StateVariableCouldBeDeclaredConstant2Child is StateVariableCouldBeDeclaredConstant2 {
    function changeIt() external {
        cannotBeconstantValue = 130;
    }
}
