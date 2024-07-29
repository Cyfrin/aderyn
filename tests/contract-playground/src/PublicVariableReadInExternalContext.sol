// SPDX-License-Identifier: MIT
pragma solidity 0.5.0;

contract PublicVariableReadUsingThis {
    string[] public testArray;
    uint256 public testUint256;
    mapping(uint256 => bool) public testMap;

    // BAD ways of reading (using this.<public state variable>)

    function readStringArray() external view returns (string memory) {
        return this.testArray(0);
    }

    function readUint256() external view returns (uint256) {
        return this.testUint256();
    }

    function readMap() external view returns (bool) {
        return this.testMap(0);
    }

    // GOOD ways of reading (using <public state variable>)

    function readStringArrayGood() external view returns (string memory) {
        return testArray[0];
    }

    function readUint256Good() external view returns (uint256) {
        return testUint256;
    }

    function readMapGood() external view returns (bool) {
        return testMap[0];
    }
}

contract DerivedFromPublicVariableReadUsingThis is PublicVariableReadUsingThis {
    // BAD ways of reading (using this.<public state variable>)

    function readStringArray() external view returns (string memory) {
        return this.testArray(0);
    }

    // GOOD ways of reading (using <public state variable>)

    function readStringArrayGood() external view returns (string memory) {
        return testArray[0];
    }
}
