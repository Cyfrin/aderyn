// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract DivisionBeforeMultiplication {
    uint public result;

    function calculateWrong(uint a, uint b, uint c, uint d) external {
        result = a * d + b / c * b / d; 
    }

    function calculateAlsoWrong(uint a, uint b, uint c) external {
        result = (a + b / c * b) * c; 
    }

    function calculateAl(uint a, uint b, uint c) external {
        result = (a / b * c); 
    }

    function calculateStillWrong(uint a, uint b, uint c) external {
        result = a + b / c * b * c; 
    }

    function calculateCorrect(uint a, uint b, uint c) external {
        result = a + b * b / c + b * c; 
    }

    function calculateAlsoCorrect(uint a, uint b, uint c, uint d) external {
        result = (a + ((b * d) / (c * b))) * d; 
    }
}