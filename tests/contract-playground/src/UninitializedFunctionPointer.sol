// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract UninitalizedFunctionPointers {

  constructor() {
    function(uint256) internal funcPtr; // VariableDeclaration of type name type string starting with "function"
    funcPtr(10);

    function(uint256) internal funcPtr2;
    funcPtr2 = assignee; // Assignment' LHS has reference declaration pointing to funcPtr2
    funcPtr2(10);

    function(uint256) internal funcPtr3 = assignee; 
    funcPtr3(10);
  }

  // CAVEAT
  // In the current setup we wouldn't be able to detect if a func ptr was assigned to, after making a function call
  // (Of course, if it's in the same block we can use `appearsAfter` and `appearsBefore` but if it's call chain, we don't
  // yet have a sense of sequence)

  function assignee(uint256 a) internal {}

}