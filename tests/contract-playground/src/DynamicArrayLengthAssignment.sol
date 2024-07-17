// SPDX-License-Identifier: MIT
pragma solidity 0.5.0;

contract DynamicArrayLengthAssignment {

	uint256[] public myArray;
    uint256[][] public myArray2;
    mapping(bytes => mapping(uint256 => uint256[])) myArray3;
    bool[] myArray4;

	function badAssignment() external {
        // BAD
		myArray.length = 200;
        myArray2[7].length = 200;
        myArray3[bytes("blah")][5].length = 100;
        myArray4.length = 900;
        myArray.length += 200;

        // GOOD
        uint256 length = 9876 * ( myArray.length + 123456789 );
        uint256 length2 = myArray.length;
	}

}