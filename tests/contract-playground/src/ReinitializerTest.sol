// SPDX-License-Identifier: MIT

pragma solidity 0.8.19;

// Mock OpenZeppelin UUPSUpgradeable contract with reinitializer
abstract contract Initializable {
    /**
     * @dev Indicates that the contract has been initialized.
     */
    bool private _initialized;

    /**
     * @dev Indicates that the contract is in the process of being initialized.
     */
    bool private _initializing;

    /**
     * @dev Modifier to protect an initializer function from being invoked twice.
     */
    modifier initializer() {
        require(!_initialized, "Initializable: contract is already initialized");
        _initialized = true;
        _;
    }

    /**
     * @dev Modifier to protect a reinitializer function from being invoked during the same upgrade.
     *
     * `version` must be different from the previous invocation of a reinitializer.
     */
    modifier reinitializer(uint8 version) {
        require(!_initializing, "Initializable: contract is initializing");
        require(_initialized, "Initializable: contract is not initialized");
        _;
    }
}

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