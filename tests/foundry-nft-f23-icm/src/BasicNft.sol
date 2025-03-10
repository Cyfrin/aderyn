// SPDX-License-Identifier: MIT

pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "icm/ICM.sol"; // icm is remapped to src/inner-core-modules
import "./Initializer.sol";

contract BasicNft is ICM, ERC721 {

    uint256 public s_tokenId;

    constructor() ERC721(PROJECT_NAME, PROJECT_SYMBOL) {
        Initializer initializer = new Initializer();
        s_tokenId = initializer.get_start_token_id();
    }

    function mint() public {
        _safeMint(msg.sender, s_tokenId);
        s_tokenId++;
    }

}