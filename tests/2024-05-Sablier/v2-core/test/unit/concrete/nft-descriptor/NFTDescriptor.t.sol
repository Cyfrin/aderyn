// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { SablierV2NFTDescriptor } from "src/SablierV2NFTDescriptor.sol";

import { NFTDescriptorMock } from "../../../mocks/NFTDescriptorMock.sol";
import { Base_Test } from "../../../Base.t.sol";

contract NFTDescriptor_Unit_Concrete_Test is Base_Test, SablierV2NFTDescriptor {
    NFTDescriptorMock internal nftDescriptorMock;

    function setUp() public virtual override {
        Base_Test.setUp();
        deployConditionally();
    }

    /// @dev Conditionally deploys {NFTDescriptorMock} normally or from a source precompiled with `--via-ir`.
    function deployConditionally() internal {
        if (!isTestOptimizedProfile()) {
            nftDescriptorMock = new NFTDescriptorMock();
        } else {
            nftDescriptorMock =
                NFTDescriptorMock(deployCode("out-optimized/NFTDescriptorMock.sol/NFTDescriptorMock.json"));
        }
        vm.label({ account: address(nftDescriptorMock), newLabel: "NFTDescriptorMock" });
    }
}
