// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { SVGElements } from "src/libraries/SVGElements.sol";

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract StringifyCardType_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function test_StringifyCardType() external view {
        assertEq(nftDescriptorMock.stringifyCardType_(SVGElements.CardType.PROGRESS), "Progress");
        assertEq(nftDescriptorMock.stringifyCardType_(SVGElements.CardType.STATUS), "Status");
        assertEq(nftDescriptorMock.stringifyCardType_(SVGElements.CardType.AMOUNT), "Amount");
        assertEq(nftDescriptorMock.stringifyCardType_(SVGElements.CardType.DURATION), "Duration");
    }
}
