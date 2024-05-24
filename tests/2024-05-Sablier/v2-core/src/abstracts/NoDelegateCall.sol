// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { Errors } from "../libraries/Errors.sol";

/// @title NoDelegateCall
/// @notice This contract implements logic to prevent delegate calls.
abstract contract NoDelegateCall {
    /// @dev The address of the original contract that was deployed.
    address private immutable ORIGINAL;

    /// @dev Sets the original contract address.
    constructor() {
        ORIGINAL = address(this);
    }

    /// @notice Prevents delegate calls.
    modifier noDelegateCall() {
        _preventDelegateCall();
        _;
    }

    /// @dev This function checks whether the current call is a delegate call, and reverts if it is.
    ///
    /// - A private function is used instead of inlining this logic in a modifier because Solidity copies modifiers into
    /// every function that uses them. The `ORIGINAL` address would get copied in every place the modifier is used,
    /// which would increase the contract size. By using a function instead, we can avoid this duplication of code
    /// and reduce the overall size of the contract.
    function _preventDelegateCall() private view {
        if (address(this) != ORIGINAL) {
            revert Errors.DelegateCall();
        }
    }
}
