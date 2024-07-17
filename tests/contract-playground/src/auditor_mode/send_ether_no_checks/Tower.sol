// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

// Goal of this contract is to test if we can establish a link between 
// the public function `enterTenthFloor` and the fact that `require(msg.sender == 0x011)` 
// could potentially be called as a result
contract Tower1 {

    function visitEighthFloor() internal {
        require(msg.sender == address(0x11));
    }

    modifier passThroughNinthFloor() {
        visitEighthFloor();
        _;
    }

    // Start Here
    function enterTenthFloor() public passThroughNinthFloor() {
        
    }

}

// Goal of this contract is to test if we can establish a link between 
// the public function `enterTenthFloor` and the fact that `x.call{value: 10}("calldata");` 
// could potentially be called as a result.
// Here, the call to send native eth is not safe
contract Tower2 {

    function visitEighthFloor(address x) internal {
        (bool success,) = x.call{value: 10}("calldata");
        if (!success) {
            revert();
        }
    }

    modifier passThroughNinthFloor(address x) {
        visitEighthFloor(x);
        _;
    }

    // Start Here
    function enterTenthFloor(address x) public passThroughNinthFloor(x) {
        
    }

}



// Goal of this contract is to test if we can establish a link between 
// the public function `enterTenthFloor` <-> `x.call{value: 10}("calldata");` 
// and public function `enterTenthFloor` <-> `require(msg.sender == 0x11);` 
// As a result the call to send native eth is safe
contract Tower3 {

    function visitEighthFloor(address x) internal {
        (bool success,) = x.call{value: 10}("calldata");
        if (!success) {
            revert();
        }
    }

    modifier passThroughNinthFloor(address x) {
        visitEighthFloor(x);
        _;
    }

    // Start Here
    function enterTenthFloor(address x) public passThroughNinthFloor(x) {
        visitSeventhFloor();
    }

    function visitSeventhFloor() internal {
        require(msg.sender == address(0x11));
    }

}


