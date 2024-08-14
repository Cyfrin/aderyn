// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

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

    function innocent() external pure {}
}