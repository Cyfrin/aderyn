// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract DelegatecallWithoutAddressChecks {

    // Any delegate calls to addresses in state variables, we don't check!
    // Because we assume that other detectors catch setting of variables 
    // either in the constructor or in other methods with modifiers.
    address public manager;

    constructor() {
    
    }

    function delegate1(address to, bytes memory data) external {
        to.delegatecall(data); // `to` is not protected, therefore BAD
    }

    function delegate2(bytes memory data) external {
        manager.delegatecall(data); // `manager` is state variable, therefore GOOD
    }

    modifier isAllowed(address to) {
        address[3] memory allowed = [address(1), address(2), address(3)];
        bool isLegit = false;
        for (uint256 i = 0; i < 3; i++) {
            if (allowed[i] == to) {
                isLegit = true;
            }
        }
        require(isLegit);
        _;
    }

    function delegate3(address to, bytes memory data) external isAllowed(to) {
        to.delegatecall(data); // `to` is protected, therefore GOOD
    } 

    // Known false negative
    function delegate4(address to, bytes memory data, address x) external {
        if (x != address(0)) { 
            to.delegatecall(data);
            // Although `to` is not protected, for now, we assume it is because
            // there is a binary operation with `x` involved which is of type address.
            // |---> So, we assume `to` (which also of type address is vetted)
            // 
            // This is limitation of using callgraphs. Later, when we add support for CFG
            // we can do a proper data dependency analysis.
        }
    } 

}