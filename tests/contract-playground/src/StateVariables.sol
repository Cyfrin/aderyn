// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract StateVariables {
    // uint256 empty, static, private, internal, public
    uint256 private staticPrivateNumber;
    uint256 internal staticInternalNumber;
    uint256 public staticPublicNumber;

    // uint256 non-empty, static, private, internal, public
    uint256 private staticNonEmptyPrivateNumber = 1;
    uint256 internal staticNonEmptyInternalNumber = 2;
    uint256 public staticNonEmptyPublicNumber = 3;

    // uint256 empty private, internal, public and altered in functions
    uint256 private emptyAlteredPrivateNumber;
    uint256 internal emptyAlteredInternalNumber;
    uint256 public emptyAlteredPublicNumber;

    // uint256 non-empty private, internal, public and altered in functions
    uint256 private nonEmptyAlteredPrivateNumber = 1;
    uint256 internal nonEmptyAlteredInternalNumber = 2;
    uint256 public nonEmptyAlteredPublicNumber = 3;

    // uint256 constant, private, internal, public
    uint256 private constant PRIVATE_CONSTANT = 1;
    uint256 internal constant INTERNAL_CONSTANT = 2;
    uint256 public constant PUBLIC_CONSTANT = 3;

    // uint256 immutable, private, internal, public
    uint256 private immutable privateImmutableNumber;
    uint256 internal immutable internalImmutableNumber;
    uint256 public immutable publicImmutableNumber;

    constructor(uint256 _privateImmutableNumber, uint256 _internalImmutableNumber, uint256 _publicImmutableNumber) {
        privateImmutableNumber = _privateImmutableNumber;
        internalImmutableNumber = _internalImmutableNumber;
        publicImmutableNumber = _publicImmutableNumber;
    }

    function setEmptyAlteredNumbers(uint256 _emptyAlteredPrivateNumber, uint256 _emptyAlteredInternalNumber, uint256 _emptyAlteredPublicNumber) public {
        emptyAlteredPrivateNumber = _emptyAlteredPrivateNumber;
        emptyAlteredInternalNumber = _emptyAlteredInternalNumber;
        emptyAlteredPublicNumber = _emptyAlteredPublicNumber;
    }

    function setNonEmptyAlteredNumbers(uint256 _nonEmptyAlteredPrivateNumber, uint256 _nonEmptyAlteredInternalNumber, uint256 _nonEmptyAlteredPublicNumber) public {
        nonEmptyAlteredPrivateNumber = _nonEmptyAlteredPrivateNumber;
        nonEmptyAlteredInternalNumber = _nonEmptyAlteredInternalNumber;
        nonEmptyAlteredPublicNumber = _nonEmptyAlteredPublicNumber;
    }
}