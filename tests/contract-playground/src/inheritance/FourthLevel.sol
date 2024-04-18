// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "./ExtendedInheritance.sol";
import "../../lib/openzeppelin-contracts/contracts/utils/structs/EnumerableSet.sol";

contract FourthLevel is ExtendedInheritance {

    function foo() external {
        emit Do(0);
    }

    function bar() external {
        emit DoSomethingElse(1);
    }

    function baz() external pure {
        revert BigError(3);
    }
}