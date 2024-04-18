// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

interface IContractInheritance {
    error BigError(uint256 something);

    function doSomething(uint256 something) external;
}
