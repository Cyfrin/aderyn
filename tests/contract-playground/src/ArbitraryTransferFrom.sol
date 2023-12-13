// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";

contract ArbitraryTransferFrom {
    using SafeERC20 for IERC20;

    IERC20 s_token;

    constructor(IERC20 token) {
        s_token = token;
    }

    function bad1(address from, address to, uint256 amount) internal {
        s_token.transferFrom(from, to, amount);
    }

    function bad2(address from, address to, uint256 amount) external {
        s_token.safeTransferFrom(from, to, amount);
    }    
    
    function bad3(address from, address to, uint256 amount) external {
        SafeERC20.safeTransferFrom(s_token, from, to, amount);
    }

    // ArbitraryTransferFromDetector has a false positive here
    function good1(address to, uint256 am) public {
        address from_msgsender = msg.sender;
        s_token.transferFrom(from_msgsender, to, am);
    }

    function good2(address to, uint256 amount) external {
        s_token.safeTransferFrom(msg.sender, to, amount);
    }

    function good3(address to, uint256 amount) external {
        SafeERC20.safeTransferFrom(s_token, msg.sender, to, amount);
    }

    function good4(address to, uint256 amount) external {
        SafeERC20.safeTransferFrom(s_token, address(this), to, amount);
    }

    function good5(address from, address to, uint256 amount) external {
        s_token.safeTransferFrom(address(this), to, amount);
    }

    function good6(address to, uint256 amount) external {
        s_token.transferFrom(msg.sender, to, amount);
    }

}