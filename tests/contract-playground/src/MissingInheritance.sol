// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

// BAD
contract MissingInheritanceCounter {
    uint256 public count;

    function increment() external {
        count += 1;
    }
}

interface IMissingInheritanceCounter {
    function count() external view returns (uint256);

    function increment() external;
}

// GOOD
contract MissingInheritanceCounter2 is IMissingInheritanceCounter {
    uint256 public count;

    function increment() external {
        count += 1;
    }
}
