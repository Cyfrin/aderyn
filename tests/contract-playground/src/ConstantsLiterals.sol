// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract ConstantsLiterals {

    uint constant public CONSTANT = 345;

    function good() external {
        uint oneTimeUseOfValue = 123;
        oneTimeUseOfValue = 456;
        uint defaultValueAnd1 = 0;
        defaultValueAnd1 = 1;
        uint multipleUseButConstant = CONSTANT;
        multipleUseButConstant = CONSTANT;
    }
    
    function bad() external {
        uint multipleUseOfValue = 987;
        multipleUseOfValue = 987;
        uint multipleUseOfValue2 = 9876;
        multipleUseOfValue2 = 9876 + 1;
    }


}

