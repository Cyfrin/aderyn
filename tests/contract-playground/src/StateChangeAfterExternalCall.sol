// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract MaliciousActor {
    // LOGIC inside hello() doesn't matter. So long as it's an external call we don't trust!
    function hello() external {
        (bool s, ) = msg.sender.call("");
        require(s, "attempt failed");
    }
}

contract StateChangeAfterExternalCall {
    uint256 s_useMe;
    MaliciousActor s_actor;

    constructor(address actor) {
        require(actor != address(0));
        s_actor = MaliciousActor(actor);
    }

    // BAD
    function badSituation1() external {
        // Interaction
        s_actor.hello();

        // Effect
        s_useMe += 1;
    }

    // BAD
    function badSituation2() external {
        // Interaction
        s_actor.hello();

        if (msg.sender != address(this)) {
            // Effect
            s_useMe -= 1;
        }
    }

    // BAD
    function badSituation3() external {
        // NOTE: Although this may seem like it's following CEI, because it's inside a loop
        //one can imagine that in the second iteration, the effect happens after the interaction
        //in the first iteration.

        for (uint256 i = 0; i < s_useMe; ++i) {
            // Effect
            s_useMe += 4;

            // Interaction
            s_actor.hello();
        }
    }

    // GOOD
    function goodSituation1() external {
        // Effect
        s_useMe += 1;

        // Interaction
        s_actor.hello();
    }

    // GOOD
    function goodSituation2() external {
        if (msg.sender != address(this)) {
            s_actor.hello();
            return;
        }

        s_useMe += 1;
    }

    modifier onlyOwner() {
        require(msg.sender == address(123), "Not allowed");
        _;
    }

    // BAD
    function badSituation4() external onlyOwner {
        s_actor.hello();
        s_useMe += 1;
    }

    // GOOD
    function goodSituation3() external onlyOwner {
        s_useMe += 1;
        s_actor.hello();
    }
}
