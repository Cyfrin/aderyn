// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC721Errors } from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";
import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract Burn_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal streamId;
    uint256 internal notTransferableStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) {
        streamId = createDefaultStream();
        notTransferableStreamId = createDefaultStreamNotTransferable();

        // Make the Recipient (owner of the NFT) the caller in this test suite.
        resetPrank({ msgSender: users.recipient });
    }

    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData = abi.encodeCall(ISablierV2Lockup.burn, streamId);
        (bool success, bytes memory returnData) = address(lockup).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    modifier whenNotDelegateCalled() {
        _;
    }

    function test_RevertGiven_Null() external whenNotDelegateCalled {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.burn(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    modifier givenStreamHasNotBeenDepleted() {
        _;
    }

    function test_RevertGiven_StatusPending()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasNotBeenDepleted
    {
        vm.warp({ newTimestamp: getBlockTimestamp() - 1 seconds });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamNotDepleted.selector, streamId));
        lockup.burn(streamId);
    }

    function test_RevertGiven_StatusStreaming()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasNotBeenDepleted
    {
        vm.warp({ newTimestamp: defaults.WARP_26_PERCENT() });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamNotDepleted.selector, streamId));
        lockup.burn(streamId);
    }

    function test_RevertGiven_StatusSettled()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasNotBeenDepleted
    {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamNotDepleted.selector, streamId));
        lockup.burn(streamId);
    }

    function test_RevertGiven_StatusCanceled()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasNotBeenDepleted
    {
        vm.warp({ newTimestamp: defaults.CLIFF_TIME() });
        resetPrank({ msgSender: users.sender });
        lockup.cancel(streamId);
        resetPrank({ msgSender: users.recipient });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_StreamNotDepleted.selector, streamId));
        lockup.burn(streamId);
    }

    modifier givenStreamHasBeenDepleted(uint256 streamId_) {
        vm.warp({ newTimestamp: defaults.END_TIME() });
        lockup.withdrawMax({ streamId: streamId_, to: users.recipient });
        _;
    }

    function test_RevertWhen_CallerUnauthorized()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasBeenDepleted(streamId)
    {
        resetPrank({ msgSender: users.eve });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Unauthorized.selector, streamId, users.eve));
        lockup.burn(streamId);
    }

    modifier whenCallerAuthorized() {
        _;
    }

    function test_RevertGiven_NFTDoesNotExist()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasBeenDepleted(streamId)
        whenCallerAuthorized
    {
        // Burn the NFT so that it no longer exists.
        lockup.burn(streamId);

        // Run the test.
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721NonexistentToken.selector, streamId));
        lockup.burn(streamId);
    }

    modifier givenNFTExists() {
        _;
    }

    function test_Burn_NonTransferableNFT()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasBeenDepleted(notTransferableStreamId)
        whenCallerAuthorized
        givenNFTExists
    {
        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: notTransferableStreamId });
        lockup.burn(notTransferableStreamId);
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721NonexistentToken.selector, notTransferableStreamId));
        lockup.getRecipient(notTransferableStreamId);
    }

    modifier givenTransferableStream() {
        _;
    }

    function test_Burn_CallerApprovedOperator()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasBeenDepleted(streamId)
        whenCallerAuthorized
        givenNFTExists
        givenTransferableStream
    {
        // Approve the operator to handle the stream.
        lockup.approve({ to: users.operator, tokenId: streamId });

        // Make the approved operator the caller in this test.
        resetPrank({ msgSender: users.operator });

        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: streamId });

        // Burn the NFT.
        lockup.burn(streamId);

        // Assert that the NFT has been burned.
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721NonexistentToken.selector, streamId));
        lockup.getRecipient(streamId);
    }

    function test_Burn_CallerNFTOwner()
        external
        whenNotDelegateCalled
        givenNotNull
        givenStreamHasBeenDepleted(streamId)
        whenCallerAuthorized
        givenNFTExists
        givenTransferableStream
    {
        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit MetadataUpdate({ _tokenId: streamId });
        lockup.burn(streamId);
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721NonexistentToken.selector, streamId));
        lockup.getRecipient(streamId);
    }
}
