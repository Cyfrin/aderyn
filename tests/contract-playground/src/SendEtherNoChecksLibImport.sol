// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {Ownable} from "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import {AccessControl} from "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";

contract SendEtherCheckOZOnlyOWner is Ownable, AccessControl {

    function callAndSendNativeEth(address x) internal {
        (bool success,) = x.call{value: 10}("calldata");
        if (!success) {
            revert();
        }
    }

    function send(address x) external onlyOwner {
        callAndSendNativeEth(x);
    }

    function sendWithRole(address x) external onlyRole(DEFAULT_ADMIN_ROLE) {
        callAndSendNativeEth(x);
    }
}
