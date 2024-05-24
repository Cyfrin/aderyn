// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { UD60x18 } from "@prb/math/src/UD60x18.sol";

abstract contract Constants {
    UD60x18 internal constant MAX_BROKER_FEE = UD60x18.wrap(0.1e18); // 10%
    uint128 internal constant MAX_UINT128 = type(uint128).max;
    uint256 internal constant MAX_UINT256 = type(uint256).max;
    uint40 internal constant MAX_UINT40 = type(uint40).max;
    uint40 internal constant MAX_UNIX_TIMESTAMP = 2_147_483_647; // 2^31 - 1
    uint40 internal constant MAY_1_2024 = 1_714_518_000;
}
