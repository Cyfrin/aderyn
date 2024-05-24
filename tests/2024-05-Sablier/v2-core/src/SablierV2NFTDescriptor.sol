// SPDX-License-Identifier: GPL-3.0-or-later
// solhint-disable max-line-length,quotes
pragma solidity >=0.8.22;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { IERC721Metadata } from "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";
import { Base64 } from "@openzeppelin/contracts/utils/Base64.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

import { ISablierV2Lockup } from "./interfaces/ISablierV2Lockup.sol";
import { ISablierV2NFTDescriptor } from "./interfaces/ISablierV2NFTDescriptor.sol";
import { Lockup } from "./types/DataTypes.sol";

import { Errors } from "./libraries/Errors.sol";
import { NFTSVG } from "./libraries/NFTSVG.sol";
import { SVGElements } from "./libraries/SVGElements.sol";

/// @title SablierV2NFTDescriptor
/// @notice See the documentation in {ISablierV2NFTDescriptor}.
contract SablierV2NFTDescriptor is ISablierV2NFTDescriptor {
    using Strings for address;
    using Strings for string;
    using Strings for uint256;

    /*//////////////////////////////////////////////////////////////////////////
                           USER-FACING CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Needed to avoid Stack Too Deep.
    struct TokenURIVars {
        address asset;
        string assetSymbol;
        uint128 depositedAmount;
        bool isTransferable;
        string json;
        bytes returnData;
        ISablierV2Lockup sablier;
        string sablierModel;
        string sablierStringified;
        string status;
        string svg;
        uint256 streamedPercentage;
        bool success;
    }

    /// @inheritdoc ISablierV2NFTDescriptor
    function tokenURI(IERC721Metadata sablier, uint256 streamId) external view override returns (string memory uri) {
        TokenURIVars memory vars;

        // Load the contracts.
        vars.sablier = ISablierV2Lockup(address(sablier));
        vars.sablierModel = mapSymbol(sablier);
        vars.sablierStringified = address(sablier).toHexString();
        vars.asset = address(vars.sablier.getAsset(streamId));
        vars.assetSymbol = safeAssetSymbol(vars.asset);
        vars.depositedAmount = vars.sablier.getDepositedAmount(streamId);

        // Load the stream's data.
        vars.status = stringifyStatus(vars.sablier.statusOf(streamId));
        vars.streamedPercentage = calculateStreamedPercentage({
            streamedAmount: vars.sablier.streamedAmountOf(streamId),
            depositedAmount: vars.depositedAmount
        });

        // Generate the SVG.
        vars.svg = NFTSVG.generateSVG(
            NFTSVG.SVGParams({
                accentColor: generateAccentColor(address(sablier), streamId),
                amount: abbreviateAmount({ amount: vars.depositedAmount, decimals: safeAssetDecimals(vars.asset) }),
                assetAddress: vars.asset.toHexString(),
                assetSymbol: vars.assetSymbol,
                duration: calculateDurationInDays({
                    startTime: vars.sablier.getStartTime(streamId),
                    endTime: vars.sablier.getEndTime(streamId)
                }),
                sablierAddress: vars.sablierStringified,
                progress: stringifyPercentage(vars.streamedPercentage),
                progressNumerical: vars.streamedPercentage,
                status: vars.status,
                sablierModel: vars.sablierModel
            })
        );

        // Performs a low-level call to handle older deployments that miss the `isTransferable` function.
        (vars.success, vars.returnData) =
            address(vars.sablier).staticcall(abi.encodeCall(ISablierV2Lockup.isTransferable, (streamId)));

        // When the call has failed, the stream NFT is assumed to be transferable.
        vars.isTransferable = vars.success ? abi.decode(vars.returnData, (bool)) : true;

        // Generate the JSON metadata.
        vars.json = string.concat(
            '{"attributes":',
            generateAttributes({
                assetSymbol: vars.assetSymbol,
                sender: vars.sablier.getSender(streamId).toHexString(),
                status: vars.status
            }),
            ',"description":"',
            generateDescription({
                sablierModel: vars.sablierModel,
                assetSymbol: vars.assetSymbol,
                sablierStringified: vars.sablierStringified,
                assetAddress: vars.asset.toHexString(),
                streamId: streamId.toString(),
                isTransferable: vars.isTransferable
            }),
            '","external_url":"https://sablier.com","name":"',
            generateName({ sablierModel: vars.sablierModel, streamId: streamId.toString() }),
            '","image":"data:image/svg+xml;base64,',
            Base64.encode(bytes(vars.svg)),
            '"}'
        );

        // Encode the JSON metadata in Base64.
        uri = string.concat("data:application/json;base64,", Base64.encode(bytes(vars.json)));
    }

    /*//////////////////////////////////////////////////////////////////////////
                            INTERNAL CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates an abbreviated representation of the provided amount, rounded down and prefixed with ">= ".
    /// @dev The abbreviation uses these suffixes:
    /// - "K" for thousands
    /// - "M" for millions
    /// - "B" for billions
    /// - "T" for trillions
    /// For example, if the input is 1,234,567, the output is ">= 1.23M".
    /// @param amount The amount to abbreviate, denoted in units of `decimals`.
    /// @param decimals The number of decimals to assume when abbreviating the amount.
    /// @return abbreviation The abbreviated representation of the provided amount, as a string.
    function abbreviateAmount(uint256 amount, uint256 decimals) internal pure returns (string memory) {
        if (amount == 0) {
            return "0";
        }

        uint256 truncatedAmount;
        unchecked {
            truncatedAmount = decimals == 0 ? amount : amount / 10 ** decimals;
        }

        // Return dummy values when the truncated amount is either very small or very big.
        if (truncatedAmount < 1) {
            return string.concat(SVGElements.SIGN_LT, " 1");
        } else if (truncatedAmount >= 1e15) {
            return string.concat(SVGElements.SIGN_GT, " 999.99T");
        }

        string[5] memory suffixes = ["", "K", "M", "B", "T"];
        uint256 fractionalAmount;
        uint256 suffixIndex = 0;

        // Truncate repeatedly until the amount is less than 1000.
        unchecked {
            while (truncatedAmount >= 1000) {
                fractionalAmount = (truncatedAmount / 10) % 100; // keep the first two digits after the decimal point
                truncatedAmount /= 1000;
                suffixIndex += 1;
            }
        }

        // Concatenate the calculated parts to form the final string.
        string memory prefix = string.concat(SVGElements.SIGN_GE, " ");
        string memory wholePart = truncatedAmount.toString();
        string memory fractionalPart = stringifyFractionalAmount(fractionalAmount);
        return string.concat(prefix, wholePart, fractionalPart, suffixes[suffixIndex]);
    }

    /// @notice Calculates the stream's duration in days, rounding down.
    function calculateDurationInDays(uint256 startTime, uint256 endTime) internal pure returns (string memory) {
        uint256 durationInDays;
        unchecked {
            durationInDays = (endTime - startTime) / 1 days;
        }

        // Return dummy values when the duration is either very small or very big.
        if (durationInDays == 0) {
            return string.concat(SVGElements.SIGN_LT, " 1 Day");
        } else if (durationInDays > 9999) {
            return string.concat(SVGElements.SIGN_GT, " 9999 Days");
        }

        string memory suffix = durationInDays == 1 ? " Day" : " Days";
        return string.concat(durationInDays.toString(), suffix);
    }

    /// @notice Calculates how much of the deposited amount has been streamed so far, as a percentage with 4 implied
    /// decimals.
    function calculateStreamedPercentage(
        uint128 streamedAmount,
        uint128 depositedAmount
    )
        internal
        pure
        returns (uint256)
    {
        // This cannot overflow because both inputs are uint128s, and zero deposit amounts are not allowed in Sablier.
        unchecked {
            return streamedAmount * 10_000 / depositedAmount;
        }
    }

    /// @notice Generates a pseudo-random HSL color by hashing together the `chainid`, the `sablier` address,
    /// and the `streamId`. This will be used as the accent color for the SVG.
    function generateAccentColor(address sablier, uint256 streamId) internal view returns (string memory) {
        // The chain ID is part of the hash so that the generated color is different across chains.
        uint256 chainId = block.chainid;

        // Hash the parameters to generate a pseudo-random bit field, which will be used as entropy.
        // | Hue     | Saturation | Lightness | -> Roles
        // | [31:16] | [15:8]     | [7:0]     | -> Bit positions
        uint32 bitField = uint32(uint256(keccak256(abi.encodePacked(chainId, sablier, streamId))));

        unchecked {
            // The hue is a degree on a color wheel, so its range is [0, 360).
            // Shifting 16 bits to the right means using the bits at positions [31:16].
            uint256 hue = (bitField >> 16) % 360;

            // The saturation is a percentage where 0% is grayscale and 100%, but here the range is bounded to [20,100]
            // to make the colors more lively.
            // Shifting 8 bits to the right and applying an 8-bit mask means using the bits at positions [15:8].
            uint256 saturation = ((bitField >> 8) & 0xFF) % 80 + 20;

            // The lightness is typically a percentage between 0% (black) and 100% (white), but here the range
            // is bounded to [30,100] to avoid dark colors.
            // Applying an 8-bit mask means using the bits at positions [7:0].
            uint256 lightness = (bitField & 0xFF) % 70 + 30;

            // Finally, concatenate the HSL values to form an SVG color string.
            return string.concat("hsl(", hue.toString(), ",", saturation.toString(), "%,", lightness.toString(), "%)");
        }
    }

    /// @notice Generates an array of JSON objects that represent the NFT's attributes:
    /// - Asset symbol
    /// - Sender address
    /// - Status
    /// @dev These attributes are useful for filtering and sorting the NFTs.
    function generateAttributes(
        string memory assetSymbol,
        string memory sender,
        string memory status
    )
        internal
        pure
        returns (string memory)
    {
        return string.concat(
            '[{"trait_type":"Asset","value":"',
            assetSymbol,
            '"},{"trait_type":"Sender","value":"',
            sender,
            '"},{"trait_type":"Status","value":"',
            status,
            '"}]'
        );
    }

    /// @notice Generates a string with the NFT's JSON metadata description, which provides a high-level overview.
    function generateDescription(
        string memory sablierModel,
        string memory assetSymbol,
        string memory sablierStringified,
        string memory assetAddress,
        string memory streamId,
        bool isTransferable
    )
        internal
        pure
        returns (string memory)
    {
        // Depending on the transferability of the NFT, declare the relevant information.
        string memory info = isTransferable
            ?
            unicode"⚠️ WARNING: Transferring the NFT makes the new owner the recipient of the stream. The funds are not automatically withdrawn for the previous recipient."
            : unicode"❕INFO: This NFT is non-transferable. It cannot be sold or transferred to another account.";

        return string.concat(
            "This NFT represents a payment stream in a Sablier V2 ",
            sablierModel,
            " contract. The owner of this NFT can withdraw the streamed assets, which are denominated in ",
            assetSymbol,
            ".\\n\\n- Stream ID: ",
            streamId,
            "\\n- ",
            sablierModel,
            " Address: ",
            sablierStringified,
            "\\n- ",
            assetSymbol,
            " Address: ",
            assetAddress,
            "\\n\\n",
            info
        );
    }

    /// @notice Generates a string with the NFT's JSON metadata name, which is unique for each stream.
    /// @dev The `streamId` is equivalent to the ERC-721 `tokenId`.
    function generateName(string memory sablierModel, string memory streamId) internal pure returns (string memory) {
        return string.concat("Sablier V2 ", sablierModel, " #", streamId);
    }

    /// @notice Maps ERC-721 symbols to human-readable model names.
    /// @dev Reverts if the symbol is unknown.
    function mapSymbol(IERC721Metadata sablier) internal view returns (string memory) {
        string memory symbol = sablier.symbol();
        if (symbol.equal("SAB-V2-LOCKUP-LIN")) {
            return "Lockup Linear";
        } else if (symbol.equal("SAB-V2-LOCKUP-DYN")) {
            return "Lockup Dynamic";
        } else if (symbol.equal("SAB-V2-LOCKUP-TRA")) {
            return "Lockup Tranched";
        } else {
            revert Errors.SablierV2NFTDescriptor_UnknownNFT(sablier, symbol);
        }
    }

    /// @notice Retrieves the asset's decimals safely, defaulting to "0" if an error occurs.
    /// @dev Performs a low-level call to handle assets in which the decimals are not implemented.
    function safeAssetDecimals(address asset) internal view returns (uint8) {
        (bool success, bytes memory returnData) = asset.staticcall(abi.encodeCall(IERC20Metadata.decimals, ()));
        if (success && returnData.length == 32) {
            return abi.decode(returnData, (uint8));
        } else {
            return 0;
        }
    }

    /// @notice Retrieves the asset's symbol safely, defaulting to a hard-coded value if an error occurs.
    /// @dev Performs a low-level call to handle assets in which the symbol is not implemented or it is a bytes32
    /// instead of a string.
    function safeAssetSymbol(address asset) internal view returns (string memory) {
        (bool success, bytes memory returnData) = asset.staticcall(abi.encodeCall(IERC20Metadata.symbol, ()));

        // Non-empty strings have a length greater than 64, and bytes32 has length 32.
        if (!success || returnData.length <= 64) {
            return "ERC20";
        }

        string memory symbol = abi.decode(returnData, (string));

        // The length check is a precautionary measure to help mitigate potential security threats from malicious assets
        // injecting scripts in the symbol string.
        if (bytes(symbol).length > 30) {
            return "Long Symbol";
        } else {
            return symbol;
        }
    }

    /// @notice Converts the provided fractional amount to a string prefixed by a dot.
    /// @param fractionalAmount A numerical value with 2 implied decimals.
    function stringifyFractionalAmount(uint256 fractionalAmount) internal pure returns (string memory) {
        // Return the empty string if the fractional amount is zero.
        if (fractionalAmount == 0) {
            return "";
        }
        // Add a leading zero if the fractional part is less than 10, e.g. for "1", this function returns ".01%".
        else if (fractionalAmount < 10) {
            return string.concat(".0", fractionalAmount.toString());
        }
        // Otherwise, stringify the fractional amount simply.
        else {
            return string.concat(".", fractionalAmount.toString());
        }
    }

    /// @notice Converts the provided percentage to a string.
    /// @param percentage A numerical value with 4 implied decimals.
    function stringifyPercentage(uint256 percentage) internal pure returns (string memory) {
        // Extract the last two decimals.
        string memory fractionalPart = stringifyFractionalAmount(percentage % 100);

        // Remove the last two decimals.
        string memory wholePart = (percentage / 100).toString();

        // Concatenate the whole and fractional parts.
        return string.concat(wholePart, fractionalPart, "%");
    }

    /// @notice Retrieves the stream's status as a string.
    function stringifyStatus(Lockup.Status status) internal pure returns (string memory) {
        if (status == Lockup.Status.DEPLETED) {
            return "Depleted";
        } else if (status == Lockup.Status.CANCELED) {
            return "Canceled";
        } else if (status == Lockup.Status.STREAMING) {
            return "Streaming";
        } else if (status == Lockup.Status.SETTLED) {
            return "Settled";
        } else {
            return "Pending";
        }
    }
}
