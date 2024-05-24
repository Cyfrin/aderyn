// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { ud2x18 } from "@prb/math/src/UD2x18.sol";
import { ud60x18 } from "@prb/math/src/UD60x18.sol";

import { Solarray } from "solarray/src/Solarray.sol";

import { ISablierV2LockupDynamic } from "../src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "../src/interfaces/ISablierV2LockupLinear.sol";
import { Broker, LockupDynamic, LockupLinear } from "../src/types/DataTypes.sol";

import { BaseScript } from "./Base.s.sol";

interface IERC20Mint {
    function mint(address beneficiary, uint256 value) external;
}

/// @notice Initializes the protocol by creating some streams.
contract Init is BaseScript {
    function run(
        ISablierV2LockupLinear lockupLinear,
        ISablierV2LockupDynamic lockupDynamic,
        IERC20 asset
    )
        public
        broadcast
    {
        address sender = broadcaster;
        address recipient = vm.addr(vm.deriveKey({ mnemonic: mnemonic, index: 1 }));

        /*//////////////////////////////////////////////////////////////////////////
                                       LOCKUP-LINEAR
        //////////////////////////////////////////////////////////////////////////*/

        // Mint enough assets to the sender.
        IERC20Mint(address(asset)).mint({ beneficiary: sender, value: 131_601.1e18 + 10_000e18 });

        // Approve the Sablier contracts to transfer the ERC-20 assets from the sender.
        asset.approve({ spender: address(lockupLinear), value: type(uint256).max });
        asset.approve({ spender: address(lockupDynamic), value: type(uint256).max });

        // Create 7 Lockup Linear streams with various amounts and durations.
        //
        // - 1st stream: meant to be depleted.
        // - 2th to 4th streams: pending or streaming.
        // - 5th stream: meant to be renounced.
        // - 6th stream: meant to canceled.
        // - 7th stream: meant to be transferred to a third party.
        uint128[] memory totalAmounts = Solarray.uint128s(0.1e18, 1e18, 100e18, 1000e18, 5000e18, 25_000e18, 100_000e18);
        uint40[] memory cliffDurations = Solarray.uint40s(0, 0, 0, 0, 24 hours, 1 weeks, 12 weeks);
        uint40[] memory totalDurations =
            Solarray.uint40s(1 seconds, 1 hours, 24 hours, 1 weeks, 4 weeks, 12 weeks, 48 weeks);
        for (uint256 i = 0; i < totalDurations.length; ++i) {
            lockupLinear.createWithDurations(
                LockupLinear.CreateWithDurations({
                    sender: sender,
                    recipient: recipient,
                    totalAmount: totalAmounts[i],
                    asset: asset,
                    cancelable: true,
                    transferable: true,
                    durations: LockupLinear.Durations({ cliff: cliffDurations[i], total: totalDurations[i] }),
                    broker: Broker(address(0), ud60x18(0))
                })
            );
        }

        // Renounce the 5th stream.
        lockupLinear.renounce({ streamId: 5 });

        // Cancel the 6th stream.
        lockupLinear.cancel({ streamId: 6 });

        /*//////////////////////////////////////////////////////////////////////////
                                       LOCKUP-DYNAMIC
        //////////////////////////////////////////////////////////////////////////*/

        // Create the default lockupDynamic stream.
        LockupDynamic.SegmentWithDuration[] memory segments = new LockupDynamic.SegmentWithDuration[](2);
        segments[0] =
            LockupDynamic.SegmentWithDuration({ amount: 2500e18, exponent: ud2x18(3.14e18), duration: 1 hours });
        segments[1] =
            LockupDynamic.SegmentWithDuration({ amount: 7500e18, exponent: ud2x18(0.5e18), duration: 1 weeks });
        lockupDynamic.createWithDurations(
            LockupDynamic.CreateWithDurations({
                sender: sender,
                recipient: recipient,
                totalAmount: 10_000e18,
                asset: asset,
                cancelable: true,
                transferable: true,
                segments: segments,
                broker: Broker(address(0), ud60x18(0))
            })
        );
    }
}
