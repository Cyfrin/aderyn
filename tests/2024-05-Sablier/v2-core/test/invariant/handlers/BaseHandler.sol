// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { StdCheats } from "forge-std/src/StdCheats.sol";

import { Constants } from "../../utils/Constants.sol";
import { Fuzzers } from "../../utils/Fuzzers.sol";
import { TimestampStore } from "../stores/TimestampStore.sol";

/// @notice Base contract with common logic needed by all handler contracts.
abstract contract BaseHandler is Constants, Fuzzers, StdCheats {
    /*//////////////////////////////////////////////////////////////////////////
                                    STATE-VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Maximum number of streams that can be created during an invariant campaign.
    uint256 internal constant MAX_STREAM_COUNT = 100;

    /// @dev Maps function names to the number of times they have been called.
    mapping(string func => uint256 calls) public calls;

    /// @dev The total number of calls made to this contract.
    uint256 public totalCalls;

    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Default ERC-20 asset used for testing.
    IERC20 public asset;

    /// @dev Reference to the timestamp store, which is needed for simulating the passage of time.
    TimestampStore public timestampStore;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(IERC20 asset_, TimestampStore timestampStore_) {
        asset = asset_;
        timestampStore = timestampStore_;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     MODIFIERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Simulates the passage of time. The time jump is upper bounded so that streams don't settle too quickly.
    /// See https://github.com/foundry-rs/foundry/issues/4994.
    /// @param timeJumpSeed A fuzzed value needed for generating random time warps.
    modifier adjustTimestamp(uint256 timeJumpSeed) {
        uint256 timeJump = _bound(timeJumpSeed, 2 minutes, 40 days);
        timestampStore.increaseCurrentTimestamp(timeJump);
        vm.warp(timestampStore.currentTimestamp());
        _;
    }

    /// @dev Checks user assumptions.
    modifier checkUsers(address sender, address recipient, address broker) {
        // The protocol doesn't allow the sender, recipient or broker to be the zero address.
        if (sender == address(0) || recipient == address(0) || broker == address(0)) {
            return;
        }

        // Prevent the contract itself from playing the role of any user.
        if (sender == address(this) || recipient == address(this) || broker == address(this)) {
            return;
        }

        _;
    }

    /// @dev Records a function call for instrumentation purposes.
    modifier instrument(string memory functionName) {
        calls[functionName]++;
        totalCalls++;
        _;
    }

    /// @dev Makes the provided sender the caller.
    modifier useNewSender(address sender) {
        resetPrank(sender);
        _;
    }
}
