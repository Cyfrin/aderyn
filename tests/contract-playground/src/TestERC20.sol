// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

contract TestERC20 {

    uint256 public constant decimals = 18;
    string public name;
    string public symbol;
    uint256 public totalSupply;

    mapping (address => uint256) public balances;
    mapping (address => mapping (address => uint256)) private allowances;

    event Approval(address indexed src, address indexed usr, uint256 wad);
    event Transfer(address indexed src, address indexed dst, uint256 wad);

    function getChainId() external view returns(uint256) {
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        return chainId;
    }

    constructor(string memory _symbol, string memory _name) {
        symbol = _symbol;
        name = _name;
    }

    function transfer(address dst, uint256 wad) external returns (bool) {
        return transferFrom(msg.sender, dst, wad);
    }

    function transferFrom(address src, address dst, uint256 wad) public returns (bool) {
        require(balances[src] >= wad, "nsufficient-balance");
        if (src != msg.sender && allowances[src][msg.sender] != type(uint256).max) {
            require(allowances[src][msg.sender] >= wad, "insufficient-allowances");
            allowances[src][msg.sender] -= wad;
        }
        balances[src] -= wad;
        balances[dst] += wad;
        emit Transfer(src, dst, wad);
        return true;
    }

    function mint(address usr, uint256 wad) external {
        balances[usr] += wad;
        totalSupply += wad;
        emit Transfer(address(0), usr, wad);
    }

    function burn(address usr, uint256 wad) external {
        require(balances[usr] >= wad, "insufficient-balance");
        if (usr != msg.sender && allowances[usr][msg.sender] != type(uint256).max) {
            require(allowances[usr][msg.sender] >= wad, "insufficient-allowances");
            allowances[usr][msg.sender] -= wad;
        }
        balances[usr] -= wad;
        totalSupply -= wad;
        emit Transfer(usr, address(0), wad);
    }

    function approve(address usr, uint256 wad) external returns (bool) {
        allowances[msg.sender][usr] = wad;
        emit Approval(msg.sender, usr, wad);
        return true;
    }

    function allowance(address owner, address spender) external view returns (uint256) {
        return allowances[owner][spender];
    }

    function balanceOf(address account) external view returns (uint256) {
        return balances[account];
    }

}