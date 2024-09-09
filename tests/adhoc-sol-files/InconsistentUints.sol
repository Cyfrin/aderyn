pragma solidity ^0.8.24;

contract InconsistentStateVariablesContract {
    // Unsigned integer variables
    uint public uintVariable; // 1
    uint256 public uint256Variable; // 1
    int public intVariable; // 1
    int256 public int256Variable; // 1 

    struct Person {
        uint personUint; // 2
        mapping (uint => uint256) personMap; // 3 2
    }

    uint[] public uintArray; // 4
    mapping(uint256 => uint other) u2uMapping; // 5 3

    // Constructor to initialize the state variables
    constructor(uint _uintInitial, uint256 _uint256Initial) { // 6 4
        uintVariable = _uintInitial;
        uint256Variable = _uint256Initial;
    }

}
