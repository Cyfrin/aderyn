// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract ArithmeticOrder {
    uint public result;

    function calculateWrong(uint a, uint b, uint c, uint d) public {
        result = a * d + b / c * b / d; 
    }

    function calculateAlsoWrong(uint a, uint b, uint c) public {
        result = (a + b / c * b) * c; 
    }

    function calculateAl(uint a, uint b, uint c) public {
        result = (a / b * c); 
    }

    function calculateStillWrong(uint a, uint b, uint c) public {
        result = a + b / c * b * c; 
    }

    function calculateCorrect(uint a, uint b, uint c) public {
        result = a + b * b / c + b * c; 
    }

    function calculateAlsoCorrect(uint a, uint b, uint c, uint d) public {
        result = (a + ((b * d) / (c * b))) * d; 
    }
}
