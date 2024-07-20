// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ISubtraction {
    function sub(uint256 a, uint256 b) external pure returns (uint256);
}

contract SubWithoutIfContract {
    ISubtraction public subtractionContract;

    constructor(address _subtractionContractAddress) {
        subtractionContract = ISubtraction(_subtractionContractAddress);
    }

    // BAD because no check made before calling `sub`
    function makeFancyFormulaWithoutIf(uint256 a, uint256 b) external view returns (uint256) {
        uint256 magicNumber = subtractionContract.sub(a, b) + 4 ** 7;
        if (magicNumber > 1000000) {
            return 500;
        } else {
            return 100;
        }
    }

    // GOOD because call to `sub` is protected by an If statement
    function makeFancyFormulaWithIf(uint256 a, uint256 b) external view returns (uint256) {
        if (a > b) {
            uint256 magicNumber = subtractionContract.sub(a, b) + 4 ** 7;
            if (magicNumber > 1000000) {
                return 500;
            } else {
                return 100;
            }
        }
        return 200;
    }

    // GOOD because call to `sub` is protected by an If statement
    function makeFancyFormulaWithIf2(uint256 a, uint256 b) external view returns (uint256) {
        if (a > b) {
            return doStuff(a, b);
        }
        return 200;
    }

    function doStuff(uint256 a, uint256 b) internal view returns(uint256) {
        uint256 magicNumber = subtractionContract.sub(a, b) + 4 ** 7;
        if (magicNumber > 1000000) {
            return 500;
        } else {
            return 100;
        }
    }

}

contract SubtractionContract {
    // Function to subtract two numbers
    function sub(uint256 a, uint256 b) external pure returns (uint256) {
        require(b <= a, "Subtraction would result in a negative number");
        return a - b;
    }
}
