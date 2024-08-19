// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract VoidConstructor {}

contract FilledConstructor {
    constructor() {
        /* some stuff */    
    }
}

// This contracts think VoidConstructor has a defined constructor and it tries to call it
contract B is VoidConstructor, FilledConstructor {
    // BAD invocation (1 instance)
    constructor() 
    VoidConstructor() 
    FilledConstructor() 
    {
        
    }
}

// BAD invocation (1 instance)
// NOTE - BUT WE CANNOT CATCH IT (because AST shows no difference between this and the below contract)
contract C is VoidConstructor() {
    constructor() {
        
    }
}

// GOOD
contract D is VoidConstructor {
    constructor() {
        
    }
}