// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract VulnerableContract {
    address public owner;
    mapping(address => bool) public authorizedUsers;

    event Authenticated(address indexed user);
    event UnauthorizedAccess(address indexed user);

    constructor() {
        owner = msg.sender;
    }

    // GOOD
    function authorizeUser(address _user) external {
        require(msg.sender == owner, "Only owner can authorize users");
        authorizedUsers[_user] = true;
    }

    // GOOD
    function revokeUser(address _user) external {
        require(msg.sender == owner, "Only owner can revoke users");
        authorizedUsers[_user] = false;
    }

    // GOOD
    function edgeCase() external {
        // This uses both `tx.origin` as well as `msg.sender` so, it's OK (Same heuristic as slither)
        require(
            tx.origin == msg.sender && msg.sender == owner,
            "Not authorized to perform this action"
        );
        emit Authenticated(msg.sender);
    }

    // BAD
    function secureAction() external {
        // Vulnerable use of tx.origin
        if (tx.origin == owner) {
            emit Authenticated(msg.sender);
        } else {
            emit UnauthorizedAccess(msg.sender);
            revert("Not authorized");
        }
    }

    // BAD
    function checkAuthorization() external view returns (bool) {
        // Vulnerable use of tx.origin
        if (tx.origin == owner || authorizedUsers[tx.origin]) {
            return true;
        }
        return false;
    }

    // BAD
    function performAction() external {
        require(tx.origin == owner, "Not authorized to perform this action");
        emit Authenticated(msg.sender);
    }
}
