// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22;

import { IERC721Metadata } from "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";

import { NFTSVG } from "../../src/libraries/NFTSVG.sol";
import { SVGElements } from "../../src/libraries/SVGElements.sol";
import { Lockup } from "../../src/types/DataTypes.sol";
import { SablierV2NFTDescriptor } from "../../src/SablierV2NFTDescriptor.sol";

/// @dev This mock is needed for:
/// - Running the tests against the `--via-ir` precompiles
/// - Testing reverts: https://github.com/foundry-rs/foundry/issues/864
contract NFTDescriptorMock is SablierV2NFTDescriptor {
    function abbreviateAmount_(uint256 amount, uint256 decimals) external pure returns (string memory) {
        return abbreviateAmount(amount, decimals);
    }

    function calculateDurationInDays_(uint256 startTime, uint256 endTime) external pure returns (string memory) {
        return calculateDurationInDays(startTime, endTime);
    }

    function calculatePixelWidth_(string memory text, bool largeFont) external pure returns (uint256) {
        return SVGElements.calculatePixelWidth(text, largeFont);
    }

    function calculateStreamedPercentage_(
        uint128 streamedAmount,
        uint128 depositedAmount
    )
        external
        pure
        returns (uint256)
    {
        return calculateStreamedPercentage(streamedAmount, depositedAmount);
    }

    function generateAccentColor_(address sablier, uint256 streamId) external view returns (string memory) {
        return generateAccentColor(sablier, streamId);
    }

    function generateAttributes_(
        string memory assetSymbol,
        string memory sender,
        string memory status
    )
        external
        pure
        returns (string memory)
    {
        return generateAttributes(assetSymbol, sender, status);
    }

    function generateDescription_(
        string memory sablierModel,
        string memory assetSymbol,
        string memory sablierAddress,
        string memory assetAddress,
        string memory streamId,
        bool isTransferable
    )
        external
        pure
        returns (string memory)
    {
        return generateDescription(sablierModel, assetSymbol, sablierAddress, assetAddress, streamId, isTransferable);
    }

    function generateName_(string memory sablierModel, string memory streamId) external pure returns (string memory) {
        return generateName(sablierModel, streamId);
    }

    function generateSVG_(NFTSVG.SVGParams memory params) external pure returns (string memory) {
        return NFTSVG.generateSVG(params);
    }

    function hourglass_(string memory status) external pure returns (string memory) {
        return SVGElements.hourglass(status);
    }

    function mapSymbol_(IERC721Metadata nft) external view returns (string memory) {
        return mapSymbol(nft);
    }

    function safeAssetDecimals_(address asset) external view returns (uint8) {
        return safeAssetDecimals(asset);
    }

    function safeAssetSymbol_(address asset) external view returns (string memory) {
        return safeAssetSymbol(asset);
    }

    function stringifyCardType_(SVGElements.CardType cardType) external pure returns (string memory) {
        return SVGElements.stringifyCardType(cardType);
    }

    function stringifyFractionalAmount_(uint256 fractionalAmount) external pure returns (string memory) {
        return stringifyFractionalAmount(fractionalAmount);
    }

    function stringifyPercentage_(uint256 percentage) external pure returns (string memory) {
        return stringifyPercentage(percentage);
    }

    function stringifyStatus_(Lockup.Status status) external pure returns (string memory) {
        return stringifyStatus(status);
    }
}
