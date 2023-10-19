// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {AccessControl} from "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
import {IERC20, SafeERC20} from "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";

contract DeprecatedOZFunctions is AccessControl {
    using SafeERC20 for IERC20;

    function grantRoleSuccess(bytes32 role, address account) external {
        grantRole(role, account);
    }

    function setupRoleFailure(bytes32 role, address account) external {
        _setupRole(role, account);
    }

    function approveSuccess(IERC20 token, address spender, uint256 value) external {
        token.approve(spender, value);
    }

    function safeApproveFailure(IERC20 token, address spender, uint256 value) external {
        token.safeApprove(spender, value);
    }
}