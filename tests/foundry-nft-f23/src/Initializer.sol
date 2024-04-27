// SPDX-License-Identifier: MIT

pragma solidity 0.8.25;

import "@oz/contracts/token/ERC721/ERC721.sol";

contract Initializer {
    function get_start_token_id() public pure returns(uint256) {
        return 10;
    }
}