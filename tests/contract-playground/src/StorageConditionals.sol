// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract StorageConditionals {
    uint256 private s_sameConditionals;
    uint256 private s_differentConditionals;

    constructor(uint256 same, uint256 different) {
        s_sameConditionals = same;
        s_differentConditionals = different;
    }

    // SAME CONDITIONALS

    event Same();

    function sameOne(uint256 number) external {
        require(number < s_sameConditionals, "Must be less than");
        emit Same();
    }

    function sameTwo(uint256 number) external {
        if (number < s_sameConditionals) {
            emit Same();
        }
    }

    function sameThree(uint256 number) external {
        if (number < s_sameConditionals) {
            emit Same();
        } else {
            revert("Must be less than");
        }
    }

    function sameButReversedOne(uint256 number) external {
        require(s_sameConditionals > number, "Must be less than");
        emit Same();
    }

    function sameButReversedTwo(uint256 number) external {
        if (s_sameConditionals > number) {
            emit Same();
        }
    }

    function sameButReversedThree(uint256 number) external {
        if (s_sameConditionals > number) {
            emit Same();
        } else {
            revert("Must be less than");
        }
    }    

    // DIFFERENT CONDITIONALS

    event Different();

    function differentOne(uint256 number) external {
        require(number < s_differentConditionals, "Must be less than");
        emit Different();
    }

    function differentTwo(uint256 number) external {
        if (number <= s_differentConditionals) {
            emit Different();
        }
    }

    function differentThree(uint256 number) external {
        if (number > s_differentConditionals) {
            emit Different();
        } else {
            revert("Must be less than");
        }
    }
}