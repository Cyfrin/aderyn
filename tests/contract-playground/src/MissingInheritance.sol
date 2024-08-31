// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

// Set 1

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

// Set 2

interface IMissingParent {
    function parent() external view returns (bool);
}

interface IMissingChild is IMissingParent {
    function child() external view returns (uint256);
}

// BAD (it could have implemented IMissingChild)
contract MissingContract2 {
    function child() external pure returns (bool) {
        return true;
    }

    function parent() external pure returns (uint256) {
        return 10;
    }
}

// GOOD (it inherits IMissingChild)
contract MissingContract3 is IMissingChild {
    function parent() external pure returns (bool) {
        return true;
    }

    function child() external pure returns (uint256) {
        return 10;
    }
}
