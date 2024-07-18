// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

// Goal of this contract is to test if we can establish a link between 
// the public function `enterTenthFloor` and the fact that `require(msg.sender == 0x011)` 
// could potentially be called as a result
contract Tower1 {

    function visitEighthFloor1() internal {
        require(msg.sender == address(0x11));
    }

    modifier passThroughNinthFloor1() {
        visitEighthFloor1();
        _;
    }

    // Start Here
    function enterTenthFloor1() public passThroughNinthFloor1() {
        
    }

}

// Goal of this contract is to test if we can establish a link between 
// the public function `enterTenthFloor` and the fact that `x.call{value: 10}("calldata");` 
// could potentially be called as a result.
// Here, the call to send native eth is not safe
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
    function enterTenthFloor2(address x) public passThroughNinthFloor2(x) {
        
    }

}



// Goal of this contract is to test if we can establish a link between 
// the public function `enterTenthFloor` <-> `x.call{value: 10}("calldata");` 
// and public function `enterTenthFloor` <-> `require(msg.sender == 0x11);` 
// As a result the call to send native eth is safe
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
    function enterTenthFloor3(address x) public passThroughNinthFloor3(x) {
        visitSeventhFloor3();
    }

    function visitSeventhFloor3() internal {
        require(msg.sender == address(0x11));
    }

}


