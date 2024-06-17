// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

contract DemoNewASTNodes {

    modifier iHaveAPlaceholder(address to) {
        require(to != address(0));
        // CAPTURE 1 instance of `PlaceholderStatement`
        _;
    }

    function useBreakAndContinueStatement(address x) internal pure iHaveAPlaceholder(x) returns(uint256 sum) {

        for (uint256 i = 0; i < 10000000; ++i) {
            if (i == 3) {
                // CAPTURE 1 instance of `Continue`
                continue;
            }

            if (i == 5) {
                // CAPTURE 1 instance of `Break`
                break;
            }

            sum++;
        }
        
    }

    function calculateSumUsingDoWhilwLoop() internal pure returns(uint256 sum) {
        uint256[] memory numbers = new uint256[](5);
        numbers[0] = 0;
        numbers[1] = 1;
        numbers[2] = 2;
        numbers[3] = 3;
        numbers[4] = 4;

        int256 i = 4; 
        
        // CAPTURE 1 instance of `DoWhileStatement`
        do {
            sum += numbers[uint256(i)];
            i--;
        } while(i >= 0);
    } 

}