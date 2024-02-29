// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract ConstantsLiterals {

    uint constant public CONSTANT_UINT = 345;
    address constant public CONSTANT_ADDRESS = 0x5B38Da6a701c568545dCfcB03FcB875f56beddC4;
    bytes32 constant public CONSTANT_BYTES32 = 0xafec9bac414fae1094e2941fdd9b8915d38d708cb68c4d092468e10ac3de28f2;

    function good() external {
        uint oneTimeUseOfValue = 123;
        oneTimeUseOfValue = 456;
        uint defaultValueAnd1 = 0;
        defaultValueAnd1 = 1;
        uint multipleUseButConstantUint = CONSTANT_UINT;
        multipleUseButConstantUint = CONSTANT_UINT;
        address multipleUseButConstantAddress = CONSTANT_ADDRESS;
        multipleUseButConstantAddress = CONSTANT_ADDRESS;
        bytes32 multipleUseButConstantBytes32 = CONSTANT_BYTES32;
        multipleUseButConstantBytes32 = CONSTANT_BYTES32;
    }
    
    // 8 instances
    function bad() external {
        uint multipleUseOfValue = 987;
        multipleUseOfValue = 987;
        uint multipleUseOfValue2 = 9876;
        multipleUseOfValue2 = 9876 + 1;
        address multipleUseOfAddress = 0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5;
        multipleUseOfAddress = 0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5;
        bytes32 multipleUseOfBytes32 = 0x8a1b3dbe6301650442bfa765d4de23775fc9a4ec4329ebb5995ec7f1e3777dc4;
        multipleUseOfBytes32 = 0x8a1b3dbe6301650442bfa765d4de23775fc9a4ec4329ebb5995ec7f1e3777dc4;
    }


}

