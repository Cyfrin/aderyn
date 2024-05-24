// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { ISablierV2LockupDynamic } from "src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "src/interfaces/ISablierV2LockupLinear.sol";

import { Fork_Test } from "./Fork.t.sol";

contract NFTDescriptor_Fork_Test is Fork_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    IERC20 internal constant DAI = IERC20(0x6B175474E89094C44Da98b954EedeAC495271d0F);
    address internal constant DAI_HOLDER = 0x66F62574ab04989737228D18C3624f7FC1edAe14;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor() Fork_Test(DAI, DAI_HOLDER) { }

    /*//////////////////////////////////////////////////////////////////////////
                                      MODIFIERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Loads the Lockup V2.0 contracts pre-deployed on Mainnet.
    modifier loadDeployments_V2_0() {
        lockupDynamic = ISablierV2LockupDynamic(0x39EFdC3dbB57B2388CcC4bb40aC4CB1226Bc9E44);
        lockupLinear = ISablierV2LockupLinear(0xB10daee1FCF62243aE27776D7a92D39dC8740f95);
        _;
    }

    /// @dev Loads the Lockup V2.1 contracts pre-deployed on Mainnet.
    modifier loadDeployments_V2_1() {
        lockupDynamic = ISablierV2LockupDynamic(0x7CC7e125d83A581ff438608490Cc0f7bDff79127);
        lockupLinear = ISablierV2LockupLinear(0xAFb979d9afAd1aD27C5eFf4E27226E3AB9e5dCC9);
        _;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Fork_Test.setUp();
    }

    /*//////////////////////////////////////////////////////////////////////////
                                   TEST FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev The following test checks whether the new NFT descriptor is compatible with Lockup Dynamic v2.0.
    ///
    /// Checklist:
    /// - It should expect a call to {ISablierV2LockupDynamic.tokenURI}.
    /// - The test would fail if the call to {ISablierV2LockupDynamic.tokenURI} reverts.
    ///
    /// Given enough fuzz runs, all the following scenarios will be fuzzed:
    /// - Multiple values of streamId.
    function testForkFuzz_TokenURI_LockupDynamic_V2_0(uint256 streamId) external loadDeployments_V2_0 {
        streamId = _bound(streamId, 1, lockupDynamic.nextStreamId() - 1);

        // Set the new NFT descriptor for the previous version of Lockup Dynamic.
        resetPrank({ msgSender: lockupDynamic.admin() });
        lockupDynamic.setNFTDescriptor(nftDescriptor);

        // Expects a successful call to the new NFT Descriptor.
        vm.expectCall({
            callee: address(nftDescriptor),
            data: abi.encodeCall(nftDescriptor.tokenURI, (lockupDynamic, streamId)),
            count: 1
        });

        // Generate the token URI using the new NFT Descriptor.
        lockupDynamic.tokenURI(streamId);
    }

    /// @dev The following test checks whether the new NFT descriptor is compatible with Lockup Dynamic v2.1.
    ///
    /// Checklist:
    /// - It should expect a call to {ISablierV2LockupDynamic.tokenURI}.
    /// - The test would fail if the call to {ISablierV2LockupDynamic.tokenURI} reverts.
    ///
    /// Given enough fuzz runs, all the following scenarios will be fuzzed:
    /// - Multiple values of streamId.
    function testForkFuzz_TokenURI_LockupDynamic_V2_1(uint256 streamId) external loadDeployments_V2_1 {
        streamId = _bound(streamId, 1, lockupDynamic.nextStreamId() - 1);

        // Set the new NFT descriptor for the previous version of Lockup Dynamic.
        resetPrank({ msgSender: lockupDynamic.admin() });
        lockupDynamic.setNFTDescriptor(nftDescriptor);

        // Expects a successful call to the new NFT Descriptor.
        vm.expectCall({
            callee: address(nftDescriptor),
            data: abi.encodeCall(nftDescriptor.tokenURI, (lockupDynamic, streamId)),
            count: 1
        });

        // Generate the token URI using the new NFT Descriptor.
        lockupDynamic.tokenURI(streamId);
    }

    /// @dev The following test checks whether the new NFT descriptor is compatible with Lockup Linear v2.0.
    ///
    /// Checklist:
    /// - It should expect a call to {ISablierV2LockupLinear.tokenURI}.
    /// - The test would fail if the call to {ISablierV2LockupLinear.tokenURI} reverts.
    ///
    /// Given enough fuzz runs, all the following scenarios will be fuzzed:
    /// - Multiple values of streamId.
    function testForkFuzz_TokenURI_LockupLinear_V2_0(uint256 streamId) external loadDeployments_V2_0 {
        streamId = _bound(streamId, 1, lockupLinear.nextStreamId() - 1);

        // Set the new NFT descriptor for the previous version of Lockup Linear.
        resetPrank({ msgSender: lockupLinear.admin() });
        lockupLinear.setNFTDescriptor(nftDescriptor);

        // Expects a successful call to the new NFT Descriptor.
        vm.expectCall({
            callee: address(nftDescriptor),
            data: abi.encodeCall(nftDescriptor.tokenURI, (lockupLinear, streamId)),
            count: 1
        });

        // Generate the token URI using the new NFT Descriptor.
        lockupLinear.tokenURI(streamId);
    }

    /// @dev The following test checks whether the new NFT descriptor is compatible with Lockup Linear v2.1.
    ///
    /// Checklist:
    /// - It should expect a call to {ISablierV2LockupLinear.tokenURI}.
    /// - The test would fail if the call to {ISablierV2LockupLinear.tokenURI} reverts.
    ///
    /// Given enough fuzz runs, all the following scenarios will be fuzzed:
    /// - Multiple values of streamId.
    function testForkFuzz_TokenURI_LockupLinear_V2_1(uint256 streamId) external loadDeployments_V2_1 {
        streamId = _bound(streamId, 1, lockupLinear.nextStreamId() - 1);

        // Set the new NFT descriptor for the previous version of Lockup Linear.
        resetPrank({ msgSender: lockupLinear.admin() });
        lockupLinear.setNFTDescriptor(nftDescriptor);

        // Expects a successful call to the new NFT Descriptor.
        vm.expectCall({
            callee: address(nftDescriptor),
            data: abi.encodeCall(nftDescriptor.tokenURI, (lockupLinear, streamId)),
            count: 1
        });

        // Generate the token URI using the new NFT Descriptor.
        lockupLinear.tokenURI(streamId);
    }
}
