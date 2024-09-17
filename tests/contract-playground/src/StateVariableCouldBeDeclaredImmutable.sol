// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

// aderyn-ignore-next-line
contract StateVariableCouldBeDeclaredImmutable {
    uint256 public immutableValue; // It can be marked immutable
    uint256 public seeminglyImmutableValue; // It cannot be marked immutable
    uint256 public variableValue;

    uint256 private immutable x;

    constructor() {
        x = 103; // aderyn-ignore
        immutableValue = 50; // aderyn-ignore
        immutableValue *= 3; // aderyn-ignore
        variableValue = 130; // aderyn-ignore
        changeSeeminglyImmutableValue();
    }

    // aderyn-ignore-next-line
    function changeSeeminglyImmutableValue() internal {
        // When you try to initialize an immutable value in an internal function that is exclsively
        // called by the constructor, yuo get the following error:

        // ERROR:
        // Cannot write to immutable here: Immutable variables can only be
        // initialized inline or assigned directly in the constructor.

        // As a result even though `seeminglyImmutableValue` is init only in the constructor, because it's
        // value is not directly assigned in the constructor definition, we must not consider that as a potentially
        // suitable immutable variable

        seeminglyImmutableValue = 130; // aderyn-ignore
    }

    function changeVariableValue() external {
        variableValue += variableValue;
    }
}
