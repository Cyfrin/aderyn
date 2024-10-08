// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract IncorrectModifierContract {
    address public owner;
    bool public isPaused;

    error StreamError();

    // Constructor to set the initial owner
    constructor() {
        owner = msg.sender;
        isPaused = false;
    }

    /*/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        BAD modifiers
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    // BAD
    modifier modifierIfRetBAD() {
        if (!isPaused) {
            return;
        }
        _;
    }

    // BAD
    modifier modifierIfPlaceBAD() {
        if (isPaused) {
            _;
        }
    }

    /*/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        GOOD modifiers
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    modifier notNullGOOD(uint256 streamId) {
        if (streamId > 100) {
            revert StreamError();
        }
        _;
    }

    modifier simpleGOOD() {
        _;
    }

    // // Modifier to check if the caller is the owner
    // modifier onlyOwnerGOOD() {
    //     require(msg.sender == owner, "Not the owner");
    //     _; // Continue execution if the condition is met
    // }
    //
    // // Modifier to check if the contract is not paused
    // modifier whenNotPausedGOOD() {
    //     require(!isPaused, "Contract is paused");
    //     _; // Continue execution if the condition is met
    // }
    //
    // // Modifier to ensure the contract is in a specific state (example: not paused)
    // modifier whenPausedGOOD() {
    //     _checkInternallyForPaused();
    //     _; // Continue execution if the condition is met
    // }
    //
    // modifier revertsButWithoutUsingRevertKeyword() {
    //     payable(address(0x3000)).transfer(130);
    //     _; // Continue execution if the condition is met
    // }

    //////////////////////////// FUNCTIONS //////////////////////////////////////////////////////////////////////

    function _checkInternallyForPaused() internal view {
        require(isPaused, "Contract is not paused");
    }
}
