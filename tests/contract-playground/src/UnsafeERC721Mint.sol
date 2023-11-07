// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {ERC721} from "../lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";

contract UnsafeERC721Mint is ERC721 {
    constructor() ERC721("UnsafeERC721Mint", "U721M") {}

    function unsafeMint(address to, uint256 tokenId) external {
        _mint(to, tokenId);
    }

    function betterMint(address to, uint256 tokenId) external {
        _safeMint(to, tokenId);
    }
}