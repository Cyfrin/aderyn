// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract StringifyPercentage_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function sp(uint256 percentage) internal view returns (string memory) {
        return nftDescriptorMock.stringifyPercentage_(percentage);
    }

    function test_StringifyPercentage_NoFractionalPart() external view {
        assertEq(sp(0), "0%", "percentage mismatch");
        assertEq(sp(100), "1%", "percentage mismatch");
        assertEq(sp(300), "3%", "percentage mismatch");
        assertEq(sp(1000), "10%", "percentage mismatch");
        assertEq(sp(4200), "42%", "percentage mismatch");
        assertEq(sp(10_000), "100%", "percentage mismatch");
    }

    function test_StringifyPercentage_FractionalPart() external view {
        assertEq(sp(1), "0.01%", "percentage mismatch");
        assertEq(sp(42), "0.42%", "percentage mismatch");
        assertEq(sp(314), "3.14%", "percentage mismatch");
        assertEq(sp(2064), "20.64%", "percentage mismatch");
        assertEq(sp(6588), "65.88%", "percentage mismatch");
        assertEq(sp(9999), "99.99%", "percentage mismatch");
    }
}
