// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ud2x18 } from "@prb/math/src/UD2x18.sol";

import { Errors } from "src/libraries/Errors.sol";
import { ISablierV2MerkleLT } from "src/interfaces/ISablierV2MerkleLT.sol";
import { MerkleLockup, MerkleLT } from "src/types/DataTypes.sol";

import { MerkleLockup_Integration_Test } from "../../MerkleLockup.t.sol";

contract CreateMerkleLT_Integration_Test is MerkleLockup_Integration_Test {
    function setUp() public override {
        MerkleLockup_Integration_Test.setUp();
    }

    modifier whenTotalPercentageNotOneHundred() {
        _;
    }

    function test_RevertWhen_TotalPercentageLessThanOneHundred() external whenTotalPercentageNotOneHundred {
        MerkleLockup.ConstructorParams memory baseParams = defaults.baseParams();
        uint256 aggregateAmount = defaults.AGGREGATE_AMOUNT();
        uint256 recipientCount = defaults.RECIPIENT_COUNT();

        MerkleLT.TrancheWithPercentage[] memory tranchesWithPercentages = defaults.tranchesWithPercentages();
        tranchesWithPercentages[0].unlockPercentage = ud2x18(0.05e18);
        tranchesWithPercentages[1].unlockPercentage = ud2x18(0.2e18);

        uint64 totalPercentage =
            tranchesWithPercentages[0].unlockPercentage.unwrap() + tranchesWithPercentages[1].unlockPercentage.unwrap();

        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2MerkleLockupFactory_TotalPercentageNotOneHundred.selector, totalPercentage
            )
        );

        merkleLockupFactory.createMerkleLT(
            baseParams, lockupTranched, tranchesWithPercentages, aggregateAmount, recipientCount
        );
    }

    function test_RevertWhen_TotalPercentageGreaterThanOneHundred() external whenTotalPercentageNotOneHundred {
        MerkleLockup.ConstructorParams memory baseParams = defaults.baseParams();
        uint256 aggregateAmount = defaults.AGGREGATE_AMOUNT();
        uint256 recipientCount = defaults.RECIPIENT_COUNT();

        MerkleLT.TrancheWithPercentage[] memory tranchesWithPercentages = defaults.tranchesWithPercentages();
        tranchesWithPercentages[0].unlockPercentage = ud2x18(0.75e18);
        tranchesWithPercentages[1].unlockPercentage = ud2x18(0.8e18);

        uint64 totalPercentage =
            tranchesWithPercentages[0].unlockPercentage.unwrap() + tranchesWithPercentages[1].unlockPercentage.unwrap();

        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2MerkleLockupFactory_TotalPercentageNotOneHundred.selector, totalPercentage
            )
        );

        merkleLockupFactory.createMerkleLT(
            baseParams, lockupTranched, tranchesWithPercentages, aggregateAmount, recipientCount
        );
    }

    modifier whenTotalPercentageOneHundred() {
        _;
    }

    function test_RevertWhen_CampaignNameTooLong() external whenTotalPercentageOneHundred {
        MerkleLockup.ConstructorParams memory baseParams = defaults.baseParams();
        MerkleLT.TrancheWithPercentage[] memory tranchesWithPercentages = defaults.tranchesWithPercentages();
        uint256 aggregateAmount = defaults.AGGREGATE_AMOUNT();
        uint256 recipientCount = defaults.RECIPIENT_COUNT();

        baseParams.name = "this string is longer than 32 characters";

        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2MerkleLockup_CampaignNameTooLong.selector, bytes(baseParams.name).length, 32
            )
        );

        merkleLockupFactory.createMerkleLT(
            baseParams, lockupTranched, tranchesWithPercentages, aggregateAmount, recipientCount
        );
    }

    modifier whenCampaignNameNotTooLong() {
        _;
    }

    function testFuzz_CreateMerkleLT(
        address admin,
        uint40 expiration
    )
        external
        whenTotalPercentageOneHundred
        whenCampaignNameNotTooLong
    {
        vm.assume(admin != users.admin);
        address expectedLT = vm.computeCreateAddress(address(merkleLockupFactory), ++merkleLockupFactoryNonce);

        MerkleLockup.ConstructorParams memory baseParams = defaults.baseParams({
            admin: admin,
            asset_: dai,
            merkleRoot: defaults.MERKLE_ROOT(),
            expiration: expiration
        });

        vm.expectEmit({ emitter: address(merkleLockupFactory) });
        emit CreateMerkleLT({
            merkleLT: ISablierV2MerkleLT(expectedLT),
            baseParams: baseParams,
            lockupTranched: lockupTranched,
            tranchesWithPercentages: defaults.tranchesWithPercentages(),
            totalDuration: defaults.TOTAL_DURATION(),
            aggregateAmount: defaults.AGGREGATE_AMOUNT(),
            recipientCount: defaults.RECIPIENT_COUNT()
        });

        address actualLT = address(createMerkleLT(admin, expiration));
        assertGt(actualLT.code.length, 0, "MerkleLT contract not created");
        assertEq(actualLT, expectedLT, "MerkleLT contract does not match computed address");
    }
}
