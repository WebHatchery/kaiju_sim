// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title KaijuNFT
 * @dev ERC721 Token for Kaiju Breeding Simulator
 * Server-custodial optimized: only owner (server) can mint.
 */
contract KaijuNFT is ERC721, Ownable {
    using Strings for uint256;

    // Base URI for metadata
    string private _baseTokenURI;

    // Mapping from token ID to specific IPFS URI (if not using base URI)
    mapping(uint256 => string) private _tokenURIs;

    // Mapping of dead kaiju (cannot be transferred if enforced)
    mapping(uint256 => bool) public isDead;

    // Events
    event KaijuMinted(uint256 indexed tokenId, address indexed to, string uri);
    event KaijuDied(uint256 indexed tokenId);
    event BaseURIUpdated(string newBaseURI);

    constructor(address initialOwner) 
        ERC721("Kaiju Breeding Simulator", "KAIJU") 
        Ownable(initialOwner) 
    {}

    /**
     * @dev Mint a new Kaiju NFT. Only callable by server (owner).
     * @param to The address that will own the minted NFT.
     * @param tokenId The ID of the token to be minted.
     * @param uri The IPFS URI for the token metadata.
     */
    function safeMint(address to, uint256 tokenId, string memory uri) public onlyOwner {
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, uri);
        emit KaijuMinted(tokenId, to, uri);
    }

    /**
     * @dev Mark a Kaiju as dead. Only callable by server.
     * @param tokenId The ID of the dead kaiju.
     */
    function markDead(uint256 tokenId) public onlyOwner {
        require(_ownerOf(tokenId) != address(0), "Kaiju does not exist");
        isDead[tokenId] = true;
        emit KaijuDied(tokenId);
    }

    /**
     * @dev Update the base URI.
     */
    function setBaseURI(string memory baseURI) public onlyOwner {
        _baseTokenURI = baseURI;
        emit BaseURIUpdated(baseURI);
    }

    /**
     * @dev Sets `_tokenURI` as the tokenURI of `tokenId`.
     */
    function _setTokenURI(uint256 tokenId, string memory _tokenURI) internal virtual {
        _tokenURIs[tokenId] = _tokenURI;
    }

    /**
     * @dev Returns the Uniform Resource Identifier (URI) for `tokenId` token.
     */
    function tokenURI(uint256 tokenId) public view virtual override returns (string memory) {
        _requireOwned(tokenId);

        string memory _tokenURI = _tokenURIs[tokenId];

        // If specific URI is set, return it
        if (bytes(_tokenURI).length > 0) {
            return _tokenURI;
        }

        return super.tokenURI(tokenId);
    }

    function _baseURI() internal view virtual override returns (string memory) {
        return _baseTokenURI;
    }
}
