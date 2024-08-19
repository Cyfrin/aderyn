// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;


// COPIED FROM Slither WIKI
contract LocalVariableShadowBug {
    uint owner;

    // BAD (owner is shadowed)
    function sensitiveFunction(address owner) public view {
        // ...
        require(owner == msg.sender);
    }

    // BAD (owner is shadowed)
    function alternateSensitiveFunction() public view {
        address owner = msg.sender;
        // ...
        require(owner == msg.sender);
    }
}


contract LocalVariableShadowBugParent {
    uint internal roll;
}


contract LocalVariableShadowingBugChild is LocalVariableShadowBugParent {
    // BAD (roll is already declared as a variable in parent contract)
    function sensitiveFfunction(address roll) public view {
        // ...
        require(roll == msg.sender);
    } 

    // This is okay because even though roll is shadowing, it's inside event definition
    event RollChanged(uint indexed roll); 

    function getRoll() external view returns(uint) {
        return roll;
    }
}

