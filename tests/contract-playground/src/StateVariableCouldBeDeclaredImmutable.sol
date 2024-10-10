// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StateVariableCouldBeDeclaredImmutable {
    // BAD (this could be marked immutable)
    uint256 potentiallyImmutableUint;

    // BAD (this could be marked immutable)
    address potentiallyImmutableAddr;

    // GOOD
    uint256 immutable goodValue;

    uint256 notImmutable;
    uint256 seeminglyImmutable;

    constructor() {
        potentiallyImmutableUint = 10; // aderyn-ignore
        potentiallyImmutableAddr = address(10); // aderyn-ignore
        potentiallyImmutableUint *= 20; // aderyn-ignore
        notImmutable = 10; // aderyn-ignore
        goodValue = 10; // aderyn-ignore
        callSecretFunc();
    }

    function callSecretFunc() internal {
        // NOTE: Although this function is only called by the constructor, it may appear
        // as if `seeminglyImmutable` can be declared immutable because no other function
        // changes it's state. However solidity puts a constraint which is that for a variable
        // to be immutable, it should only be changed in the constructor or inlined in
        // where it's defined. So the flow would be that the user is first notified that this
        // internal function can be inlined into the constructor (coz that's the only place it's called)
        // Then, this immutable detector picks it up and flags it as potentially immutable.
        seeminglyImmutable = 3; // aderyn-ignore
    }

    function changeNotImmutableVar() external {
        notImmutable *= 3; // aderyn-ignore
    }
}
