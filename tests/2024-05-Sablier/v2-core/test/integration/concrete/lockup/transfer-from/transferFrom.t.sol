// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract TransferFrom_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) {
        resetPrank({ msgSender: users.recipient });
    }

    function test_RevertGiven_StreamNotTransferable() external {
        uint256 notTransferableStreamId = createDefaultStreamNotTransferable();
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2Lockup_NotTransferable.selector, notTransferableStreamId)
        );
        lockup.transferFrom({ from: users.recipient, to: users.alice, tokenId: notTransferableStreamId });
    }

    modifier givenStreamTransferable() {
        _;
    }

    function test_TransferFrom() external givenStreamTransferable {
        // Create a stream.
        uint256 streamId = createDefaultStream();

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit Transfer({ from: users.recipient, to: users.alice, tokenId: streamId });
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: streamId });

        // Transfer the NFT.
        lockup.transferFrom({ from: users.recipient, to: users.alice, tokenId: streamId });

        // Assert that Alice is the new stream recipient (and NFT owner).
        address actualRecipient = lockup.getRecipient(streamId);
        address expectedRecipient = users.alice;
        assertEq(actualRecipient, expectedRecipient, "recipient");
    }
}
