// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/// @title CLZ Subtraction Pattern Vulnerability Test
/// @notice Tests the sub(const, clz(x)) pattern used in msb/bitLength/log2 calculations
/// @dev This pattern is VERY common in math libraries and causes wraparound when x=0
contract CLZSubtractionPatternTest {
    // ============================================
    // VULNERABLE PATTERNS - H1 Issue
    // ============================================

    /// @notice VULNERABLE: msb calculation without zero check
    /// @dev msb(x) = 255 - clz(x), but when x=0: 255 - 256 = 2^256-1 (wraparound!)
    function brokenMsb(uint256 x) public pure returns (uint256 msb) {
        assembly {
            msb := sub(255, clz(x))
        }
    }

    /// @notice VULNERABLE: bitLength calculation without zero check
    /// @dev bitLength(x) = 256 - clz(x), when x=0: 256 - 256 = 0
    function brokenBitLength(uint256 x) public pure returns (uint256 bits) {
        assembly {
            bits := sub(256, clz(x))
        }
    }

    /// @notice VULNERABLE: log2 calculation without zero check
    /// @dev log2(x) = 255 - clz(x)
    function brokenLog2(uint256 x) public pure returns (uint256 log) {
        assembly {
            log := sub(255, clz(x))
        }
    }

    /// @notice VULNERABLE: Using msb result in shift operation
    /// @dev Demonstrates cascading failure when msb is used in shl
    function brokenInitialGuess(uint256 x) public pure returns (uint256 guess) {
        uint256 msb;
        assembly {
            msb := sub(255, clz(x))
            guess := shl(msb, 1)
        }
    }

    /// @notice VULNERABLE: Using msb as array index
    /// @dev This would cause out-of-bounds access
    function brokenArrayAccess(uint256 x, uint256[256] memory arr) public pure returns (uint256) {
        uint256 msb;
        assembly {
            msb := sub(255, clz(x))
        }
        return arr[msb];
    }

    // ============================================
    // VULNERABLE PATTERNS - SOLIDITY VARIANT
    // ============================================

    /// @notice VULNERABLE: Solidity-level msb without zero check
    /// @dev Pattern: uint256 z; assembly { z := clz(x) } return 255 - z;
    function brokenMsbSolidity(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 255 - clzVal;
    }

    /// @notice VULNERABLE: Solidity-level bitLength without zero check
    function brokenBitLengthSolidity(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - clzVal;
    }

    /// @notice VULNERABLE: Solidity-level log2 without zero check
    function brokenLog2Solidity(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 255 - clzVal;
    }

    /// @notice VULNERABLE: Using result in calculations
    function brokenCalculationSolidity(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        uint256 msb = 255 - clzVal;
        return 2 ** msb;
    }

    // ============================================
    // VULNERABLE PATTERNS - REVERSE ORDER
    // ============================================

    /// @notice VULNERABLE: Reverse order in Yul - sub(clz(x), K)
    /// @dev When x=0: clz(0) - 255 = 256 - 255 = 1 (WRONG!)
    function brokenReverseYul(uint256 x) public pure returns (uint256 result) {
        assembly {
            result := sub(clz(x), 255)
        }
    }

    /// @notice VULNERABLE: Reverse order in Solidity - clzVal - K
    /// @dev Pattern from some versions of PRBMath
    function brokenReverseSolidity(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return clzVal - 255;
    }

    /// @notice VULNERABLE: Reverse with expression - (clz(x) + 1) - K
    function brokenReverseExpression(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return (clzVal + 1) - 256;
    }

    // ============================================
    // VULNERABLE PATTERNS - COMPLEX EXPRESSIONS
    // ============================================

    /// @notice VULNERABLE: K - (clzVal + 1)
    function brokenComplexExpression1(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 255 - (clzVal + 1);
    }

    /// @notice VULNERABLE: K - (1 + clzVal)
    function brokenComplexExpression2(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - (1 + clzVal);
    }

    /// @notice VULNERABLE: (K + offset) - clzVal
    function brokenComplexExpression3(uint256 x) public pure returns (uint256) {
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return (255 + 10) - clzVal;
    }

    // ============================================
    // SAFE PATTERNS - SOLIDITY VARIANT
    // ============================================

    /// @notice FLAGGED: Solidity-level msb with if check (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeMsbSolidityIf(uint256 x) public pure returns (uint256) {
        if (x == 0) revert("msb(0) undefined");
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 255 - clzVal;
    }

    /// @notice FLAGGED: Solidity-level msb with require (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeMsbSolidityRequire(uint256 x) public pure returns (uint256) {
        require(x != 0, "msb(0) undefined");
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 255 - clzVal;
    }

    /// @notice FLAGGED: Solidity-level bitLength (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeBitLengthSolidity(uint256 x) public pure returns (uint256) {
        if (x == 0) return 0;
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - clzVal;
    }

    /// @notice FLAGGED: Solidity-level with unchecked block (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeMsbSolidityUnchecked(uint256 x) public pure returns (uint256) {
        require(x != 0, "msb(0) undefined");
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        unchecked {
            return 255 - clzVal;
        }
    }

    // ============================================
    // VULNERABLE PATTERNS - ASSEMBLY
    // ============================================

    /// @notice VULNERABLE: Using bitLength in normalization
    /// @dev Common pattern in fixed-point math libraries
    function brokenNormalization(uint256 x) public pure returns (uint256 normalized) {
        uint256 bits;
        assembly {
            bits := sub(256, clz(x))
            normalized := shl(bits, x)
        }
    }

    // ============================================
    // SAFE PATTERNS
    // ============================================

    /// @notice FLAGGED: msb with zero check (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeMsb(uint256 x) public pure returns (uint256 msb) {
        require(x != 0, "msb(0) is undefined");
        assembly {
            msb := sub(255, clz(x))
        }
    }

    /// @notice FLAGGED: bitLength with zero check (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeBitLength(uint256 x) public pure returns (uint256 bits) {
        if (x == 0) return 0;
        assembly {
            bits := sub(256, clz(x))
        }
    }

    /// @notice FLAGGED: log2 with zero check (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeLog2(uint256 x) public pure returns (uint256 log) {
        require(x != 0, "log2(0) is undefined");
        assembly {
            log := sub(255, clz(x))
        }
    }

    /// @notice FLAGGED: Initial guess with validation (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeInitialGuess(uint256 x) public pure returns (uint256 guess) {
        require(x != 0, "Cannot calculate guess for zero");
        uint256 msb;
        assembly {
            msb := sub(255, clz(x))
            guess := shl(msb, 1)
        }
    }

    // ============================================
    // INVESTIGATION FUNCTIONS
    // ============================================

    /// @notice Test sub(255, clz(x)) for various inputs
    /// @return input The input value
    /// @return clzResult The CLZ result
    /// @return subResult The result of 255 - clz(x)
    /// @return wrapsAround Whether the result wrapped around
    function testSub255(uint256 x)
        public
        pure
        returns (uint256 input, uint256 clzResult, uint256 subResult, bool wrapsAround)
    {
        input = x;
        assembly {
            clzResult := clz(x)
            subResult := sub(255, clzResult)
        }
        // Check if wraparound occurred (result > 255)
        wrapsAround = subResult > 255;
    }

    /// @notice Test sub(256, clz(x)) for various inputs
    function testSub256(uint256 x)
        public
        pure
        returns (uint256 input, uint256 clzResult, uint256 subResult, bool isZero)
    {
        input = x;
        assembly {
            clzResult := clz(x)
            subResult := sub(256, clzResult)
        }
        isZero = subResult == 0;
    }

    /// @notice Demonstrate the cascading failure in sqrt-like algorithm
    /// @dev Simplified example showing how msb error propagates
    function demonstrateSqrtFailure(uint256 x)
        public
        pure
        returns (uint256 msb, uint256 initialGuess, bool isBroken)
    {
        assembly {
            msb := sub(255, clz(x))
            initialGuess := shl(msb, 1)
        }
        // When x == 0:
        // msb = 2^256-1
        // initialGuess = shl(2^256-1, 1) = 0
        // This breaks the sqrt algorithm!
        isBroken = (x == 0 && initialGuess == 0);
    }

    /// @notice Test common constants used in sub(K, clz(x))
    /// @param x Input value
    /// @param K Constant (commonly 255, 256, 159, etc.)
    /// @return result The result of K - clz(x)
    /// @return overflows Whether it overflows/wraps
    function testSubWithConstant(uint256 x, uint256 K) public pure returns (uint256 result, bool overflows) {
        uint256 clzResult;
        assembly {
            clzResult := clz(x)
            result := sub(K, clzResult)
        }
        // Check if result wrapped around (when K < clzResult)
        overflows = (x == 0 && K < 256);
    }

    /// @notice Demonstrate economic impact in price calculation
    /// @dev Simplified example of how this affects DeFi protocols
    function brokenPriceNormalization(uint256 amount) public pure returns (uint256 normalizedPrice) {
        // Simulate price calculation using msb for normalization
        uint256 msb;
        assembly {
            msb := sub(255, clz(amount))
            // When amount == 0: msb = 2^256-1
            // This completely breaks price calculation!
            normalizedPrice := shl(msb, 1000) // Base price * 2^msb
        }
        // When amount == 0: normalizedPrice = shl(2^256-1, 1000) = 0
        // This could allow buying assets for free!
    }

    /// @notice Safe price normalization
    function safePriceNormalization(uint256 amount) public pure returns (uint256 normalizedPrice) {
        require(amount != 0, "Amount must be non-zero");
        uint256 msb;
        assembly {
            msb := sub(255, clz(amount))
            normalizedPrice := shl(msb, 1000)
        }
    }
}
