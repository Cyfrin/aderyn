// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2MerkleLT } from "src/interfaces/ISablierV2MerkleLT.sol";

import { MerkleLockup_Integration_Test } from "../../MerkleLockup.t.sol";

contract HasExpired_Integration_Test is MerkleLockup_Integration_Test {
    function setUp() public virtual override {
        MerkleLockup_Integration_Test.setUp();
    }

    function test_HasExpired_ExpirationZero() external {
        ISablierV2MerkleLT testLockup = createMerkleLT({ expiration: 0 });
        assertFalse(testLockup.hasExpired(), "campaign expired");
    }

    modifier givenExpirationNotZero() {
        _;
    }

    function test_HasExpired_ExpirationLessThanBlockTimestamp() external view givenExpirationNotZero {
        assertFalse(merkleLT.hasExpired(), "campaign expired");
    }

    function test_HasExpired_ExpirationEqualToBlockTimestamp() external givenExpirationNotZero {
        vm.warp({ newTimestamp: defaults.EXPIRATION() });
        assertTrue(merkleLT.hasExpired(), "campaign not expired");
    }

    function test_HasExpired_ExpirationGreaterThanBlockTimestamp() external givenExpirationNotZero {
        vm.warp({ newTimestamp: defaults.EXPIRATION() + 1 seconds });
        assertTrue(merkleLT.hasExpired(), "campaign not expired");
    }
}
