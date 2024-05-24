// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";
import { Broker } from "src/types/DataTypes.sol";

import { Base_Test } from "test/Base.t.sol";

/// @dev This contracts avoids duplicating test logic for {SablierV2LockupLinear} and {SablierV2LockupDynamic};
/// both of these contracts inherit from {SablierV2Lockup}.
abstract contract Lockup_Integration_Shared_Test is Base_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev A test contract that is meant to be overridden by the implementing contract, which will be
    /// either {SablierV2LockupLinear} or {SablierV2LockupDynamic}.
    ISablierV2Lockup internal lockup;

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        // Make the Sender the default caller in this test suite.
        resetPrank({ msgSender: users.sender });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Creates the default stream.
    function createDefaultStream() internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream but make it not cancelable.
    function createDefaultStreamNotCancelable() internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the NFT transfer disabled.
    function createDefaultStreamNotTransferable() internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided address.
    function createDefaultStreamWithAsset(IERC20 asset) internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided broker.
    function createDefaultStreamWithBroker(Broker memory broker) internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided end time.
    function createDefaultStreamWithEndTime(uint40 endTime) internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided user as the recipient and the sender.
    function createDefaultStreamWithIdenticalUsers(address user) internal virtual returns (uint256 streamId) {
        return createDefaultStreamWithUsers({ recipient: user, sender: user });
    }

    /// @dev Creates the default stream with the provided recipient.
    function createDefaultStreamWithRecipient(address recipient) internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided sender.
    function createDefaultStreamWithSender(address sender) internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided start time.
    function createDefaultStreamWithStartTime(uint40 startTime) internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided total amount.
    function createDefaultStreamWithTotalAmount(uint128 totalAmount) internal virtual returns (uint256 streamId);

    /// @dev Creates the default stream with the provided sender and recipient.
    function createDefaultStreamWithUsers(
        address recipient,
        address sender
    )
        internal
        virtual
        returns (uint256 streamId);
}
