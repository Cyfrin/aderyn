// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {Ownable} from "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";

contract EmptyBlocksWithOwnableConstructor is Ownable {
    // Good
    constructor() Ownable() {

    }
}

contract EmptyBlocksWithEmptyConstructor {
    // Good
    constructor() {

    }
}

contract EmptyBlocksNestedInReceiverAndFallbacks {

    // Good
    constructor() {

    }

     // Bad
    receive() external payable {
        // Empty block
        {

        }
    }

    // Bad
    fallback() external payable {
        // Empty block
        {

        }
    }
}

contract EmptyBlocks {
    // Bad
    constructor() {
        emit EmptyEvent();
        // Empty Block Below
        {

        }
    }

    // Good
    receive() external payable {
    }

    // Good
    fallback() external payable {
    }

    // Bad
    function emptyFunction() external {
    }

    // Bad
    function emptyFunctionWithComment() external {
        // This is a comment
    }

    event EmptyEvent();
    // Bad
    function emptyBlockInsideNormalFunction() external {
        emit EmptyEvent();
        // Empty block below
        {
        }
    }

    // Bad
    function emptyBlockWithCommentInsideNormalFunction() external {
        emit EmptyEvent();
        // Empty block below
        {
            // This is a comment
        }
    }

}