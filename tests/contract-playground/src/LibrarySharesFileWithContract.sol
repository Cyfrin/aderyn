// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract IAmASimpleContract {
    using IAmASimpleLibrary for uint256;

    event ModValue(uint256 indexed a);

    constructor(uint256 a) {
        uint256 b = a.add_mod_4(a);
        emit ModValue(b);
    }
}

// In Solidity, libraries are typically designed to be backward compatible, so they require flexible
// pragma versions to support a wide range of contracts. By placing libraries in separate files with
// floating pragmas, you ensure that they remain adaptable to different contract versions, while
// contracts in other files can stick to fixed pragma versions for stability.

library IAmASimpleLibrary {
    function add_mod_4(uint256 a, uint256 b) external pure returns (uint256) {
        return (a + b) % 4;
    }
}
