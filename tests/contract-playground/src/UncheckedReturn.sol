// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;



contract UncheckedReturnExample {

    function one() public pure returns(uint256) {
        return 1;
    }

    function callOneAndDoNothing() internal pure {
        // BAD we're not doing anything with 1
        one();
    }

    event OneCalled(uint256 what);
    error NotTwo();

    function callOneAndDoSomething() internal {
        // GOOD (we're passing one to emit)
        emit OneCalled(one());
    }

    function callTwoAndDoNothing() internal pure {
        // BAD (we're doing nothing)
        UncheckedHelperExternal(address(0x12345)).two();
    }

    function callTwoAndDoSomething() internal pure {
        // GOOD (we're storing the return value in a variable)
        uint256 _answer = UncheckedHelperExternal(address(0x12345)).two();
    }

    function callTwoAndRequireSomething() internal pure {
        // GOOD (we're using the return value in a require)
        require(UncheckedHelperExternal(address(0x12345)).two() == 2, "Not two");
    }

    function callTwoAndEmitError() internal pure {
        // GOOD (we're using the return value in an error)
        if (UncheckedHelperExternal(address(0x12345)).two() != 2) {
            revert NotTwo();
        }
    }

}


contract UncheckedHelperExternal {

    function two() external pure returns(uint256) {
        return 2;
    }

}