// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/// @title CLZ Type Casting Vulnerability Test
/// @notice Tests how clz(x) interacts with type casts to narrow integer types.
/// @dev clz(x) returns 0–256. Only uint8/int8 cannot represent 256, so they overflow.
contract CLZTypeCastingTest {
    // ============================================
    // UNSAFE CASTING PATTERNS
    // ============================================

    /// @notice VULNERABLE: Casting CLZ to uint8
    /// @dev uint8 max is 255; clz(0) = 256 overflows to 0
    function unsafeCastToUint8(uint256 x) public pure returns (uint8) {
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        return uint8(clzResult);
    }

    /// @notice VULNERABLE: Casting CLZ to int8
    /// @dev int8 range is -128..127; clz(0) = 256 wraps when cast
    function unsafeCastToInt8(uint256 x) public pure returns (int8) {
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        return int8(uint8(clzResult));
    }

    /// @notice SAFE IN PRACTICE: uint16 can represent 256
    /// @dev Detector intentionally does NOT flag these wider casts
    function unsafeCastToUint16(uint256 x) public pure returns (uint16) {
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        return uint16(clzResult);
    }

    /// @notice SAFE IN PRACTICE: int16 can represent 256
    /// @dev Detector intentionally does NOT flag these wider casts
    function unsafeCastToInt16(uint256 x) public pure returns (int16) {
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        return int16(uint16(clzResult));
    }

    // ============================================
    // SAFE CASTING PATTERNS
    // ============================================

    /// @notice SAFE: Guard ensures clz(x) is never 256
    function safeCastToUint8(uint256 x) public pure returns (uint8) {
        if (x == 0) return 255;
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        return uint8(clzResult);
    }

    /// @notice SAFE: Guard + upper bound check for int8
    function safeCastToInt8(uint256 x) public pure returns (int8) {
        if (x == 0) return 127;
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        require(clzResult <= 127);
        return int8(uint8(clzResult));
    }

    /// @notice SAFE: Explicitly check uint8 bounds
    function safeCastToUint8WithAssert(uint256 x) public pure returns (uint8) {
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        require(clzResult <= 255);
        return uint8(clzResult);
    }

    // INVESTIGATION FUNCTIONS
    
    function testCastingZero()
        public
        pure
        returns (uint256 clz256, uint8 asUint8, uint16 asUint16, uint32 asUint32)
    {
        uint256 x = 0;
        assembly {
            clz256 := clz(x)
        }
        asUint8 = uint8(clz256);
        asUint16 = uint16(clz256);
        asUint32 = uint32(clz256);
    }

    function testCasting(uint256 x)
        public
        pure
        returns (uint256 clzFull, uint8 asUint8, int8 asInt8, bool overflowsUint8)
    {
        assembly {
            clzFull := clz(x)
        }
        asUint8 = uint8(clzFull);
        asInt8 = int8(uint8(clzFull));
        overflowsUint8 = clzFull > 255;
    }

    /// @notice Example: bit position using uint8 cast (vulnerable for x == 0)
    function calculateBitPosition(uint256 x) public pure returns (uint8 position) {
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        position = uint8(255 - clzResult);
    }

    /// @notice Safe bit position calculation
    function calculateBitPositionSafe(uint256 x) public pure returns (uint8 position) {
        require(x != 0);
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        position = uint8(255 - clzResult);
    }

    // ============================================
    // EDGE CASE TESTS
    // ============================================

    /// @notice Test casting CLZ of maximum value
    function testCastingMax() public pure returns (uint8 result) {
        uint256 x = type(uint256).max;
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        result = uint8(clzResult);
    }

    /// @notice Test casting CLZ of 1
    function testCastingOne() public pure returns (uint8 result) {
        uint256 x = 1;
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        result = uint8(clzResult);
    }

    /// @notice Test casting CLZ of 2
    function testCastingTwo() public pure returns (uint8 result) {
        uint256 x = 2;
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
        }
        result = uint8(clzResult);
    }
}
