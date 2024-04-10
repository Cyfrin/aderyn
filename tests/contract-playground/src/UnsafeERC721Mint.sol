// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {ERC721} from "../lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import {ERC20} from "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract UnsafeERC721Mint is ERC721 {
    constructor() ERC721("UnsafeERC721Mint", "U721M") {}

    // Bad
    function unsafeMint(address to, uint256 tokenId) external {
        _mint(to, tokenId);
    }

    // Good
    function betterMint(address to, uint256 tokenId) external {
        _safeMint(to, tokenId);
    }
}

contract SafeERC20Mint is ERC20 {
    constructor() ERC20("SafeERC20Mint", "S20M") {}

    // Good
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}