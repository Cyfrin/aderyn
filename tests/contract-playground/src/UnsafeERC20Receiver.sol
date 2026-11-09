// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;
interface IReceiverNFT {
    function approve(address spender, uint256 id) external;
    function transferFrom(address from, address to, uint256 id) external;
}
interface IReceiverERC20 {
    function approve(address spender, uint256 value) external returns (bool);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
}
contract UnsafeERC20Receiver {
    function moveEther(address payable to) external payable {
        to.transfer(msg.value);
    }

    function approveNft(IReceiverNFT nft, address to, uint256 id) external {
        nft.approve(to, id);
    }

    function moveNft(IReceiverNFT nft, address to, uint256 id) external {
        nft.transferFrom(msg.sender, to, id);
    }

    function tokenOps(IReceiverERC20 token, address to, uint256 value) external {
        token.approve(to, value);
        token.transfer(to, value);
        token.transferFrom(msg.sender, to, value);
    }
}
