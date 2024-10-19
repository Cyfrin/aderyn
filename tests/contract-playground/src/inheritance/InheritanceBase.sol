// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IContractInheritance.sol";

contract InheritanceBase is IContractInheritance {
    event Do(uint256 something);

    address public s_baseAddress;

    constructor() {
        s_baseAddress = address(123);
    }

    function doSomething(uint256 something) external virtual {
        emit Do(something);
    }
}
