// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

library ErrorLibrary {
    error UnusedLibraryError();
    error UsedInARequire();
    error LibraryError();
}

contract UnusedError {
    uint256 public number = 0;
    
    error CannotRenounceWhilePaused(address account);
    error UnusedError1(address account);

    function perform() external {
       number++;
    }

    function goodError() external view {
        revert CannotRenounceWhilePaused(msg.sender);
    }

    function goodLibraryError() external pure {
        revert ErrorLibrary.LibraryError();
    }

    function goodRequire() external pure {
        require(false, ErrorLibrary.UsedInARequire());
    }
}
