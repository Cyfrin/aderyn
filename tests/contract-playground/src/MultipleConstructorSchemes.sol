pragma solidity 0.4.22;

contract MultipleConstructorSchemes {
    uint public x;

    constructor() public {
        x = 1;
    }

    function MultipleConstructorSchemes() public {
        x = 2;
    }
}