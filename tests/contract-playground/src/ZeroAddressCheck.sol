// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

interface IERC20 {
    function transfer(address recipient, uint256 amount) external returns (bool);
}

contract ZeroAddressCheck {
    address public addr;
    IERC20 public token;

    function goodAddress1(address newAddr) public {
        if (newAddr == address(0)) revert();
        addr = newAddr;
    }

    function goodAddress2(address newAddr) public {
        require(newAddr != address(0), "Address cannot be zero");
        addr = newAddr;
    }

    function goodToken1(address newAddr) public {
        if (newAddr == address(0)) revert();
        token = IERC20(newAddr);
    }

    function goodToken2(address newAddr) public {
        require(newAddr != address(0), "Address cannot be zero");
        token = IERC20(newAddr);
    }

    function goodToken3(IERC20 newToken) public {
        require(address(newToken) != address(0), "Address cannot be zero");
        token = newToken;
    }

    function goodToken4(IERC20 newToken) public {
        if (address(newToken) == address(0)) revert();
        token = newToken;
    }

    function bad1(address newAddr) public {
        addr = newAddr;
    }

    function bad2(IERC20 newToken) public {
        token = newToken;
    }

    // False negative
    function bad3(address newAddr) public {
        token = IERC20(newAddr);
    }
}