// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableCouldBeDeclaredImmutable {
    uint256 public immutableValue; // It can be marked immutable
    uint256 public variableValue;

    // aderyn-ignore-next-line(state-variable-could-be-declared-constant)
    address public h = address(3); // This is a candidate for constant

    constructor() {
        immutableValue = 50;
        immutableValue *= 3;
        variableValue = 130;
    }

    // aderyn-ignore-next-line(state-variable-changes-without-events)
    function changeItNow() external {
        variableValue = uint256(uint160(h));
    }
}
