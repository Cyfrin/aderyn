// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC721Errors } from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract GetRecipient_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) {
        defaultStreamId = createDefaultStream();
    }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721NonexistentToken.selector, nullStreamId));
        lockup.getRecipient(nullStreamId);
    }

    modifier givenNotNull() {
        _;
    }

    function test_RevertGiven_NFTBurned() external {
        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.END_TIME() });

        // Make the Recipient the caller.
        resetPrank({ msgSender: users.recipient });

        // Deplete the stream.
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });

        // Burn the NFT.
        lockup.burn(defaultStreamId);

        // Expect the relevant error when retrieving the recipient.
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721NonexistentToken.selector, defaultStreamId));
        lockup.getRecipient(defaultStreamId);
    }

    modifier givenNFTNotBurned() {
        _;
    }

    function test_GetRecipient() external view givenNotNull givenNFTNotBurned {
        address actualRecipient = lockup.getRecipient(defaultStreamId);
        address expectedRecipient = users.recipient;
        assertEq(actualRecipient, expectedRecipient, "recipient");
    }
}
