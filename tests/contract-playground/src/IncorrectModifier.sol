// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract IncorrectModifierContract {
    address public owner;
    bool public isPaused;

    // Event to log changes
    event OwnershipTransferred(
        address indexed oldOwner,
        address indexed newOwner
    );
    event ContractPaused(bool indexed isPaused);

    error StreamError();

    // Constructor to set the initial owner
    constructor() {
        owner = msg.sender;
        isPaused = false;
    }

    /*/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        BAD modifiers (They don't revert the transaction)
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    // Modifier to check if the caller is the owner
    // BAD
    modifier onlyOwner() {
        if (msg.sender == owner) {
            _; // Continue execution if the condition is met
        }
        // Note: No revert or require, so this does nothing if the condition is not met
    }

    // // Modifier to check if the contract is not paused
    // // BAD
    modifier whenNotPaused() {
        if (!isPaused) {
            _; // Continue execution if the condition is met
        }
        // Note: No revert or require, so this does nothing if the contract is paused
    }

    // BAD
    modifier whenNotPaused2() {
        if (!isPaused) {
            return;
        }
        _;
        // Note: No revert or require here as well
    }

    /*/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        GOOD modcifiers (These revert the transactions)
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    modifier notNullGOOD(uint256 streamId) {
        if (streamId > 100) {
            revert StreamError();
        }
        _;
    }

    modifier notNull2GOOD(uint256 streamId) {
        if (streamId > 100) {
            revert();
        }
        _;
    }

    // Modifier to check if the caller is the owner
    modifier onlyOwnerGOOD() {
        require(msg.sender == owner, "Not the owner");
        _; // Continue execution if the condition is met
    }

    // Modifier to check if the contract is not paused
    modifier whenNotPausedGOOD() {
        require(!isPaused, "Contract is paused");
        _; // Continue execution if the condition is met
    }

    // Modifier to ensure the contract is in a specific state (example: not paused)
    modifier whenPausedGOOD() {
        _checkInternallyForPaused();
        _; // Continue execution if the condition is met
    }

    modifier revertsButWithoutUsingRevertKeyword() {
        payable(address(0x3000)).transfer(130);
        _; // Continue execution if the condition is met
    }

    //////////////////////////// FUNCTIONS //////////////////////////////////////////////////////////////////////

    function _checkInternallyForPaused() internal view {
        require(isPaused, "Contract is not paused");
    }

    // Public function to transfer ownership
    function transferOwnership(address newOwner) external onlyOwner {
        address oldOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }

    // Public function to pause or unpause the contract
    function setPause(bool _isPaused) external onlyOwner {
        isPaused = _isPaused;
        emit ContractPaused(isPaused);
    }

    // Public function to demonstrate usage of modifiers
    function performAction() external whenNotPaused {
        // Action to be performed when the contract is not paused
        // This function is vulnerable due to the weak modifiers
    }

    // Function to withdraw Ether from the contract (for testing vulnerabilities)
    function withdraw(address payable to, uint256 amount) external onlyOwner {
        to.transfer(amount);
    }
}
