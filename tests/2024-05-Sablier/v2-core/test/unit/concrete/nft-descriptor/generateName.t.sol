// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract GenerateName_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function gn(string memory sablierModel, string memory streamId) internal view returns (string memory) {
        return nftDescriptorMock.generateName_(sablierModel, streamId);
    }

    function dyn(string memory streamId) internal pure returns (string memory) {
        return string.concat("Sablier V2 Lockup Dynamic #", streamId);
    }

    function lin(string memory streamId) internal pure returns (string memory) {
        return string.concat("Sablier V2 Lockup Linear #", streamId);
    }

    function test_GenerateName_Empty() external view {
        assertEq(gn("", ""), "Sablier V2  #", "metadata name");
        assertEq(gn("A", ""), "Sablier V2 A #", "metadata name");
        assertEq(gn("", "1"), "Sablier V2  #1", "metadata name");
    }

    function test_GenerateName() external view {
        assertEq(gn("Lockup Dynamic", "1"), dyn("1"), "metadata name");
        assertEq(gn("Lockup Dynamic", "42"), dyn("42"), "metadata name");
        assertEq(gn("Lockup Dynamic", "1337"), dyn("1337"), "metadata name");
        assertEq(gn("Lockup Dynamic", "1234567"), dyn("1234567"), "metadata name");
        assertEq(gn("Lockup Dynamic", "123456890"), dyn("123456890"), "metadata name");
        assertEq(gn("Lockup Linear", "1"), lin("1"), "metadata name");
        assertEq(gn("Lockup Linear", "42"), lin("42"), "metadata name");
        assertEq(gn("Lockup Linear", "1337"), lin("1337"), "metadata name");
        assertEq(gn("Lockup Linear", "1234567"), lin("1234567"), "metadata name");
        assertEq(gn("Lockup Linear", "123456890"), lin("123456890"), "metadata name");
    }
}
