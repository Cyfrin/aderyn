// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

function make() pure {
}

// Not used  (should be captured)
error NotNice();

// Used in below function (should not be captured)
error OutsideError();

function iLiveOutsideContracts() {
    revert OutsideError();
}