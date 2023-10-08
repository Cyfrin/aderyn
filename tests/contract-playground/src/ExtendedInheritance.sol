// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import "./InheritanceBase.sol";

contract ExtendedInheritance is InheritanceBase {
    event DoSomethingElse(uint256 somethingElse);

    function doSomething(uint256 somethingElse) external override {
        emit DoSomethingElse(somethingElse);
    }
}