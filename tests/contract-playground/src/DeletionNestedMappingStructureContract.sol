// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;


contract NestedMappingBalanceStructure {
    
    struct Person {
        uint256[] names;
        mapping(address => uint256) age;
    }

    mapping(address => Person) private people;

    function remove() internal{
        // We are deleting from a mapping whose value is a struct which contains a member of type mapping.
        // Therefore, capture it!
        delete people[msg.sender];
    }

}