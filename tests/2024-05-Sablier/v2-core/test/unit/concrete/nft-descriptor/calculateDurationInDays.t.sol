// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { SVGElements } from "src/libraries/SVGElements.sol";

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract CalculateDurationInDays_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function test_CalculateDurationInDays_Zero() external view {
        uint256 startTime = block.timestamp;
        uint256 endTime = startTime + 1 days - 1 seconds;
        string memory actualDurationInDays = nftDescriptorMock.calculateDurationInDays_(startTime, endTime);
        string memory expectedDurationInDays = string.concat(SVGElements.SIGN_LT, " 1 Day");
        assertEq(actualDurationInDays, expectedDurationInDays, "durationInDays");
    }

    function test_CalculateDurationInDays_One() external view {
        uint256 startTime = block.timestamp;
        uint256 endTime = startTime + 1 days;
        string memory actualDurationInDays = nftDescriptorMock.calculateDurationInDays_(startTime, endTime);
        string memory expectedDurationInDays = "1 Day";
        assertEq(actualDurationInDays, expectedDurationInDays, "durationInDays");
    }

    function test_CalculateDurationInDays_FortyTwo() external view {
        uint256 startTime = block.timestamp;
        uint256 endTime = startTime + 42 days;
        string memory actualDurationInDays = nftDescriptorMock.calculateDurationInDays_(startTime, endTime);
        string memory expectedDurationInDays = "42 Days";
        assertEq(actualDurationInDays, expectedDurationInDays, "durationInDays");
    }

    function test_CalculateDurationInDays_Leet() external view {
        uint256 startTime = block.timestamp;
        uint256 endTime = startTime + 1337 days;
        string memory actualDurationInDays = nftDescriptorMock.calculateDurationInDays_(startTime, endTime);
        string memory expectedDurationInDays = "1337 Days";
        assertEq(actualDurationInDays, expectedDurationInDays, "durationInDays");
    }

    function test_CalculateDurationInDays_TenThousand() external view {
        uint256 startTime = block.timestamp;
        uint256 endTime = startTime + 10_000 days;
        string memory actualDurationInDays = nftDescriptorMock.calculateDurationInDays_(startTime, endTime);
        string memory expectedDurationInDays = string.concat(SVGElements.SIGN_GT, " 9999 Days");
        assertEq(actualDurationInDays, expectedDurationInDays, "durationInDays");
    }

    function test_CalculateDurationInDays_Overflow() external view {
        uint256 startTime = block.timestamp;
        uint256 endTime = startTime - 1 seconds;
        string memory actualDurationInDays = nftDescriptorMock.calculateDurationInDays_(startTime, endTime);
        string memory expectedDurationInDays = string.concat(SVGElements.SIGN_GT, " 9999 Days");
        assertEq(actualDurationInDays, expectedDurationInDays, "durationInDays");
    }
}
