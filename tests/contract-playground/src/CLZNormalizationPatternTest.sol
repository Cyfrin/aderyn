// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/// @title CLZ Normalization Pattern Vulnerability Test (H2)
/// @notice Tests the shr(C, shl(clz(x), x)) pattern used in lnWad/log2 normalization
/// @dev This pattern is from Solady-style lnWad implementation, mentioned in EIP-7939
contract CLZNormalizationPatternTest {
    // ============================================
    // VULNERABLE PATTERNS - H2 Issue
    // ============================================

    /// @notice VULNERABLE: lnWad-style normalization without zero check
    /// @dev Pattern: x' = shr(C, shl(clz(x), x)), k = sub(C, clz(x))
    function brokenNormalize(uint256 x) public pure returns (uint256 normalized, uint256 k) {
        assembly {
            let r := clz(x)
            // Normalize x to range (1,2)*2^96
            normalized := shr(159, shl(r, x))
            // Calculate log2 offset: k = 159 - clz(x)
            k := sub(159, r)
        }
        // When x == 0:
        // r = 256
        // normalized = shr(159, shl(256, 0)) = 0
        // k = sub(159, 256) = 2^256 - 97 (GARBAGE!)
    }

    /// @notice VULNERABLE: Simplified lnWad calculation
    /// @dev Demonstrates how garbage values propagate through ln calculation
    function brokenLnWad(uint256 x) public pure returns (int256 result) {
        uint256 normalized;
        uint256 k;
        assembly {
            let r := clz(x)
            normalized := shr(159, shl(r, x))
            k := sub(159, r)
        }
        // When x == 0: normalized = 0, k = 2^256-97
        // This produces completely wrong ln(x) instead of reverting!
        // Simplified: result = ln(normalized) + k * ln(2)
        // (In reality, there's a polynomial approximation)
        result = int256(k); // Garbage value!
    }

    /// @notice VULNERABLE: exp/pow using normalization
    /// @dev Shows how this affects exponential calculations
    function brokenExpNormalization(uint256 x) public pure returns (uint256 result) {
        uint256 normalized;
        uint256 k;
        assembly {
            let r := clz(x)
            normalized := shr(159, shl(r, x))
            k := sub(159, r)
        }
        // Use normalized and k in exp calculation
        // When x == 0: garbage values lead to wrong result
        result = normalized + k; // Simplified, shows garbage propagation
    }

    /// @notice VULNERABLE: AMM curve calculation using ln
    /// @dev Economic impact: wrong prices in AMM
    function brokenAmmPrice(uint256 reserve) public pure returns (uint256 price) {
        uint256 normalized;
        uint256 k;
        assembly {
            let r := clz(reserve)
            normalized := shr(159, shl(r, reserve))
            k := sub(159, r)
        }
        // AMM price calculation using ln(reserve)
        // When reserve == 0: price is garbage, not revert!
        unchecked {
            price = 1e18 * k / (normalized + 1);
        }
    }

    /// @notice VULNERABLE: Interest rate calculation
    /// @dev Shows impact on lending protocols
    function brokenInterestRate(uint256 utilization) public pure returns (uint256 rate) {
        uint256 normalized;
        uint256 k;
        assembly {
            let r := clz(utilization)
            normalized := shr(159, shl(r, utilization))
            k := sub(159, r)
        }
        // Interest rate based on ln(utilization)
        // When utilization == 0: rate is garbage!
        unchecked {
            rate = k * 100; // Simplified
        }
    }

    // ============================================
    // SAFE PATTERNS
    // ============================================

    /// @notice FLAGGED: Normalization with zero check (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeNormalize(uint256 x) public pure returns (uint256 normalized, uint256 k) {
        require(x != 0, "Cannot normalize zero");
        uint256 r;
        assembly {
            r := clz(x)
            normalized := shr(159, shl(r, x))
            k := sub(159, r)
        }
    }

    /// @notice SAFE: lnWad with zero check
    function safeLnWad(uint256 x) public pure returns (int256 result) {
        require(x != 0, "ln(0) is undefined");
        uint256 k;
        assembly {
            let r := clz(x)
            k := sub(159, r)
        }
        result = int256(k);
    }

    /// @notice FLAGGED: Zero check in assembly (detector flags all patterns)
    /// @dev Has proper guard, but detector conservatively flags it for manual review
    function safeNormalizeAsm(uint256 x) public pure returns (uint256 normalized, uint256 k) {
        assembly {
            if iszero(x) {
                revert(0, 0)
            }
            let r := clz(x)
            normalized := shr(159, shl(r, x))
            k := sub(159, r)
        }
    }

    // ============================================
    // INVESTIGATION FUNCTIONS
    // ============================================

    /// @notice Test normalization with various constants
    /// @param x Input value
    /// @param C Constant for shift (commonly 159, 96, 127, etc.)
    function testNormalizationWithConstant(uint256 x, uint256 C)
        public
        pure
        returns (uint256 clzResult, uint256 normalized, uint256 k, bool isGarbage)
    {
        assembly {
            clzResult := clz(x)
            normalized := shr(C, shl(clzResult, x))
            k := sub(C, clzResult)
        }
        // Check if result is garbage (when x == 0)
        isGarbage = (x == 0);
    }

    /// @notice Demonstrate the full lnWad-style calculation
    function demonstrateLnWadPattern(uint256 x)
        public
        pure
        returns (uint256 r, uint256 normalized, uint256 k, bool isBroken)
    {
        assembly {
            r := clz(x)
            normalized := shr(159, shl(r, x))
            k := sub(159, r)
        }
        // When x == 0:
        // r = 256
        // normalized = 0
        // k = 2^256 - 97
        isBroken = (x == 0 && k > type(uint128).max);
    }

    /// @notice Test shift overflow behavior
    /// @dev shl(256, x) should return 0 for any x
    function testShiftOverflow(uint256 x) public pure returns (uint256 shifted, bool isZero) {
        assembly {
            shifted := shl(256, x)
        }
        isZero = (shifted == 0);
    }

    /// @notice Compare broken vs safe normalization
    function compareNormalization(uint256 x)
        public
        pure
        returns (uint256 brokenNorm, uint256 brokenK, bool safeReverts)
    {
        // Broken version
        assembly {
            let r := clz(x)
            brokenNorm := shr(159, shl(r, x))
            brokenK := sub(159, r)
        }
        // Safe version would revert for x == 0
        safeReverts = (x == 0);
    }

    /// @notice Demonstrate economic impact in DeFi
    /// @dev Shows how garbage values affect protocol economics
    function demonstrateEconomicImpact(uint256 amount)
        public
        pure
        returns (uint256 calculatedValue, bool isExploitable)
    {
        uint256 normalized;
        uint256 k;
        assembly {
            let r := clz(amount)
            normalized := shr(159, shl(r, amount))
            k := sub(159, r)
        }
        // Simulate value calculation (e.g., collateral value, debt, etc.)
        // Use unchecked to demonstrate the overflow issue
        unchecked {
            calculatedValue = k * 1e18;
        }
        // When amount == 0: calculatedValue is huge (wraps around)
        // This could allow:
        // - Infinite collateral from zero deposit
        // - Wrong liquidation thresholds
        // - Broken interest calculations
        isExploitable = (amount == 0 && k > type(uint128).max);
    }

    /// @notice Test with different normalization constants
    /// @dev Common constants: 159 (lnWad), 96, 127, 255
    function testCommonConstants(uint256 x)
        public
        pure
        returns (uint256 norm159, uint256 k159, uint256 norm96, uint256 k96)
    {
        assembly {
            let r := clz(x)

            // Constant 159 (lnWad style)
            norm159 := shr(159, shl(r, x))
            k159 := sub(159, r)

            // Constant 96 (alternative normalization)
            norm96 := shr(96, shl(r, x))
            k96 := sub(96, r)
        }
    }
}