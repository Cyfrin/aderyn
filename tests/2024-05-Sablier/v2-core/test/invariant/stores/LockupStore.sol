// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup } from "src/types/DataTypes.sol";

/// @dev Storage variables needed by all lockup handlers.
contract LockupStore {
    /*//////////////////////////////////////////////////////////////////////////
                                     VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    mapping(uint256 streamId => bool recorded) public isPreviousStatusRecorded;
    uint256 public lastStreamId;
    mapping(uint256 streamId => Lockup.Status status) public previousStatusOf;
    mapping(uint256 streamId => address recipient) public recipients;
    mapping(uint256 streamId => address sender) public senders;
    uint256[] public streamIds;

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    function pushStreamId(uint256 streamId, address sender, address recipient) external {
        // Store the stream IDs, the senders, and the recipients.
        streamIds.push(streamId);
        senders[streamId] = sender;
        recipients[streamId] = recipient;

        // Update the last stream ID.
        lastStreamId = streamId;
    }

    function updateIsPreviousStatusRecorded(uint256 streamId) external {
        isPreviousStatusRecorded[streamId] = true;
    }

    function updatePreviousStatusOf(uint256 streamId, Lockup.Status status) external {
        previousStatusOf[streamId] = status;
    }

    function updateRecipient(uint256 streamId, address newRecipient) external {
        recipients[streamId] = newRecipient;
    }
}
