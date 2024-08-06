// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

// COPIED THIS EXAMPLE FROM SLITHER WIKI

//Modified from https://github.com/nomad-xyz/ExcessivelySafeCall
contract BadGuy {
    function youveActivateMyTrapCard() external pure returns (bytes memory) {
        assembly {
            revert(0, 1000000)
        }
    }
}

contract Mark {
    function oops(address badGuy) public {
        bool success;
        bytes memory ret;

        // Mark pays a lot of gas for this copy
        // BAD (There is no gas limit, so decoding unknown amounts of return data may cause OOG)c
        (success, ret) = badGuy.call(
            abi.encodeWithSelector(BadGuy.youveActivateMyTrapCard.selector)
        );

        // Mark may OOG here, preventing local state changes
        //importantCleanup();
    }
}

contract Mark2 {
    function oops(address badGuy) public {
        bool success;
        bytes memory ret;

        // GOOD (we have a gas limit so that code after this call may also be executed)
        (success, ret) = badGuy.call{gas: 1000}(
            abi.encodeWithSelector(BadGuy.youveActivateMyTrapCard.selector)
        );

        // Mark may OOG here, preventing local state changes
        //importantCleanup();
    }
}

contract Mark3 {
    function oops(address badGuy) public {
        bool success;

        // GOOD (we don't do returndatacopy)
        (success, ) = badGuy.call(
            abi.encodeWithSelector(BadGuy.youveActivateMyTrapCard.selector)
        );

        // Mark may OOG here, preventing local state changes
        //importantCleanup();
    }
}

contract Mark4 {
    address goodGuy;

    function oops() public {
        bool success;
        bytes memory ret;

        // Mark pays a lot of gas for this copy
        // GOOD (There is no gas limit, but we're sending a call to a state variable address so it must be safe)
        (success, ret) = goodGuy.call(
            abi.encodeWithSelector(BadGuy.youveActivateMyTrapCard.selector)
        );

        // Mark may OOG here, preventing local state changes
        //importantCleanup();
    }
}
