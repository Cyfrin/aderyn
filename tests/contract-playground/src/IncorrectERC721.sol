// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract IncorrectERC721 {
    string public name = "IncorrectNFT";
    string public symbol = "INFT";
    uint256 public totalSupply;

    mapping(uint256 => address) public owners;
    mapping(address => uint256) public balances;
    mapping(uint256 => address) public tokenApprovals;
    mapping(address => mapping(address => bool)) public operatorApprovals;

    function balanceOf(address owner) external view returns (uint72) {
        return uint72(balances[owner]);
    }

    function ownerOf(uint256 tokenId) public view returns (bytes4) {
        return bytes4(keccak256(abi.encodePacked(owners[tokenId])));
    }

    function approve(
        address to,
        uint256 tokenId
    ) external returns (bytes memory) {
        address owner = msg.sender;
        require(
            to != address(uint160(bytes20(owner))),
            "Approval to current owner"
        );
        require(msg.sender == owner, "Approve caller is not owner");

        tokenApprovals[tokenId] = to;
        return "";
    }

    function getApproved(uint256 tokenId) external view returns (uint72) {
        return uint72(uint160(tokenApprovals[tokenId]));
    }

    function setApprovalForAll(
        address operator,
        bool approved
    ) external returns (bytes4) {
        operatorApprovals[msg.sender][operator] = approved;
        return bytes4(keccak256(abi.encodePacked(operator, approved)));
    }

    function isApprovedForAll(
        address owner,
        address operator
    ) external view returns (uint72) {
        return operatorApprovals[owner][operator] ? uint72(1) : uint72(0);
    }

    function transferFrom(
        address from,
        address to,
        uint256 tokenId
    ) external returns (bytes memory) {
        require(
            address(uint160(bytes20(ownerOf(tokenId)))) == from,
            "Transfer from incorrect owner"
        );
        require(to != address(0), "Transfer to the zero address");

        _transfer(from, to, tokenId);
        return "";
    }

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId,
        bytes memory _data
    ) external returns (uint8) {
        return 1;
    }

    function _transfer(address from, address to, uint256 tokenId) internal {
        owners[tokenId] = to;
        balances[from] -= 1;
        balances[to] += 1;
    }

    function mint(address to, uint256 tokenId) external returns (uint72) {
        require(to != address(0), "Mint to the zero address");
        require(owners[tokenId] == address(0), "Token already minted");

        _mint(to, tokenId);
        return uint72(tokenId);
    }

    function _mint(address to, uint256 tokenId) internal {
        owners[tokenId] = to;
        balances[to] += 1;
        totalSupply += 1;
    }
}

interface MyIERC721 {
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 indexed tokenId
    );
    event Approval(
        address indexed owner,
        address indexed approved,
        uint256 indexed tokenId
    );
    event ApprovalForAll(
        address indexed owner,
        address indexed operator,
        bool approved
    );

    function balanceOf(address owner) external view returns (uint256 balance);

    function ownerOf(uint256 tokenId) external view returns (address owner);

    function approve(address to, uint256 tokenId) external;

    function getApproved(
        uint256 tokenId
    ) external view returns (address operator);

    function setApprovalForAll(address operator, bool _approved) external;

    function isApprovedForAll(
        address owner,
        address operator
    ) external view returns (bool);

    function transferFrom(address from, address to, uint256 tokenId) external;

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId
    ) external;
}

contract CorrectERC721 is MyIERC721 {
    string public name = "CorrectNFT";
    string public symbol = "CNFT";
    uint256 public totalSupply;

    mapping(uint256 => address) private _owners;
    mapping(address => uint256) private _balances;
    mapping(uint256 => address) private _tokenApprovals;
    mapping(address => mapping(address => bool)) private _operatorApprovals;

    function balanceOf(address owner) external view override returns (uint256) {
        require(owner != address(0), "Balance query for the zero address");
        return _balances[owner];
    }

    function ownerOf(uint256 tokenId) public view override returns (address) {
        address owner = _owners[tokenId];
        require(owner != address(0), "Owner query for nonexistent token");
        return owner;
    }

    function approve(address to, uint256 tokenId) external override {
        address owner = ownerOf(tokenId);
        require(to != owner, "Approval to current owner");
        require(
            msg.sender == owner || isApprovedForAll(owner, msg.sender),
            "Approve caller is not owner nor approved for all"
        );

        _approve(to, tokenId);
    }

    function getApproved(
        uint256 tokenId
    ) public view override returns (address) {
        require(
            _owners[tokenId] != address(0),
            "Approved query for nonexistent token"
        );
        return _tokenApprovals[tokenId];
    }

    function setApprovalForAll(
        address operator,
        bool approved
    ) external override {
        require(operator != msg.sender, "Approve to caller");

        _operatorApprovals[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    function isApprovedForAll(
        address owner,
        address operator
    ) public view override returns (bool) {
        return _operatorApprovals[owner][operator];
    }

    function transferFrom(
        address from,
        address to,
        uint256 tokenId
    ) external override {
        require(
            _isApprovedOrOwner(msg.sender, tokenId),
            "Transfer caller is not owner nor approved"
        );
        _transfer(from, to, tokenId);
    }

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId
    ) external override {
        safeTransferFrom(from, to, tokenId, "");
    }

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId,
        bytes memory _data
    ) public {
        require(
            _isApprovedOrOwner(msg.sender, tokenId),
            "Transfer caller is not owner nor approved"
        );
        _safeTransfer(from, to, tokenId, _data);
    }

    function _safeTransfer(
        address from,
        address to,
        uint256 tokenId,
        bytes memory _data
    ) internal {
        _transfer(from, to, tokenId);
    }

    function _transfer(address from, address to, uint256 tokenId) internal {
        require(ownerOf(tokenId) == from, "Transfer from incorrect owner");
        require(to != address(0), "Transfer to the zero address");

        _approve(address(0), tokenId);

        _balances[from] -= 1;
        _balances[to] += 1;
        _owners[tokenId] = to;

        emit Transfer(from, to, tokenId);
    }

    function _approve(address to, uint256 tokenId) internal {
        _tokenApprovals[tokenId] = to;
        emit Approval(ownerOf(tokenId), to, tokenId);
    }

    function _isApprovedOrOwner(
        address spender,
        uint256 tokenId
    ) internal view returns (bool) {
        require(
            _owners[tokenId] != address(0),
            "Operator query for nonexistent token"
        );
        address owner = ownerOf(tokenId);
        return (spender == owner ||
            getApproved(tokenId) == spender ||
            isApprovedForAll(owner, spender));
    }
}

interface IERC721Receiver {
    function onERC721Received(
        address operator,
        address from,
        uint256 tokenId,
        bytes calldata data
    ) external returns (bytes4);
}
