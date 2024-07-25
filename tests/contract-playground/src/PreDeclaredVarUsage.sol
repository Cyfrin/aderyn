// SPDX-License-Identifier: MIT
pragma solidity ^0.4.0;

contract PreDeclaredVariableUsage {

    function useBeforeDeclaring() external pure returns (uint) {
        /* a, b used here */
        a = 100;
        uint b = 100; 

        /* But, a is declared here */
        uint a;

        return a + b;
    }

}