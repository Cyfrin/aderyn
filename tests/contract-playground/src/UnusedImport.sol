// SPDX-License-Identifier: MIT

pragma solidity 0.8.19;

// BAD (this import is not used)
import "./U2.sol";

// BAD (this import is not used)
import {U3} from "./U3.sol";

// GOOD This imports U5 and we use it in this contract
import {U5} from "./U4.sol";
import "./U4.sol";

contract UnusedImport {
    function helpMe(U5.SIU5 memory structInsideU5) external {
        this.helpMe(structInsideU5); // to avoid empty block detector 
    }
}

