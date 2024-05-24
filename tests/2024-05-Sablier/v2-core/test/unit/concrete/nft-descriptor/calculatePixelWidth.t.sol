// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract CalculatePixelWidth_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    uint256 internal constant CHAR_WIDTH_LARGE = 16;
    uint256 internal constant CHAR_WIDTH_SMALL = 13;

    function cpw(string memory text, bool largeFont) internal view returns (uint256) {
        return nftDescriptorMock.calculatePixelWidth_(text, largeFont);
    }

    function large(string memory text) internal pure returns (uint256) {
        return bytes(text).length * CHAR_WIDTH_LARGE;
    }

    function small(string memory text) internal pure returns (uint256) {
        return bytes(text).length * CHAR_WIDTH_SMALL;
    }

    function test_CalculatePixelWidth_EmptyString() external view {
        uint256 actualWidth = cpw({ text: "", largeFont: false });
        uint256 expectedWidth = 0;
        assertEq(actualWidth, expectedWidth, "width");
    }

    function test_CalculatePixelWidth_Caption() external view {
        bool largeFont = false;
        assertEq(cpw("Progress", largeFont), small("Progress"), "pixel width");
        assertEq(cpw("Status", largeFont), small("Status"), "pixel width");
        assertEq(cpw("Streamed", largeFont), small("Streamed"), "pixel width");
        assertEq(cpw("Duration", largeFont), small("Duration"), "pixel width");
    }

    function test_CalculatePixelWidth_Progress() external view {
        bool largeFont = true;
        assertEq(cpw("0%", largeFont), large("0%"), "pixel width");
        assertEq(cpw("0.01%", largeFont), large("0.01%"), "pixel width");
        assertEq(cpw("0.42%", largeFont), large("0.42%"), "pixel width");
        assertEq(cpw("1%", largeFont), large("1%"), "pixel width");
        assertEq(cpw("3.14%", largeFont), large("3.14%"), "pixel width");
        assertEq(cpw("20.64%", largeFont), large("20.64%"), "pixel width");
        assertEq(cpw("99.99%", largeFont), large("99.99%"), "pixel width");
        assertEq(cpw("100%", largeFont), large("100%"), "pixel width");
    }

    function test_CalculatePixelWidth_Status() external view {
        bool largeFont = true;
        assertEq(cpw("Depleted", largeFont), large("Depleted"), "pixel width");
        assertEq(cpw("Canceled", largeFont), large("Canceled"), "pixel width");
        assertEq(cpw("Streaming", largeFont), large("Streaming"), "pixel width");
        assertEq(cpw("Settled", largeFont), large("Settled"), "pixel width");
        assertEq(cpw("Pending", largeFont), large("Pending"), "pixel width");
    }

    function test_CalculatePixelWidth_Streamed() external view {
        bool largeFont = true;
        assertEq(cpw("&lt; 1", largeFont), large("< 1"), "pixel width");
        assertEq(cpw("&#8805; 42.73K", largeFont), large(" 42.73K") + CHAR_WIDTH_LARGE, "pixel width");
        assertEq(cpw("&#8805; 1.23M", largeFont), large(" 1.23M") + CHAR_WIDTH_LARGE, "pixel width");
        assertEq(cpw("&#8805; 8.10B", largeFont), large(" 8.10B") + CHAR_WIDTH_LARGE, "pixel width");
        assertEq(cpw("&#8805; 4.91T", largeFont), large(" 4.91T") + CHAR_WIDTH_LARGE, "pixel width");
        assertEq(cpw("&#8805; 999.99T", largeFont), large(" 999.99T") + CHAR_WIDTH_LARGE, "pixel width");
    }

    function test_CalculatePixelWidth_Duration() external view {
        bool largeFont = true;
        assertEq(cpw("&lt; 1 Day", largeFont), large("< 1 Day"), "pixel width");
        assertEq(cpw("1 Day", largeFont), large("1 Day"), "pixel width");
        assertEq(cpw("1337 Days", largeFont), large("1337 Days"), "pixel width");
        assertEq(cpw("9999 Days", largeFont), large("9999 Days"), "pixel width");
        assertEq(cpw("&gt; 9999 Days", largeFont), large("> 9999 Days"), "pixel width");
    }
}
