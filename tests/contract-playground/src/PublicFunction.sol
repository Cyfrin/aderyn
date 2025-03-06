// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

interface IERC165 {
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
}

contract BaseContract is IERC165 {
    function supportsInterface(bytes4 interfaceId) public virtual view returns (bool) {
        return interfaceId == type(IERC165).interfaceId;
    }
}

interface SecondInterface {}

contract PublicFunction is BaseContract, SecondInterface {
    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return interfaceId == type(SecondInterface).interfaceId || super.supportsInterface(interfaceId);
    }
}