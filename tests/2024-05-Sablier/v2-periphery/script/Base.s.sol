// SPDX-License-Identifier: GPL-3.0-or-later
// solhint-disable no-console
pragma solidity >=0.8.22 <0.9.0;

import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { Sphinx } from "@sphinx-labs/contracts/SphinxPlugin.sol";

import { console2 } from "forge-std/src/console2.sol";
import { Script } from "forge-std/src/Script.sol";
import { stdJson } from "forge-std/src/StdJson.sol";

contract BaseScript is Script, Sphinx {
    using Strings for uint256;
    using stdJson for string;

    /// @dev Included to enable compilation of the script without a $MNEMONIC environment variable.
    string internal constant TEST_MNEMONIC = "test test test test test test test test test test test junk";

    /// @dev The project name for the Sphinx plugin.
    string internal constant TEST_SPHINX_PROJECT_NAME = "test-test";

    /// @dev Needed for the deterministic deployments.
    bytes32 internal constant ZERO_SALT = bytes32(0);

    /// @dev The address of the transaction broadcaster.
    address internal broadcaster;

    /// @dev Used to derive the broadcaster's address if $EOA is not defined.
    string internal mnemonic;

    /// @dev The project name for the Sphinx plugin.
    string internal sphinxProjectName;

    /// @dev Initializes the transaction broadcaster like this:
    ///
    /// - If $EOA is defined, use it.
    /// - Otherwise, derive the broadcaster address from $MNEMONIC.
    /// - If $MNEMONIC is not defined, default to a test mnemonic.
    /// - If $SPHINX_PROJECT_NAME is not defined, default to a test project name.
    ///
    /// The use case for $EOA is to specify the broadcaster key and its address via the command line.
    constructor() {
        address from = vm.envOr({ name: "EOA", defaultValue: address(0) });
        if (from != address(0)) {
            broadcaster = from;
        } else {
            mnemonic = vm.envOr({ name: "MNEMONIC", defaultValue: TEST_MNEMONIC });
            (broadcaster,) = deriveRememberKey({ mnemonic: mnemonic, index: 0 });
        }
        sphinxProjectName = vm.envOr({ name: "SPHINX_PROJECT_NAME", defaultValue: TEST_SPHINX_PROJECT_NAME });
    }

    modifier broadcast() {
        vm.startBroadcast(broadcaster);
        _;
        vm.stopBroadcast();
    }

    /// @dev Configures the Sphinx plugin to manage the deployment of the contracts.
    /// Refer to https://github.com/sphinx-labs/sphinx/tree/main/docs.
    ///
    /// CLI example:
    /// bun sphinx propose script/DeploybatchLockup.s.sol --networks testnets --sig "runSphinx()"
    function configureSphinx() public override {
        sphinxConfig.mainnets = ["arbitrum", "avalanche", "base", "bnb", "gnosis", "ethereum", "optimism", "polygon"];
        sphinxConfig.orgId = vm.envOr({ name: "SPHINX_ORG_ID", defaultValue: TEST_MNEMONIC });
        sphinxConfig.owners = [broadcaster];
        sphinxConfig.projectName = sphinxProjectName;
        sphinxConfig.testnets = ["sepolia"];
        sphinxConfig.threshold = 1;
    }

    /// @dev The presence of the salt instructs Forge to deploy contracts via this deterministic CREATE2 factory:
    /// https://github.com/Arachnid/deterministic-deployment-proxy
    ///
    /// Notes:
    /// - The salt format is "ChainID <chainid>, Version <version>".
    /// - The version is obtained from `package.json`.
    function constructCreate2Salt() public view returns (bytes32) {
        string memory chainId = block.chainid.toString();
        string memory json = vm.readFile("package.json");
        string memory version = json.readString(".version");
        string memory create2Salt = string.concat("ChainID ", chainId, ", Version ", version);
        console2.log("The CREATE2 salt is \"%s\"", create2Salt);
        return bytes32(abi.encodePacked(create2Salt));
    }
}
