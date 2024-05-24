// SPDX-License-Identifier: UNLICENSED
// solhint-disable max-line-length,quotes
pragma solidity >=0.8.22 <0.9.0;

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract GenerateAttributes_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function test_GenerateAttributes_Empty() external view {
        string memory actualAttributes = nftDescriptorMock.generateAttributes_("", "", "");
        string memory expectedAttributes =
            '[{"trait_type":"Asset","value":""},{"trait_type":"Sender","value":""},{"trait_type":"Status","value":""}]';
        assertEq(actualAttributes, expectedAttributes, "metadata attributes");
    }

    function test_GenerateAttributes() external view {
        string memory actualAttributes =
            nftDescriptorMock.generateAttributes_("DAI", "0x50725493D337CdC4e381f658e10d29d128BD6927", "Streaming");
        string memory expectedAttributes =
            '[{"trait_type":"Asset","value":"DAI"},{"trait_type":"Sender","value":"0x50725493D337CdC4e381f658e10d29d128BD6927"},{"trait_type":"Status","value":"Streaming"}]';
        assertEq(actualAttributes, expectedAttributes, "metadata attributes");
    }
}
