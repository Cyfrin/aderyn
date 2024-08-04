// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;


// COPIED FROM Slither WIKI
contract LocalVariableShadowingBug {
    uint owner;

    // BAD (owner is shadowed)
    function sensitive_function(address owner) public view {
        // ...
        require(owner == msg.sender);
    }

    // BAD (owner is shadowed)
    function alternate_sensitive_function() public view {
        address owner = msg.sender;
        // ...
        require(owner == msg.sender);
    }
}

/**********************/

contract LocalVariableShadowingBugParent {
    uint internal roll;
}


contract LocalVariableShadowingBugChild is LocalVariableShadowingBugParent {
    // BAD (roll is already declared as a variable in parent contract)
    function sensitive_function(address roll) public view {
        // ...
        require(roll == msg.sender);
    } 

    function get_roll() external view returns(uint) {
        return roll;
    }
}

