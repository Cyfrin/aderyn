// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

abstract contract WrongOrderOfLayout {
    function divide(int256 numerator, int256 denominator) external virtual returns (uint256);

    struct Allowed {
      bool isEven;
    }

    uint256 public multiplier;

    error DivideByZero();

    event Divided();

    modifier isZero(uint256 value) {
      _;
    }
}