// SPDX-License-Identifier: No License

import "../lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol";

pragma solidity 0.8.19;

contract InitializedContract is Initializable {
    bool private initialized;
    address private owner;

    constructor() {
        owner = msg.sender;
    }

    modifier firstTimeInitializing() {
        require(!initialized, "Contract already initialized");
        _;
    }

    // GOOD
    function initializeWithModifier() external firstTimeInitializing {
        initialized = true;
        // Additional initialization logic here
    }

    // GOOD
    function initializeWithRevert() external {
        if (msg.sender != owner) {
            revert("Only owner can initialize");
        }
        require(!initialized, "Contract already initialized");
        initialized = true;
        // Additional initialization logic here
    }

    // BAD
    function initializeWithoutModifierOrRevert() external {
        initialized = true;
        // Additional initialization logic here
    }

    // GOOD
    function initializeWithModifierNamedInitiliazer() external initializer {
        // Additional initialization logic here
    }
}
