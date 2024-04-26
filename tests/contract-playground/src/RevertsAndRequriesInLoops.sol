// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

contract RevertsAndRequiresInLoops {

    // Maximum of 10 ids allowed
    mapping(uint256 id => uint256 amt) funds;

    function deductMassFundsV1() external {
        for (uint256 id = 0; id < 10; ++id) {
            require(funds[id] != 0, "Empty funds");
            delete funds[id];
        }
    }

    function deductMassFundsV2() external {
        for (uint256 id = 0; id < 10; ++id) {
            if(funds[id] == 0) {
                revert();
            }
            delete funds[id];
        }
    }

     function deductMassFundsV3() external returns(uint256[] memory failedIds) {
        uint256[] memory failedIds = new uint256[](10);
        for (uint256 id = 0; id < 10; ++id) {
            if(funds[id] == 0) {
                failedIds[id] = 1;
            }
            delete funds[id];
        }
    }

}