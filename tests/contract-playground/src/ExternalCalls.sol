// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

interface IMyTarget {
    function extCall(uint256) external payable;
}

contract MyTarget is IMyTarget {
    uint256 s_a;

    function extCall(uint256 m_a) public payable {
        s_a = m_a;
    }
}

contract BaseEx {
    MyTarget t2;

    function baseThing(address x) public {
        t2 = MyTarget(x);
    }
}

contract ChildEx is BaseEx {
    MyTarget t1;

    constructor(MyTarget t) {
        t1 = t;
    }

    // Functions that make external calls

    function ext1() external payable {
        t1.extCall(0);
    }

    function ext2(address target) external {
        MyTarget(target).extCall(0);
    }

    function ext3() external {
        this.ext1();
    }

    function ext4() external {
        IMyTarget(address(t1)).extCall(0);
    }

    // Functions that make external calls with options

    function ext5() external {
        t1.extCall{gas: 100}(0);
    }

    function ext6(address target) external {
        MyTarget(target).extCall{value: 100}(0);
    }

    function ext7() external {
        this.ext1{gas: 100}();
    }

    function ext8() external {
        IMyTarget(address(t1)).extCall{value: 100}(0);
    }

    function ext9() external {
        (bool success, ) = payable(address(t1)).call{
            value: address(this).balance
        }("");
        if (success) {
            revert();
        }
    }

    // Functions that don't make external calls

    function notExt1() external {
        super.baseThing(address(0));
    }

    function notExt2() external {
        BaseEx.baseThing(address(0));
    }
}
