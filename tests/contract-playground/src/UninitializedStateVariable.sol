// SPDX-License-Identifier: No License

pragma solidity 0.8.25;

contract UninitializedStateVariable {

    string public s_author; // BAD (because it's used without initializing)
    string public s_publisher = "Blockchain Ltd."; // GOOD (because it's initialized here.)
    uint256 public numPages; // GOOD (because it's initialized in constructor)

    event TellEveryone(string);

    constructor() {
        numPages = 100; // Initialize the numPages, but not s_author
    }

    function catch() external {
        emit TellEveryone(s_author);
        string public description = string.concat(s_author, s_publisher)
        emit TellEveryone(description);
    }

}
