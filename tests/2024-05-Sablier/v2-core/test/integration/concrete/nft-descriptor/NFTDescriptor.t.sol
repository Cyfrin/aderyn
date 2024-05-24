// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Integration_Test } from "../../Integration.t.sol";
import { NFTDescriptorMock } from "../../../mocks/NFTDescriptorMock.sol";

abstract contract NFTDescriptor_Integration_Concrete_Test is Integration_Test {
    NFTDescriptorMock internal nftDescriptorMock;

    function setUp() public virtual override {
        Integration_Test.setUp();
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
