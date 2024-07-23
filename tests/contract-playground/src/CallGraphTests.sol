// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract Tower1 {

    function visitEighthFloor1() internal {
        require(msg.sender == address(0x11));
    }

    modifier passThroughNinthFloor1() {
        visitEighthFloor1();
        _;
    }

    // Start Here
    function enterTenthFloor1() external passThroughNinthFloor1() {
        
    }

}


contract Tower2 {

    function visitEighthFloor2(address x) internal {
        (bool success,) = x.call{value: 10}("calldata");
        if (!success) {
            revert();
        }
    }

    modifier passThroughNinthFloor2(address x) {
        visitEighthFloor2(x);
        _;
    }

    // Start Here
    function enterTenthFloor2(address x) external passThroughNinthFloor2(x) {
        
    }

}


contract Tower3 {

    function visitEighthFloor3(address x) internal {
        (bool success,) = x.call{value: 10}("calldata");
        if (!success) {
            revert();
        }
    }

    modifier passThroughNinthFloor3(address x) {
        visitEighthFloor3(x);
        _;
    }

    // Start Here
    function enterTenthFloor3(address x) external passThroughNinthFloor3(x) {
        visitSeventhFloor3();
    }

    function visitSeventhFloor3() internal {
        require(msg.sender == address(0x11));
    }

}

contract Tower4 {
    // A recursive function should have itself as upstream and downstream
    function recurse(string memory something) private {
        recurse(something);
    }
}