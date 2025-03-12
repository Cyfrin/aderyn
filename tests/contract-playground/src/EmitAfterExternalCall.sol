// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract MaliciousActor {
    // LOGIC inside hello() doesn't matter. So long as it's an external call we don't trust!
    function hello() external {
        (bool s, ) = msg.sender.call("");
        require(s, "attempt failed");
    }
}

contract EmitAfterExternalCall {
    event SomeEvent();

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
        emit SomeEvent();
    }

    // BAD
    function badSituation2() external {
        // Interaction
        s_actor.hello();

        if (msg.sender != address(this)) {
            // Effect
            emit SomeEvent();
        }
    }

    // BAD
    function badSituation3() external {
        // NOTE: Although this may seem like it's following CEI, because it's inside a loop
        //one can imagine that in the second iteration, the effect happens after the interaction
        //in the first iteration.

        for (uint256 i = 0; i < 0; ++i) {
            // Effect
            emit SomeEvent();

            // Interaction
            s_actor.hello();
        }
    }

    // GOOD
    function goodSituation1() external {
        // Effect
        emit SomeEvent();

        // Interaction
        s_actor.hello();
    }

    // GOOD
    function goodSituation2() external {
        if (msg.sender != address(this)) {
            s_actor.hello();
            return;
        }

        emit SomeEvent();
    }
}

contract ReentrantContract {
	function f() external {
		if (BugReentrancyEvents(msg.sender).counter() == 1) {
			BugReentrancyEvents(msg.sender).count(this);
		}
	}
}
contract Counter {
	uint public counter;
	event CounterEvent(uint);

}
// BAD
contract BugReentrancyEvents is Counter {
    function count(ReentrantContract d) external {
        counter += 1;
        d.f();
        emit CounterEvent(counter);
    }
}
// GOOD
contract NoReentrancyEvents is Counter {
	function count(ReentrantContract d) external {
        counter += 1;
        emit CounterEvent(counter);
        d.f();
    }
}
