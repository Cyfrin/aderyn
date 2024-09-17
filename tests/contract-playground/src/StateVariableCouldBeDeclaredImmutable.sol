// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableCouldBeDeclaredImmutable {
    uint256 public immutableValue; // It can be marked immutable
    uint256 public seeminglyImmutableValue; // It cannot be marked immutable
    uint256 public variableValue;

    // aderyn-ignore-next-line(state-variable-could-be-declared-constant)
    address public h = address(3); // This is a candidate for constant

    uint256 private immutable x;

    constructor() {
        x = 103;
        immutableValue = 50;
        immutableValue *= 3;
        variableValue = 130;
        changeSeeminglyImmutableValue();
    }

    function changeSeeminglyImmutableValue() internal {
        // When you try to initialize an immutable value in an internal function that is exclsively
        // called by the constructor, yuo get the following error:

        // ERROR:
        // Cannot write to immutable here: Immutable variables can only be
        // initialized inline or assigned directly in the constructor.

        // As a result even though `seeminglyImmutableValue` is init only in the constructor, because it's
        // value is not directly assigned in the constructor definition, we must not consider that as a potentially
        // suitable immutable variable

        // TODO: Future improvement:detect cases where an internal function is esclusively called by the
        // constructor and mutates a state variable and suggest that it be done in the constructor function itself
        // So this detector can then pick it up.
        seeminglyImmutableValue = 130;
    }

    // aderyn-ignore-next-line(state-variable-changes-without-events)
    function changeItNow() external {
        variableValue = uint256(uint160(h));
    }
}
