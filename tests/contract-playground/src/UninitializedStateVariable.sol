// SPDX-License-Identifier: No License

pragma solidity 0.8.25;

contract UninitializedStateVariable {

    string public s_author; // BAD (because it's used without initializing)
    string public s_publisher = "Blockchain Ltd."; // GOOD (because it's initialized here.)
    uint256 public numPages; // GOOD (because it's initialized in constructor)

    // For arrays and mappings, it's okay to use them without initializing
    uint256[] public arr; // GOOD
    mapping(uint256 => uint256[]) private map; // GOOD

    address destination; // BAD

    function transfer() payable public {
        payable(destination).transfer(msg.value); // `destination` does not have any assignments.
    }

    event TellEveryone(string);

    constructor() {
        numPages = 100; // Initialize the numPages, but not s_author
    }

    function tell() external {
        emit TellEveryone(s_author);
        string memory description = string.concat(s_author, s_publisher);
        emit TellEveryone(description);
    }

}


contract UninitializedStateVariableBase {
    uint256 public myVar; // initialized in extension, hence not captured
}

contract UninitializedStateVariableExtension is UninitializedStateVariableBase {
    constructor() {
        myVar = 4;
    }
}