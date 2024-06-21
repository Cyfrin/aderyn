// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.19;

abstract contract AbstractContract {
    address public admin;
    function transferAdmin(address newAdmin) public virtual {
        if (admin != msg.sender) {
            revert("CallerNotAdmin");
        }
        require(newAdmin != address(0), "InvalidAdmin");
        admin = newAdmin;
    }
}