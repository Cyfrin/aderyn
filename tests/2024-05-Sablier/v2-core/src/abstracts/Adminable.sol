// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IAdminable } from "../interfaces/IAdminable.sol";
import { Errors } from "../libraries/Errors.sol";

/// @title Adminable
/// @notice See the documentation in {IAdminable}.
abstract contract Adminable is IAdminable {
    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc IAdminable
    address public override admin;

    /*//////////////////////////////////////////////////////////////////////////
                                      MODIFIERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Reverts if called by any account other than the admin.
    modifier onlyAdmin() {
        if (admin != msg.sender) {
            revert Errors.CallerNotAdmin({ admin: admin, caller: msg.sender });
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////////////////
                         USER-FACING NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc IAdminable
    function transferAdmin(address newAdmin) public virtual override onlyAdmin {
        // Effect: update the admin.
        admin = newAdmin;

        // Log the transfer of the admin.
        emit IAdminable.TransferAdmin({ oldAdmin: msg.sender, newAdmin: newAdmin });
    }
}
