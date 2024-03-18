// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract EmptyBlocks {
    // Good
    constructor() {
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