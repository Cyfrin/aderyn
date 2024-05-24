// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup } from "src/types/DataTypes.sol";

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract StringifyStatus_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function test_StringifyStatus() external view {
        assertEq(nftDescriptorMock.stringifyStatus_(Lockup.Status.DEPLETED), "Depleted", "depleted status mismatch");
        assertEq(nftDescriptorMock.stringifyStatus_(Lockup.Status.CANCELED), "Canceled", "canceled status mismatch");
        assertEq(nftDescriptorMock.stringifyStatus_(Lockup.Status.STREAMING), "Streaming", "streaming status mismatch");
        assertEq(nftDescriptorMock.stringifyStatus_(Lockup.Status.SETTLED), "Settled", "settled status mismatch");
        assertEq(nftDescriptorMock.stringifyStatus_(Lockup.Status.PENDING), "Pending", "pending status mismatch");
    }
}
