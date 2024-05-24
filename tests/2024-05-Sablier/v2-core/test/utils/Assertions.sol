// SPDX-License-Identifier: GPL-3.0-or-later
// solhint-disable event-name-camelcase
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { PRBMathAssertions } from "@prb/math/test/utils/Assertions.sol";

import { Lockup, LockupDynamic, LockupLinear, LockupTranched } from "../../src/types/DataTypes.sol";

abstract contract Assertions is PRBMathAssertions {
    /*//////////////////////////////////////////////////////////////////////////
                                       EVENTS
    //////////////////////////////////////////////////////////////////////////*/

    event log_named_array(string key, LockupDynamic.Segment[] segments);

    event log_named_array(string key, LockupTranched.Tranche[] tranches);

    event log_named_uint128(string key, uint128 value);

    event log_named_uint40(string key, uint40 value);

    /*//////////////////////////////////////////////////////////////////////////
                                     ASSERTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Compares two {Lockup.Amounts} struct entities.
    function assertEq(Lockup.Amounts memory a, Lockup.Amounts memory b) internal {
        assertEqUint128(a.deposited, b.deposited, "amounts.deposited");
        assertEqUint128(a.refunded, b.refunded, "amounts.refunded");
        assertEqUint128(a.withdrawn, b.withdrawn, "amounts.withdrawn");
    }

    /// @dev Compares two {IERC20} values.
    function assertEq(IERC20 a, IERC20 b) internal pure {
        assertEq(address(a), address(b));
    }

    /// @dev Compares two {IERC20} values.
    function assertEq(IERC20 a, IERC20 b, string memory err) internal pure {
        assertEq(address(a), address(b), err);
    }

    /// @dev Compares two {LockupDynamic.Stream} struct entities.
    function assertEq(LockupDynamic.StreamLD memory a, LockupDynamic.StreamLD memory b) internal {
        assertEq(a.asset, b.asset, "asset");
        assertEq(a.endTime, b.endTime, "endTime");
        assertEq(a.isCancelable, b.isCancelable, "isCancelable");
        assertEq(a.isDepleted, b.isDepleted, "isDepleted");
        assertEq(a.isStream, b.isStream, "isStream");
        assertEq(a.isTransferable, b.isTransferable, "isTransferable");
        assertEq(a.recipient, b.recipient, "recipient");
        assertEq(a.segments, b.segments, "segments");
        assertEq(a.sender, b.sender, "sender");
        assertEq(a.startTime, b.startTime, "startTime");
        assertEq(a.wasCanceled, b.wasCanceled, "wasCanceled");
    }

    /// @dev Compares two {LockupLinear.Stream} struct entities.
    function assertEq(LockupLinear.StreamLL memory a, LockupLinear.StreamLL memory b) internal {
        assertEq(a.amounts, b.amounts);
        assertEq(a.asset, b.asset, "asset");
        assertEq(a.cliffTime, b.cliffTime, "cliffTime");
        assertEq(a.endTime, b.endTime, "endTime");
        assertEq(a.isCancelable, b.isCancelable, "isCancelable");
        assertEq(a.isDepleted, b.isDepleted, "isDepleted");
        assertEq(a.isStream, b.isStream, "isStream");
        assertEq(a.isTransferable, b.isTransferable, "isTransferable");
        assertEq(a.recipient, b.recipient, "recipient");
        assertEq(a.sender, b.sender, "sender");
        assertEq(a.startTime, b.startTime, "startTime");
        assertEq(a.wasCanceled, b.wasCanceled, "wasCanceled");
    }

    /// @dev Compares two {LockupTranched.Stream} struct entities.
    function assertEq(LockupTranched.StreamLT memory a, LockupTranched.StreamLT memory b) internal {
        assertEq(a.asset, b.asset, "asset");
        assertEq(a.endTime, b.endTime, "endTime");
        assertEq(a.isCancelable, b.isCancelable, "isCancelable");
        assertEq(a.isDepleted, b.isDepleted, "isDepleted");
        assertEq(a.isStream, b.isStream, "isStream");
        assertEq(a.isTransferable, b.isTransferable, "isTransferable");
        assertEq(a.recipient, b.recipient, "recipient");
        assertEq(a.sender, b.sender, "sender");
        assertEq(a.startTime, b.startTime, "startTime");
        assertEq(a.tranches, b.tranches, "tranches");
        assertEq(a.wasCanceled, b.wasCanceled, "wasCanceled");
    }

    /// @dev Compares two {LockupDynamic.Timestamps} struct entities.
    function assertEq(LockupDynamic.Timestamps memory a, LockupDynamic.Timestamps memory b) internal {
        assertEqUint40(a.end, b.end, "timestamps.end");
        assertEqUint40(a.start, b.start, "timestamps.start");
    }

    /// @dev Compares two {LockupLinear.Timestamps} struct entities.
    function assertEq(LockupLinear.Timestamps memory a, LockupLinear.Timestamps memory b) internal {
        assertEqUint40(a.cliff, b.cliff, "timestamps.cliff");
        assertEqUint40(a.end, b.end, "timestamps.end");
        assertEqUint40(a.start, b.start, "timestamps.start");
    }

    /// @dev Compares two {LockupTranched.Timestamps} struct entities.
    function assertEq(LockupTranched.Timestamps memory a, LockupTranched.Timestamps memory b) internal {
        assertEqUint40(a.end, b.end, "timestamps.end");
        assertEqUint40(a.start, b.start, "timestamps.start");
    }

    /// @dev Compares two {LockupDynamic.Segment} arrays.
    function assertEq(LockupDynamic.Segment[] memory a, LockupDynamic.Segment[] memory b) internal {
        if (keccak256(abi.encode(a)) != keccak256(abi.encode(b))) {
            emit log("Error: a == b not satisfied [LockupDynamic.Segment[]]");
            emit log_named_array("   Left", a);
            emit log_named_array("  Right", b);
            fail();
        }
    }

    /// @dev Compares two {LockupDynamic.Segment} arrays.
    function assertEq(LockupDynamic.Segment[] memory a, LockupDynamic.Segment[] memory b, string memory err) internal {
        if (keccak256(abi.encode(a)) != keccak256(abi.encode(b))) {
            emit log_named_string("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Compares two {LockupTranched.Tranche} arrays.
    function assertEq(LockupTranched.Tranche[] memory a, LockupTranched.Tranche[] memory b) internal {
        if (keccak256(abi.encode(a)) != keccak256(abi.encode(b))) {
            emit log("Error: a == b not satisfied [LockupTranched.Tranche[]]");
            emit log_named_array("   Left", a);
            emit log_named_array("  Right", b);
            fail();
        }
    }

    /// @dev Compares two {LockupTranched.Tranche} arrays.
    function assertEq(
        LockupTranched.Tranche[] memory a,
        LockupTranched.Tranche[] memory b,
        string memory err
    )
        internal
    {
        if (keccak256(abi.encode(a)) != keccak256(abi.encode(b))) {
            emit log_named_string("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Compares two {Lockup.Status} enum values.
    function assertEq(Lockup.Status a, Lockup.Status b) internal pure {
        assertEq(uint256(a), uint256(b), "status");
    }

    /// @dev Compares two {Lockup.Status} enum values.
    function assertEq(Lockup.Status a, Lockup.Status b, string memory err) internal pure {
        assertEq(uint256(a), uint256(b), err);
    }

    /// @dev Compares two `uint128` numbers.
    function assertEqUint128(uint128 a, uint128 b) internal {
        if (a != b) {
            emit log("Error: a == b not satisfied [uint128]");
            emit log_named_uint128("   Left", a);
            emit log_named_uint128("  Right", b);
            fail();
        }
    }

    /// @dev Compares two `uint128` numbers.
    function assertEqUint128(uint128 a, uint128 b, string memory err) internal {
        if (a != b) {
            emit log_named_string("Error", err);
            assertEqUint128(a, b);
        }
    }

    /// @dev Compares two `uint40` numbers.
    function assertEqUint40(uint40 a, uint40 b) internal {
        if (a != b) {
            emit log("Error: a == b not satisfied [uint40]");
            emit log_named_uint40("   Left", a);
            emit log_named_uint40("  Right", b);
            fail();
        }
    }

    /// @dev Compares two `uint40` numbers.
    function assertEqUint40(uint40 a, uint40 b, string memory err) internal {
        if (a != b) {
            emit log_named_string("Error", err);
            assertEqUint40(a, b);
        }
    }

    /// @dev Compares two {Lockup.Status} enum values.
    function assertNotEq(Lockup.Status a, Lockup.Status b) internal pure {
        assertNotEq(uint256(a), uint256(b), "status");
    }

    /// @dev Compares two {Lockup.Status} enum values.
    function assertNotEq(Lockup.Status a, Lockup.Status b, string memory err) internal pure {
        assertNotEq(uint256(a), uint256(b), err);
    }
}
