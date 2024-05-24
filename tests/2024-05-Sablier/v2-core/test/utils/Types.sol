// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

struct Users {
    // Default admin for all Sablier V2 contracts.
    address payable admin;
    // Impartial user.
    address payable alice;
    // Default stream broker.
    address payable broker;
    // Malicious user.
    address payable eve;
    // Default NFT operator.
    address payable operator;
    // Default stream recipient.
    address payable recipient;
    // Default stream sender.
    address payable sender;
}
