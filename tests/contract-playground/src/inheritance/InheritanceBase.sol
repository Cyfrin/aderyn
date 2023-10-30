// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IContractInheritance.sol";

contract InheritanceBase is IContractInheritance {
    event Do(uint256 something);

    function doSomething(uint256 something) external virtual {
        emit Do(something);
    }
}