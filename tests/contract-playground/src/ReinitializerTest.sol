// SPDX-License-Identifier: MIT

import "../lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol";

pragma solidity 0.8.19;

/**
 * @title ReinitializerContract
 * @dev Test contract for verifying the Unprotected Initializer detector behavior
 * with OpenZeppelin's initializer and reinitializer modifiers
 */
contract ReinitializerContract is Initializable {
    uint256 private value;
    uint8 private version;

    // GOOD: Uses initializer
    function initialize() external initializer {
        value = 100;
        version = 1;
    }

    // GOOD: Uses reinitializer
    function reinitialize() external reinitializer(2) {
        value = 200;
        version = 2;
    }

    // BAD: Initialization function without protection
    function initializeWithoutProtection() external {
        value = 300;
        version = 3;
    }
} 