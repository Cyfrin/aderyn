// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Check for collision of function signature

contract DomainSeparatorCollision1 {
    // GOOD (because this contract is not ERC20)
    function DOMAIN_SEPARATOR() external pure returns (bytes32) {
        return bytes32("Hello World!");
    }
}

// Our heuristic for determining if a contract is meant to be an ERC20 token is by looking at the
// linearized base contract chain and searching  for "ERC20" substring in any of the parent contract name
// Same heuristic is used for dangerous _mint detection where we try to look for ERC721 contract
abstract contract SomeERC20 {

}

contract DomainSeparatorCollision2 is SomeERC20 {
    // BAD
    function DOMAIN_SEPARATOR() external pure returns (bytes32) {
        return bytes32("Hello World!");
    }
}

contract DomainSeparatorCollision3 is SomeERC20 {
    // BAD
    uint256 public constant DOMAIN_SEPARATOR = 13813;
}

contract DomainSeparatorCollision4 is SomeERC20 {
    // BAD
    uint256 public DOMAIN_SEPARATOR = 13813;
}

contract DomainSeparatorCollision5 is SomeERC20 {
    struct D {
        uint256 a;
    }

    // BAD
    D public DOMAIN_SEPARATOR;

    function whyBad() external view {
        this.DOMAIN_SEPARATOR();
    }
}

contract DomainSeparatorCollision6 is SomeERC20 {
    // GOOD
    uint256[] public DOMAIN_SEPARATOR;

    // Below way of accessing without passing an argument is not possible - so it's a different signature
    // function whyBad() external view {
    //     this.DOMAIN_SEPARATOR();
    // }
}

contract DomainSeparatorCollision7 is SomeERC20 {
    // GOOD
    mapping(uint256 => bool) public DOMAIN_SEPARATOR;

    // Below way of accessing without passing an argument is not possible - so it's a different signature
    // function whyBad() external view {
    //     this.DOMAIN_SEPARATOR();
    // }
}

contract DomainSeparatorCollision8 is SomeERC20 {
    // GOOD (function signature doesn't exactly match because of the parameter `j` here)
    function DOMAIN_SEPARATOR(uint256 j) external pure returns (bytes32) {
        return bytes32(j);
    }
}
