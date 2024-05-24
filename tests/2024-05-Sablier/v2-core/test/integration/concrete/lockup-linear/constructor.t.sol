// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { UD60x18 } from "@prb/math/src/UD60x18.sol";
import { SablierV2LockupLinear } from "src/SablierV2LockupLinear.sol";

import { LockupLinear_Integration_Concrete_Test } from "./LockupLinear.t.sol";

contract Constructor_LockupLinear_Integration_Concrete_Test is LockupLinear_Integration_Concrete_Test {
    function test_Constructor() external {
        // Expect the relevant event to be emitted.
        vm.expectEmit();
        emit TransferAdmin({ oldAdmin: address(0), newAdmin: users.admin });

        // Construct the contract.
        SablierV2LockupLinear constructedLockupLinear =
            new SablierV2LockupLinear({ initialAdmin: users.admin, initialNFTDescriptor: nftDescriptor });

        // {SablierV2Lockup.constant}
        UD60x18 actualMaxBrokerFee = constructedLockupLinear.MAX_BROKER_FEE();
        UD60x18 expectedMaxBrokerFee = UD60x18.wrap(0.1e18);
        assertEq(actualMaxBrokerFee, expectedMaxBrokerFee, "MAX_BROKER_FEE");

        // {SablierV2Lockup.constructor}
        address actualAdmin = constructedLockupLinear.admin();
        address expectedAdmin = users.admin;
        assertEq(actualAdmin, expectedAdmin, "admin");

        uint256 actualStreamId = constructedLockupLinear.nextStreamId();
        uint256 expectedStreamId = 1;
        assertEq(actualStreamId, expectedStreamId, "nextStreamId");

        address actualNFTDescriptor = address(constructedLockupLinear.nftDescriptor());
        address expectedNFTDescriptor = address(nftDescriptor);
        assertEq(actualNFTDescriptor, expectedNFTDescriptor, "nftDescriptor");
    }
}
