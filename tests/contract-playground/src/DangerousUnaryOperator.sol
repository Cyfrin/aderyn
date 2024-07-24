// SPDX-License-Identifier: MIT
pragma solidity ^0.4.0;

// =+ instead of +=

contract WronglyUseOperator {
    
    function wronglyUseOperators() external pure returns(int256){

        int256 counter = 1000;

        counter=+1; // BAD
        counter=-1; // BAD

        counter= +1; // GOOD (because there is a space, it's not a typo)
        counter= -1; // GOOD (because there is a space, it's not a typo)

        counter+=1; // GOOD
        counter-=1; // GOOD

        return counter;
    }
}