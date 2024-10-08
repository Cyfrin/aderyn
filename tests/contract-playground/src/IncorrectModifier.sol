// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract IncorrectModifierContract {
    uint256 public constant USEME = 100;
    address public immutable owner;
    bool public immutable isPaused;

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
        if (streamId > USEME) {
            revert StreamError();
        }
        _;
    }

    modifier notNull2GOOD(uint256 streamId) {
        if (streamId > USEME) {
            revert();
        }
        _;
    }

    modifier simpleGOOD() {
        _;
    }

    // Modifier to check if the caller is the owner
    modifier onlyOwnerGOOD() {
        require(msg.sender == owner, "Not the owner");
        _; // Continue execution if the condition is met
    }

    // Modifier to check if the contract is not paused
    modifier whenNotPausedGOOD() {
        if (msg.sender != owner) {
            require(!isPaused, "Contract is paused");
            return;
        }
        _;
    }

    modifier whenPausedGOOD() {
        if (msg.sender != owner) {
            _checkInternallyForPaused();
            return;
        }
        _;
    }

    modifier revertsButWithoutUsingRevertKeyword() {
        if (msg.sender != owner) {
            payable(owner).transfer(USEME);
            return;
        }
        _;
    }

    //////////////////////////// FUNCTIONS //////////////////////////////////////////////////////////////////////

    function _checkInternallyForPaused() internal view {
        require(isPaused, "Contract is not paused");
    }
}
