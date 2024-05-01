// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract MissingEvent {
    uint256 public startDate;
    address public owner;

    event StartDateUpdated(uint256 indexed oldDate, uint256 indexed newDate);
    event OwnerUpdated(address indexed previousOwner, address indexed newOwner);

    constructor(uint256 _startDate, address _owner) {
      startDate = _startDate;
      owner = _owner;
    }

    function updateStartDate(uint256 _startDate) public {
      uint256 currentStartDate = startDate;
      startDate = _startDate;
      emit StartDateUpdated(currentStartDate, startDate);
    }

    function changeOwner(address _newOwner) public {
      owner = _newOwner;
    }
}
