// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { UD60x18 } from "@prb/math/src/UD60x18.sol";
import { SablierV2LockupDynamic } from "src/SablierV2LockupDynamic.sol";

import { LockupDynamic_Integration_Concrete_Test } from "./LockupDynamic.t.sol";

contract Constructor_LockupDynamic_Integration_Concrete_Test is LockupDynamic_Integration_Concrete_Test {
    function test_Constructor() external {
        // Expect the relevant event to be emitted.
        vm.expectEmit();
        emit TransferAdmin({ oldAdmin: address(0), newAdmin: users.admin });

        // Construct the contract.
        SablierV2LockupDynamic constructedLockupDynamic = new SablierV2LockupDynamic({
            initialAdmin: users.admin,
            initialNFTDescriptor: nftDescriptor,
            maxSegmentCount: defaults.MAX_SEGMENT_COUNT()
        });

        // {SablierV2Lockup.constant}
        UD60x18 actualMaxBrokerFee = constructedLockupDynamic.MAX_BROKER_FEE();
        UD60x18 expectedMaxBrokerFee = UD60x18.wrap(0.1e18);
        assertEq(actualMaxBrokerFee, expectedMaxBrokerFee, "MAX_BROKER_FEE");

        // {SablierV2Lockup.constructor}
        address actualAdmin = constructedLockupDynamic.admin();
        address expectedAdmin = users.admin;
        assertEq(actualAdmin, expectedAdmin, "admin");

        uint256 actualStreamId = constructedLockupDynamic.nextStreamId();
        uint256 expectedStreamId = 1;
        assertEq(actualStreamId, expectedStreamId, "nextStreamId");

        address actualNFTDescriptor = address(constructedLockupDynamic.nftDescriptor());
        address expectedNFTDescriptor = address(nftDescriptor);
        assertEq(actualNFTDescriptor, expectedNFTDescriptor, "nftDescriptor");

        // {SablierV2LockupDynamic.constructor}
        uint256 actualMaxSegmentCount = constructedLockupDynamic.MAX_SEGMENT_COUNT();
        uint256 expectedMaxSegmentCount = defaults.MAX_SEGMENT_COUNT();
        assertEq(actualMaxSegmentCount, expectedMaxSegmentCount, "MAX_SEGMENT_COUNT");
    }
}
