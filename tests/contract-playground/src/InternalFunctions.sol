// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract InternalFunctionExample {
    address public owner;
    uint256 public value;

    constructor() {
        owner = msg.sender;
    }

    function setValue(uint256 _newValue) external onlyOwner {
        // Here, the internal function `_internalSet` is called.
        _internalSet(_newValue);
        internalSet2(_newValue);
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can call this function");
        _;
    }

    function _internalSet(uint256 _newValue) internal {
        // This function is called only once within the contract.
        value = _newValue;
    }

    function internalSet2(uint256 _newValue) internal {
        // This function is called only once within the contract.
        value = _newValue;
    }
}
