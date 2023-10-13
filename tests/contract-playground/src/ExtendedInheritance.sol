// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "./InheritanceBase.sol";

contract ExtendedInheritance is InheritanceBase {
    event DoSomethingElse(uint256 somethingElse);

    function doSomething(uint256 somethingElse) external override {
        emit DoSomethingElse(somethingElse);
    }

    function doSomethingElse(address target) external {
        for (uint256 i = 0; i < 3; i++) {
            target.delegatecall(abi.encodeWithSignature("doSomething(uint256)", i));
        }
    }
}