// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

/// @title IAdminable
/// @notice Contract module that provides a basic access control mechanism, with an admin that can be
/// granted exclusive access to specific functions. The inheriting contract must set the initial admin
/// in the constructor.
interface IAdminable {
    /*//////////////////////////////////////////////////////////////////////////
                                       EVENTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Emitted when the admin is transferred.
    /// @param oldAdmin The address of the old admin.
    /// @param newAdmin The address of the new admin.
    event TransferAdmin(address indexed oldAdmin, address indexed newAdmin);

    /*//////////////////////////////////////////////////////////////////////////
                                 CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice The address of the admin account or contract.
    function admin() external view returns (address);

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Transfers the contract admin to a new address.
    ///
    /// @dev Notes:
    /// - Does not revert if the admin is the same.
    /// - This function can potentially leave the contract without an admin, thereby removing any
    /// functionality that is only available to the admin.
    ///
    /// Requirements:
    /// - `msg.sender` must be the contract admin.
    ///
    /// @param newAdmin The address of the new admin.
    function transferAdmin(address newAdmin) external;
}
