// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/// @title CLZ Signed Integer Usage Test
/// @notice Tests CLZ usage with signed integer types (int256, int128, etc.)
/// @dev  Issue: CLZ on negative values always returns 0 due to two's complement
contract CLZSignedIntegerTest {

    /// @notice VULNERABLE: CLZ on int256 without validation
    /// @dev For negative values, CLZ always returns 0 (highest bit is 1)
    function brokenBitLengthInt256(int256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - clzVal;
        // For x = -1: CLZ = 0, returns 256 (WRONG!)
        // For x = -100: CLZ = 0, returns 256 (WRONG!)
    }

    /// @notice VULNERABLE: CLZ on int128 without validation
    function brokenBitLengthInt128(int128 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - clzVal;
    }

    /// @notice VULNERABLE: CLZ on int64 without validation
    function brokenBitLengthInt64(int64 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - clzVal;
    }

    /// @notice VULNERABLE: Using CLZ result in branching logic
    /// @dev Negative values will always take the "large number" branch
    function brokenBranchingInt256(int256 x) public pure returns (string memory) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        uint256 bitLength = 256 - clzVal;

        if (bitLength > 128) {
            return "Large number"; // Negative values always go here!
        } else {
            return "Small number";
        }
    }

    /// @notice VULNERABLE: Using CLZ for range estimation
    function brokenRangeCheckInt256(int256 x) public pure returns (bool) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        uint256 bitLength = 256 - clzVal;
        // Trying to check if value fits in 64 bits
        return bitLength <= 64; // Always false for negative!
    }

    /// @notice VULNERABLE: Mixed signed/unsigned operations
    function brokenMixedOperations(int256 signed, uint256 unsigned) public pure returns (uint256) {
        uint256 signedClz;
        uint256 unsignedClz;
        assembly {
            signedClz := clz(signed)
            unsignedClz := clz(unsigned)
        }
        // Comparing CLZ results - broken for negative signed values
        return signedClz + unsignedClz;
    }

    // ============================================
    // SAFE PATTERNS
    // ============================================

    /// @notice SAFE: Validate non-negative before CLZ
    function safeBitLengthInt256(int256 x) public pure returns (uint256) {
        require(x >= 0, "CLZ requires non-negative value");
        uint256 unsigned = uint256(x);
        uint256 clzVal;
        assembly {
            clzVal := clz(unsigned)
        }
        return 256 - clzVal;
    }

    /// @notice SAFE: Convert to absolute value
    function safeBitLengthAbsInt256(int256 x) public pure returns (uint256) {
        uint256 abs = x >= 0 ? uint256(x) : uint256(-x);
        uint256 clzVal;
        assembly {
            clzVal := clz(abs)
        }
        return 256 - clzVal;
    }

    /// @notice SAFE: Explicit handling of negative values
    function safeBitLengthExplicitInt256(int256 x) public pure returns (uint256) {
        if (x < 0) {
            // For negative values in two's complement, all bits are significant
            return 256;
        }
        uint256 unsigned = uint256(x);
        uint256 clzVal;
        assembly {
            clzVal := clz(unsigned)
        }
        return 256 - clzVal;
    }

    /// @notice SAFE: Type conversion with validation
    function safeConvertAndCheck(int256 x) public pure returns (uint256) {
        require(x >= 0, "Value must be non-negative for CLZ");
        return safeBitLengthInt256(x);
    }

    // ============================================
    // INVESTIGATION FUNCTIONS
    // ============================================

    /// @notice Test CLZ behavior with various signed values
    function testSignedValues(int256 x)
        public
        pure
        returns (int256 input, uint256 clzResult, uint256 bitLength, bool isNegative)
    {
        input = x;
        isNegative = x < 0;
        assembly {
            clzResult := clz(x)
        }
        bitLength = 256 - clzResult;
    }

    /// @notice Demonstrate two's complement representation
    function demonstrateTwosComplement()
        public
        pure
        returns (uint256 minusOne, uint256 minusOneHundred, uint256 clzMinusOne, uint256 clzMinusOneHundred)
    {
        int256 neg1 = -1;
        int256 neg100 = -100;

        assembly {
            minusOne := neg1
            minusOneHundred := neg100
            clzMinusOne := clz(neg1)
            clzMinusOneHundred := clz(neg100)
        }
        // Both CLZ results will be 0 because highest bit is 1
    }

    /// @notice Compare CLZ of positive and negative values
    function comparePositiveNegative(int256 positive, int256 negative)
        public
        pure
        returns (uint256 posClz, uint256 negClz, bool negativeAlwaysZero)
    {
        require(positive > 0, "First arg must be positive");
        require(negative < 0, "Second arg must be negative");

        assembly {
            posClz := clz(positive)
            negClz := clz(negative)
        }
        negativeAlwaysZero = (negClz == 0);
    }

    /// @notice Test all signed integer types
    function testAllSignedTypes(int8 i8, int16 i16, int32 i32, int64 i64, int128 i128, int256 i256)
        public
        pure
        returns (uint256 clz8, uint256 clz16, uint256 clz32, uint256 clz64, uint256 clz128, uint256 clz256)
    {
        assembly {
            clz8 := clz(i8)
            clz16 := clz(i16)
            clz32 := clz(i32)
            clz64 := clz(i64)
            clz128 := clz(i128)
            clz256 := clz(i256)
        }
        // All negative values will have CLZ = 0
    }
    // ============================================
    // NEW TEST CASES (User Feedback)
    // ============================================

    /// @notice VULNERABLE: CLZ on function parameter (Problem #3)
    function brokenParam(int256 param) public pure {
        assembly {
            pop(clz(param))
        }
    }

    /// @notice VULNERABLE: CLZ on expression (Problem #2)
    function brokenExpression(int256 x) public pure {
        assembly {
            pop(clz(sub(x, 1)))
        }
    }

    /// @notice VULNERABLE: CLZ on nested expression
    function brokenNestedExpression(int256 x) public pure {
        assembly {
            pop(clz(add(sub(x, 1), 5)))
        }
    }

    /// @notice SAFE: CLZ on cast to uint (Problem #4)
    function safeCastExpression(int256 x) public pure {
        assembly {
            function uint256(a) -> r { r := a }
            pop(clz(uint256(x))) // Should be ignored
        }
    }

    /// @notice VULNERABLE: CLZ on cast to int (Problem #6)
    function brokenCastToInt(uint256 x) public pure {
        assembly {
            function int256(a) -> r { r := a }
            pop(clz(int256(x)))
        }
    }
}