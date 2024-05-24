// SPDX-License-Identifier: GPL-3.0-or-later
// solhint-disable max-line-length,quotes
pragma solidity >=0.8.22;

import { Math } from "@openzeppelin/contracts/utils/math/Math.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

library SVGElements {
    using Strings for string;
    using Strings for uint256;

    /*//////////////////////////////////////////////////////////////////////////
                                     CONSTANTS
    //////////////////////////////////////////////////////////////////////////*/

    string internal constant BACKGROUND =
        '<rect width="100%" height="100%" filter="url(#Noise)"/><rect x="70" y="70" width="860" height="860" fill="#fff" fill-opacity=".03" rx="45" ry="45" stroke="#fff" stroke-opacity=".1" stroke-width="4"/>';

    string internal constant BACKGROUND_COLOR = "hsl(230,21%,11%)";

    string internal constant FLOATING_TEXT =
        '<path id="FloatingText" fill="none" d="M125 45h750s80 0 80 80v750s0 80 -80 80h-750s-80 0 -80 -80v-750s0 -80 80 -80"/>';

    string internal constant GLOW = '<circle id="Glow" r="500" fill="url(#RadialGlow)"/>';

    string internal constant LOGO =
        '<path id="Logo" fill="#fff" fill-opacity=".1" d="m133.559,124.034c-.013,2.412-1.059,4.848-2.923,6.402-2.558,1.819-5.168,3.439-7.888,4.996-14.44,8.262-31.047,12.565-47.674,12.569-8.858.036-17.838-1.272-26.328-3.663-9.806-2.766-19.087-7.113-27.562-12.778-13.842-8.025,9.468-28.606,16.153-35.265h0c2.035-1.838,4.252-3.546,6.463-5.224h0c6.429-5.655,16.218-2.835,20.358,4.17,4.143,5.057,8.816,9.649,13.92,13.734h.037c5.736,6.461,15.357-2.253,9.38-8.48,0,0-3.515-3.515-3.515-3.515-11.49-11.478-52.656-52.664-64.837-64.837l.049-.037c-1.725-1.606-2.719-3.847-2.751-6.204h0c-.046-2.375,1.062-4.582,2.726-6.229h0l.185-.148h0c.099-.062,.222-.148,.37-.259h0c2.06-1.362,3.951-2.621,6.044-3.842C57.763-3.473,97.76-2.341,128.637,18.332c16.671,9.946-26.344,54.813-38.651,40.199-6.299-6.096-18.063-17.743-19.668-18.811-6.016-4.047-13.061,4.776-7.752,9.751l68.254,68.371c1.724,1.601,2.714,3.84,2.738,6.192Z"/>';

    string internal constant HOURGLASS_BACKGROUND_CIRCLE =
        '<path d="M 50,360 a 300,300 0 1,1 600,0 a 300,300 0 1,1 -600,0" fill="#fff" fill-opacity=".02" stroke="url(#HourglassStroke)" stroke-width="4"/>';

    string internal constant HOURGLASS_FILL =
        '<path d="m566,161.201v-53.924c0-19.382-22.513-37.563-63.398-51.198-40.756-13.592-94.946-21.079-152.587-21.079s-111.838,7.487-152.602,21.079c-40.893,13.636-63.413,31.816-63.413,51.198v53.924c0,17.181,17.704,33.427,50.223,46.394v284.809c-32.519,12.96-50.223,29.206-50.223,46.394v53.924c0,19.382,22.52,37.563,63.413,51.198,40.763,13.592,94.954,21.079,152.602,21.079s111.831-7.487,152.587-21.079c40.886-13.636,63.398-31.816,63.398-51.198v-53.924c0-17.196-17.704-33.435-50.223-46.401V207.603c32.519-12.967,50.223-29.206,50.223-46.401Zm-347.462,57.793l130.959,131.027-130.959,131.013V218.994Zm262.924.022v262.018l-130.937-131.006,130.937-131.013Z" fill="#161822"></path>';

    string internal constant HOURGLASS_STROKE =
        '<g fill="none" stroke="url(#HourglassStroke)" stroke-linecap="round" stroke-miterlimit="10" stroke-width="4"><path d="m565.641,107.28c0,9.537-5.56,18.629-15.676,26.973h-.023c-9.204,7.596-22.194,14.562-38.197,20.592-39.504,14.936-97.325,24.355-161.733,24.355-90.48,0-167.948-18.582-199.953-44.948h-.023c-10.115-8.344-15.676-17.437-15.676-26.973,0-39.735,96.554-71.921,215.652-71.921s215.629,32.185,215.629,71.921Z"/><path d="m134.36,161.203c0,39.735,96.554,71.921,215.652,71.921s215.629-32.186,215.629-71.921"/><line x1="134.36" y1="161.203" x2="134.36" y2="107.28"/><line x1="565.64" y1="161.203" x2="565.64" y2="107.28"/><line x1="184.584" y1="206.823" x2="184.585" y2="537.579"/><line x1="218.181" y1="218.118" x2="218.181" y2="562.537"/><line x1="481.818" y1="218.142" x2="481.819" y2="562.428"/><line x1="515.415" y1="207.352" x2="515.416" y2="537.579"/><path d="m184.58,537.58c0,5.45,4.27,10.65,12.03,15.42h.02c5.51,3.39,12.79,6.55,21.55,9.42,30.21,9.9,78.02,16.28,131.83,16.28,49.41,0,93.76-5.38,124.06-13.92,2.7-.76,5.29-1.54,7.75-2.35,8.77-2.87,16.05-6.04,21.56-9.43h0c7.76-4.77,12.04-9.97,12.04-15.42"/><path d="m184.582,492.656c-31.354,12.485-50.223,28.58-50.223,46.142,0,9.536,5.564,18.627,15.677,26.969h.022c8.503,7.005,20.213,13.463,34.524,19.159,9.999,3.991,21.269,7.609,33.597,10.788,36.45,9.407,82.181,15.002,131.835,15.002s95.363-5.595,131.807-15.002c10.847-2.79,20.867-5.926,29.924-9.349,1.244-.467,2.473-.942,3.673-1.424,14.326-5.696,26.035-12.161,34.524-19.173h.022c10.114-8.342,15.677-17.433,15.677-26.969,0-17.562-18.869-33.665-50.223-46.15"/><path d="m134.36,592.72c0,39.735,96.554,71.921,215.652,71.921s215.629-32.186,215.629-71.921"/><line x1="134.36" y1="592.72" x2="134.36" y2="538.797"/><line x1="565.64" y1="592.72" x2="565.64" y2="538.797"/><polyline points="481.822 481.901 481.798 481.877 481.775 481.854 350.015 350.026 218.185 218.129"/><polyline points="218.185 481.901 218.231 481.854 350.015 350.026 481.822 218.152"/></g>';

    string internal constant HOURGLASS_LOWER_BULB_LARGE =
        '<path d="m481.46,481.54v81.01c-2.35.77-4.82,1.51-7.39,2.23-30.3,8.54-74.65,13.92-124.06,13.92-53.6,0-101.24-6.33-131.47-16.16v-81l46.3-46.31h170.33l46.29,46.31Z" fill="url(#SandBottom)"/><path d="m435.17,435.23c0,1.17-.46,2.32-1.33,3.44-7.11,9.08-41.93,15.98-83.81,15.98s-76.7-6.9-83.82-15.98c-.87-1.12-1.33-2.27-1.33-3.44v-.04l8.34-8.35.01-.01c13.72-6.51,42.95-11.02,76.8-11.02s62.97,4.49,76.72,11l8.42,8.42Z" fill="url(#SandTop)"/>';

    string internal constant HOURGLASS_LOWER_BULB_SMALL =
        '<path d="m481.46,504.101v58.449c-2.35.77-4.82,1.51-7.39,2.23-30.3,8.54-74.65,13.92-124.06,13.92-53.6,0-101.24-6.33-131.47-16.16v-58.439h262.92Z" fill="url(#SandBottom)"/><ellipse cx="350" cy="504.101" rx="131.462" ry="28.108" fill="url(#SandTop)"/>';

    string internal constant HOURGLASS_UPPER_BULB =
        '<polygon points="350 350.026 415.03 284.978 285 284.978 350 350.026" fill="url(#SandBottom)"/><path d="m416.341,281.975c0,.914-.354,1.809-1.035,2.68-5.542,7.076-32.661,12.45-65.28,12.45-32.624,0-59.738-5.374-65.28-12.45-.681-.872-1.035-1.767-1.035-2.68,0-.914.354-1.808,1.035-2.676,5.542-7.076,32.656-12.45,65.28-12.45,32.619,0,59.738,5.374,65.28,12.45.681.867,1.035,1.762,1.035,2.676Z" fill="url(#SandTop)"/>';

    string internal constant NOISE =
        '<filter id="Noise"><feFlood x="0" y="0" width="100%" height="100%" flood-color="hsl(230,21%,11%)" flood-opacity="1" result="floodFill"/><feTurbulence baseFrequency=".4" numOctaves="3" result="Noise" type="fractalNoise"/><feBlend in="Noise" in2="floodFill" mode="soft-light"/></filter>';

    /// @dev Escape character for "≥".
    string internal constant SIGN_GE = "&#8805;";

    /// @dev Escape character for ">".
    string internal constant SIGN_GT = "&gt;";

    /// @dev Escape character for "<".
    string internal constant SIGN_LT = "&lt;";

    /*//////////////////////////////////////////////////////////////////////////
                                     DATA TYPES
    //////////////////////////////////////////////////////////////////////////*/

    enum CardType {
        PROGRESS,
        STATUS,
        AMOUNT,
        DURATION
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     COMPONENTS
    //////////////////////////////////////////////////////////////////////////*/

    function card(CardType cardType, string memory content) internal pure returns (uint256, string memory) {
        return card({ cardType: cardType, content: content, circle: "" });
    }

    function card(
        CardType cardType,
        string memory content,
        string memory circle
    )
        internal
        pure
        returns (uint256 width, string memory card_)
    {
        string memory caption = stringifyCardType(cardType);

        // The progress card can have a fixed width because the content is never longer than the caption. The former
        // has 6 characters (at most, e.g. "42.09%"), whereas the latter has 8 characters ("Progress").
        if (cardType == CardType.PROGRESS) {
            // The progress can be 0%, in which case the circle is not rendered.
            if (circle.equal("")) {
                width = 144; // 2 * 20 (margins) + 8 * 13 (charWidth)
            } else {
                width = 208; // 3 * 20 (margins) + 8 * 13 (charWidth) + 44 (diameter)
            }
        }
        // For the other cards, the width is calculated dynamically based on the number of characters.
        else {
            uint256 captionWidth = calculatePixelWidth({ text: caption, largeFont: false });
            uint256 contentWidth = calculatePixelWidth({ text: content, largeFont: true });

            // Use the greater of the two widths, and add the left and the right margin.
            unchecked {
                width = Math.max(captionWidth, contentWidth) + 40;
            }
        }

        card_ = string.concat(
            '<g id="',
            caption,
            '" fill="#fff">',
            '<rect width="',
            width.toString(),
            '" height="100" fill-opacity=".03" rx="15" ry="15" stroke="#fff" stroke-opacity=".1" stroke-width="4"/>',
            '<text x="20" y="34" font-family="\'Courier New\',Arial,monospace" font-size="22px">',
            caption,
            "</text>",
            '<text x="20" y="72" font-family="\'Courier New\',Arial,monospace" font-size="26px">',
            content,
            "</text>",
            circle,
            "</g>"
        );
    }

    function floatingText(string memory offset, string memory text) internal pure returns (string memory) {
        return string.concat(
            '<textPath startOffset="',
            offset,
            '" href="#FloatingText" fill="#fff" font-family="\'Courier New\',Arial,monospace" fill-opacity=".8" font-size="26px">',
            '<animate additive="sum" attributeName="startOffset" begin="0s" dur="50s" from="0%" repeatCount="indefinite" to="100%"/>',
            text,
            "</textPath>"
        );
    }

    function gradients(string memory accentColor) internal pure returns (string memory) {
        string memory radialGlow = string.concat(
            '<radialGradient id="RadialGlow">',
            '<stop offset="0%" stop-color="',
            accentColor,
            '" stop-opacity=".6"/>',
            '<stop offset="100%" stop-color="',
            BACKGROUND_COLOR,
            '" stop-opacity="0"/>',
            "</radialGradient>"
        );
        string memory sandTop = string.concat(
            '<linearGradient id="SandTop" x1="0%" y1="0%">',
            '<stop offset="0%" stop-color="',
            accentColor,
            '"/>',
            '<stop offset="100%" stop-color="',
            BACKGROUND_COLOR,
            '"/>',
            "</linearGradient>"
        );
        string memory sandBottom = string.concat(
            '<linearGradient id="SandBottom" x1="100%" y1="100%">',
            '<stop offset="10%" stop-color="',
            BACKGROUND_COLOR,
            '"/>',
            '<stop offset="100%" stop-color="',
            accentColor,
            '"/>',
            '<animate attributeName="x1" dur="6s" repeatCount="indefinite" values="30%;60%;120%;60%;30%;"/>',
            "</linearGradient>"
        );
        // Needs to be declared last so that the stroke is painted on top of the sand.
        string memory hourglassStroke = string.concat(
            '<linearGradient id="HourglassStroke" gradientTransform="rotate(90)" gradientUnits="userSpaceOnUse">',
            '<stop offset="50%" stop-color="',
            accentColor,
            '"/>',
            '<stop offset="80%" stop-color="',
            BACKGROUND_COLOR,
            '"/>',
            "</linearGradient>"
        );
        return string.concat(radialGlow, sandTop, sandBottom, hourglassStroke);
    }

    function hourglass(string memory status) internal pure returns (string memory) {
        bool settledOrDepleted = status.equal("Settled") || status.equal("Depleted");
        return string.concat(
            '<g id="Hourglass">',
            HOURGLASS_BACKGROUND_CIRCLE,
            HOURGLASS_FILL,
            settledOrDepleted ? "" : HOURGLASS_UPPER_BULB, // empty or filled
            settledOrDepleted ? HOURGLASS_LOWER_BULB_LARGE : HOURGLASS_LOWER_BULB_SMALL,
            HOURGLASS_STROKE, // needs to be declared last so that the stroke is painted on top of the sand
            "</g>"
        );
    }

    function progressCircle(
        uint256 progressNumerical,
        string memory accentColor
    )
        internal
        pure
        returns (string memory)
    {
        if (progressNumerical == 0) {
            return "";
        }
        return string.concat(
            '<g fill="none">',
            '<circle cx="166" cy="50" r="22" stroke="',
            BACKGROUND_COLOR,
            '" stroke-width="10"/>',
            '<circle cx="166" cy="50" pathLength="10000" r="22" stroke="',
            accentColor,
            '" stroke-dasharray="10000" stroke-dashoffset="',
            (10_000 - progressNumerical).toString(),
            '" stroke-linecap="round" stroke-width="5" transform="rotate(-90)" transform-origin="166 50"/>',
            "</g>"
        );
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Calculates the pixel width of the provided string.
    /// @dev Notes:
    /// - A factor of ~0.6 is applied to the two font sizes used in the SVG (26px and 22px) to approximate the average
    /// character width.
    /// - It is assumed that escaped characters are placed at the beginning of `text`.
    /// - It is further assumed that there is no other semicolon in `text`.
    function calculatePixelWidth(string memory text, bool largeFont) internal pure returns (uint256 width) {
        uint256 length = bytes(text).length;
        if (length == 0) {
            return 0;
        }

        unchecked {
            uint256 charWidth = largeFont ? 16 : 13;
            uint256 semicolonIndex;
            for (uint256 i = 0; i < length; ++i) {
                if (bytes(text)[i] == ";") {
                    semicolonIndex = i;
                }
                width += charWidth;
            }

            // Account for escaped characters (such as &#8805;).
            width -= charWidth * semicolonIndex;
        }
    }

    /// @notice Retrieves the card type as a string.
    function stringifyCardType(CardType cardType) internal pure returns (string memory) {
        if (cardType == CardType.PROGRESS) {
            return "Progress";
        } else if (cardType == CardType.STATUS) {
            return "Status";
        } else if (cardType == CardType.AMOUNT) {
            return "Amount";
        } else {
            return "Duration";
        }
    }
}
