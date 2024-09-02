// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract DeadCodeExample {
    uint256 public value;

    constructor(uint256 _value) {
        value = _value;
    }

    function setValue(uint256 _value) external {
        value = _value;
    }

    // BAD - Dead code: This function is implemented but never used anywhere in the contract
    function unusedInternalFunction() internal pure returns (string memory) {
        return "This function is never called";
    }

}

abstract contract DeadCodeExample2 {

    // GOOD - NOT Dead code: (It's inside an abstract contract)
    function unusedInternalFunction() internal pure returns (string memory) {
        return "This function is never called";
    }

}


library DeadCodeExample3 {

    // GOOD - NOT Dead code: (It's inside a library)
    function unusedInternalFunction() internal pure returns (string memory) {
        return "This function is never called";
    }

}
