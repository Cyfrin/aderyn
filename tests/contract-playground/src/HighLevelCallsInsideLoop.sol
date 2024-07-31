// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

contract HighLevelCallsInLoop {
    address payable[] destinations;

    function bad() external {
        // BAD for loop (the fallback may revert causing DoS)
        for (uint i = 0; i < destinations.length; i++) {
            destinations[i].transfer(i);
        }
    }

    function bad2() external {
        // BAD for loop (the fallback may revert causing DoS)
        for (uint i = 0; i < destinations.length; i++) {
            facilitateTransfer(i, i * 2);
        }
    }

    function facilitateTransfer(uint256 index, uint256 money) internal {
        destinations[index].transfer(money);
    }

    function bad3() external view {
        // BAD for loop
        for (uint i = 0; i < destinations.length; i++) {
            SimplyRevert(destinations[i]).sayHello();
        }
    }

    function goodButTreatedAsBad() external view {
        // BAD for loop
        // NOTE - This is actually good, but we still catch it (False Positive).
        // The way around it is to perform this function call with low level
        // call and then you can just break off the loop if the bool success returns false!
        for (uint i = 0; i < destinations.length; i++) {
            SimplyRevert(destinations[i]).innocent();
        }
    }
}

contract SimplyRevert {
    error HellNo(string);

    function sayHello() external pure {
        revert();
    }

    function sayHellNo() external pure {
        revert HellNo("HellNo instead of Hello");
    }

    function innocent() external pure {}
}
