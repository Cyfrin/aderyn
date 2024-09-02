// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

interface SVIERC20 {}

contract StateVariableCouldBeDeclaredConstant {
    // This state variable could be declared constant but isn't
    uint256 public constantValue = 100;

    // Other state variables
    uint256 public variableValue; // This one cannot be marked constant. (It can be marked immutable)

    SVIERC20 public h = SVIERC20(address(3)); // This could be declared constant

    constructor() {
        variableValue = 50;
    }

    function getConstantValue() external view returns (uint256) {
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

contract StateVariableCouldBeDeclaredConstant2Child is
    StateVariableCouldBeDeclaredConstant2
{
    function changeIt() external {
        cannotBeconstantValue = 130;
    }
}
