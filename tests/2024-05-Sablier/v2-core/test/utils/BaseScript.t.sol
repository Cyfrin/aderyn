// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { StdAssertions } from "forge-std/src/StdAssertions.sol";

import { BaseScript } from "script/Base.s.sol";

contract BaseScript_Test is StdAssertions {
    using Strings for uint256;

    BaseScript internal baseScript;

    function setUp() public {
        baseScript = new BaseScript();
    }

    function test_ConstructCreate2Salt() public view {
        string memory chainId = block.chainid.toString();
        string memory version = "1.1.2";
        string memory salt = string.concat("ChainID ", chainId, ", Version ", version);

        bytes32 actualSalt = baseScript.constructCreate2Salt();
        bytes32 expectedSalt = bytes32(abi.encodePacked(salt));
        assertEq(actualSalt, expectedSalt, "CREATE2 salt mismatch");
    }
}
