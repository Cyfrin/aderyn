// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract CalculateStreamedPercentage_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function test_CalculateStreamedPercentage_Zero() external view {
        uint256 actualStreamedPercentage =
            nftDescriptorMock.calculateStreamedPercentage_({ streamedAmount: 0, depositedAmount: 1337e18 });
        uint256 expectedStreamedPercentage = 0;
        assertEq(actualStreamedPercentage, expectedStreamedPercentage, "streamedPercentage");
    }

    function test_CalculateStreamedPercentage_Streaming() external view {
        uint256 actualStreamedPercentage =
            nftDescriptorMock.calculateStreamedPercentage_({ streamedAmount: 100e18, depositedAmount: 400e18 });
        uint256 expectedStreamedPercentage = 2500;
        assertEq(actualStreamedPercentage, expectedStreamedPercentage, "streamedPercentage");
    }

    function test_CalculateStreamedPercentage_Settled() external view {
        uint256 actualStreamedPercentage =
            nftDescriptorMock.calculateStreamedPercentage_({ streamedAmount: 1337e18, depositedAmount: 1337e18 });
        uint256 expectedStreamedPercentage = 10_000;
        assertEq(actualStreamedPercentage, expectedStreamedPercentage, "streamedPercentage");
    }
}
