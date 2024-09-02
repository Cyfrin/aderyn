// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract AderynCustomIgnore {

    // This will be reported by empty-block and useless-public-function 
    function f1() public {

    }


    // This will be not reported by either of the detectors
    // aderyn-ignore-next-line(useless-public-function ,    empty-block)
    function f2() public {

    }

    // This will never be reported
    // aderyn-ignore-next-line
    function f3() public {

    }

    // This will be reported only by useless-public-function 
    // aderyn-ignore-next-line (empty-block)
    function f4() public {

    }

}
