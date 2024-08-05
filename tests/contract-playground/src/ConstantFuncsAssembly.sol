// SPDX-License-Identifier: MIT
pragma solidity ^0.4.0;

contract AssemblyExample {
    // State variable
    uint256 public value;

    // BAD (view function contains assembly)
    function setValue(uint256 _value) external view {
        assembly {
            // Load the location of the 'value' storage slot
            sstore(0, _value)
        }
    }

    // BAD (pure function contains assembly)
    function getConstantValue() external pure returns (uint256) {
        uint256 result;
        assembly {
            // Inline assembly to set the result to a constant value
            result := 42
        }
        return result;
    }

    function useAssembly() internal pure returns (uint256) {
        uint256 result;
        assembly {
            // Inline assembly to set the result to a constant value
            result := 42
        }
        return result;
    }

    // BAD (pure function contains assembly)
    function getConstantValue2() external pure returns (uint256) {
        return useAssembly();
    }
}
