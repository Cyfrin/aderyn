// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract StringifyFractionalAmount_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function sfa(uint256 amount) internal view returns (string memory) {
        return nftDescriptorMock.stringifyFractionalAmount_(amount);
    }

    function test_FractionalAmount_Zero() external view {
        assertEq(sfa(0), "", "fractional part mismatch");
    }

    function test_FractionalAmount_LeadingZero() external view {
        assertEq(sfa(1), ".01", "fractional part mismatch");
        assertEq(sfa(5), ".05", "fractional part mismatch");
        assertEq(sfa(9), ".09", "fractional part mismatch");
    }

    function test_FractionalAmount_NoLeadingZero() external view {
        assertEq(sfa(10), ".10", "fractional part mismatch");
        assertEq(sfa(12), ".12", "fractional part mismatch");
        assertEq(sfa(33), ".33", "fractional part mismatch");
        assertEq(sfa(42), ".42", "fractional part mismatch");
        assertEq(sfa(70), ".70", "fractional part mismatch");
        assertEq(sfa(99), ".99", "fractional part mismatch");
    }
}
