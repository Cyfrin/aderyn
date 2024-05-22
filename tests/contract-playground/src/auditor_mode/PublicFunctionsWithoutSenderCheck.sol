// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Ownable} from "../../lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import {AccessControl} from "../../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";

// FunctionDefinitions where implemented is true

contract OwnableExamples is Ownable {
    // Do not capture
    function onlyOwnerFunction() external onlyOwner {
        // Function body
    }

    // Do not capture
    function onlyOwnerManualRequireCheck() public view {
        require(msg.sender == owner(), "Caller is not the owner");
        // Function body
    }

    // Do not capture
    function onlyOwnerManualIfRevertCheck() public view {
        if (msg.sender != owner()) {
            revert("Caller is not the owner");
        }
        // Function body
    }

    // Capture
    function transferOwnershipFunction(address newOwner) external {
        // Function body
    }
}

contract AccessControlExamples is AccessControl {
    bytes32 public constant ROLE = keccak256("ROLE");
    // Do not capture
    function onlyRoleFunction() external onlyRole(ROLE) {
        // Function body
    }

    // Do not capture
    // False positive - but this is OK, it will force the auditor to check
    // that the `hasRole` return value is used correctly
    function onlyRoleManualRequireCheck() public view {
        require(hasRole(ROLE, msg.sender), "Caller is not the owner");
        // Function body
    }

    // Do not capture
    // False positive - but this is OK, it will force the auditor to check
    // that the `hasRole` return value is used correctly
    function onlyRoleManualIfRevertCheck() public view {
        if (!hasRole(ROLE, msg.sender)) {
            revert("Caller is not the owner");
        }
        // Function body
    }

    // Capture
    function grantRoleFunction(address account) external {
        // Function body
    }
}


contract ManualCheckExamples {
    address public owner;

    // Do not capture
    function onlyOwnerManualCheck() public view {
        require(msg.sender == owner, "Caller is not the owner");
        // Function body
    }

    // Do not capture
    function onlyOwnerManualIfRevertCheck() public view {
        if (msg.sender != owner) {
            revert("Caller is not the owner");
        }
        // Function body
    }

    // Capture
    function transferOwnershipFunction(address newOwner) external {
        // Function body
    }
}