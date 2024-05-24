// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { StdCheats } from "forge-std/src/StdCheats.sol";

import { ISablierV2LockupDynamic } from "../../src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "../../src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "../../src/interfaces/ISablierV2LockupTranched.sol";
import { ISablierV2NFTDescriptor } from "../../src/interfaces/ISablierV2NFTDescriptor.sol";

abstract contract DeployOptimized is StdCheats {
    /// @dev Deploys {SablierV2LockupDynamic} from an optimized source compiled with `--via-ir`.
    function deployOptimizedLockupDynamic(
        address initialAdmin,
        ISablierV2NFTDescriptor nftDescriptor_,
        uint256 maxSegmentCount
    )
        internal
        returns (ISablierV2LockupDynamic)
    {
        return ISablierV2LockupDynamic(
            deployCode(
                "out-optimized/SablierV2LockupDynamic.sol/SablierV2LockupDynamic.json",
                abi.encode(initialAdmin, address(nftDescriptor_), maxSegmentCount)
            )
        );
    }

    /// @dev Deploys {SablierV2LockupLinear} from an optimized source compiled with `--via-ir`.
    function deployOptimizedLockupLinear(
        address initialAdmin,
        ISablierV2NFTDescriptor nftDescriptor_
    )
        internal
        returns (ISablierV2LockupLinear)
    {
        return ISablierV2LockupLinear(
            deployCode(
                "out-optimized/SablierV2LockupLinear.sol/SablierV2LockupLinear.json",
                abi.encode(initialAdmin, address(nftDescriptor_))
            )
        );
    }

    /// @dev Deploys {SablierV2LockupTranched} from an optimized source compiled with `--via-ir`.
    function deployOptimizedLockupTranched(
        address initialAdmin,
        ISablierV2NFTDescriptor nftDescriptor_,
        uint256 maxTrancheCount
    )
        internal
        returns (ISablierV2LockupTranched)
    {
        return ISablierV2LockupTranched(
            deployCode(
                "out-optimized/SablierV2LockupTranched.sol/SablierV2LockupTranched.json",
                abi.encode(initialAdmin, address(nftDescriptor_), maxTrancheCount)
            )
        );
    }

    /// @dev Deploys {SablierV2NFTDescriptor} from an optimized source compiled with `--via-ir`.
    function deployOptimizedNFTDescriptor() internal returns (ISablierV2NFTDescriptor) {
        return
            ISablierV2NFTDescriptor(deployCode("out-optimized/SablierV2NFTDescriptor.sol/SablierV2NFTDescriptor.json"));
    }

    function deployOptimizedCore(
        address initialAdmin,
        uint256 maxSegmentCount,
        uint256 maxTrancheCount
    )
        internal
        returns (
            ISablierV2LockupDynamic lockupDynamic_,
            ISablierV2LockupLinear lockupLinear_,
            ISablierV2LockupTranched lockupTranched_,
            ISablierV2NFTDescriptor nftDescriptor_
        )
    {
        nftDescriptor_ = deployOptimizedNFTDescriptor();
        lockupDynamic_ = deployOptimizedLockupDynamic(initialAdmin, nftDescriptor_, maxSegmentCount);
        lockupLinear_ = deployOptimizedLockupLinear(initialAdmin, nftDescriptor_);
        lockupTranched_ = deployOptimizedLockupTranched(initialAdmin, nftDescriptor_, maxTrancheCount);
    }
}
