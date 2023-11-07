// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {AccessControl} from "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
import {IERC20, SafeERC20} from "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";

contract DeprecatedOZFunctions is AccessControl {
    using SafeERC20 for IERC20;

    // Good
    function grantRole0(bytes32 role, address account) external {
        grantRole(role, account);
    }

    // Good
    function safeTransferFrom(IERC20 token, address from, address to, uint256 value) external {
        token.safeTransferFrom(from, to, value);
    }

    // Bad (deprecated_oz_functions)
    function setupRole(bytes32 role, address account) external {
        _setupRole(role, account);
    }

    // Bad (deprecated_oz_functions)
    function safeApprove(IERC20 token, address spender, uint256 value) external {
        token.safeApprove(spender, value);
    }

    // Bad (unsafe_erc20_functions)
    function approve(IERC20 token, address spender, uint256 value) external {
        token.approve(spender, value);
    }

    // Bad (unsafe_erc20_functions)
    function approveAndCheckReturnValue(IERC20 token, address spender, uint256 value) external returns (bool success) {
        require(token.approve(spender, value));
        success = token.approve(spender, value);
        if (!success) {
            revert();
        }
        return token.approve(spender, value);
    }

    // Bad (unsafe_erc20_functions)
    function transferFrom(IERC20 token, address from, address to, uint256 value) external {
        token.transferFrom(from, to, value);
    }
}
