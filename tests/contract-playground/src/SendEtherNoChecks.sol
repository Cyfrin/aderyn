// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;


////// GOOD ////////////////

contract SendEtherNoChecks2 {

    function callAndSendNativeEth(address x) internal {
        (bool success,) = x.call{value: 10}("calldata");
        if (!success) {
            revert();
        }
    }

    modifier mod1(address x) {
        callAndSendNativeEth(x);
        _;
    }

    // Start Here
    function func1(address x) external mod1(x) {
        func2();
    }

    function func2() internal view {
        require(msg.sender == address(0x11));
    }

}

/////////// BAD ///////////////

// Sending eth from func1 in the following contracts is not safe because there is no check on any address 
// before sending native eth.

/// BAD
contract SendEtherNoChecks3 {

    function callAndSendNativeEth(address x) internal {
        (bool success,) = x.call{value: 10}("calldata");
        if (!success) {
            revert();
        }
    }

    modifier mod1(address x) {
        callAndSendNativeEth(x);
        _;
    }

    // Start Here
    function func1(address x) external mod1(x) {
        
    }

}



// BAD 
contract SendEtherNoChecks4 {

    uint256 public constant BAL = 100;

    function transferBalance(address x) internal {
        payable(x).transfer(BAL);
    }

    modifier mod1(address x) {
        transferBalance(x);
        
        _;
    }

    // Start Here
    function func1(address x) external mod1(x) {
        
    }

}

// BAD
contract SendEtherNoChecks5 {

    uint256 public constant BAL = 100;

    function sendBalance(address x) internal {
        (bool success) = payable(x).send(BAL);
        require(success, "Unable to send balance");
    }

    modifier mod1(address x) {
        sendBalance(x);
        _;
    }

    // Start Here
    function func1(address x) external mod1(x) {
        
    }

}
