pragma solidity 0.4.22;

contract BooleanEquality {
    function badCheck(bool isEven) external pure returns (uint256) {
        if (isEven == true) {
            return 100;
        }
        return 0;
    }

    function badCheck2(bool isEven) external pure returns (uint256) {
        if (isEven == !true) {
            return 200;
        }
        return 130;
    }

    function badCheck3(bool isEven) external pure returns (uint256) {
        if (isEven == false) {
            return 100;
        }
        return 0;
    }

    function badCheck4(bool isEven) external pure returns (uint256) {
        if (isEven == !false) {
            return 200;
        }
        return 130;
    }
}
