// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @title SyndicateDeterministicAddresses
/// @notice Library containing deterministic addresses and utility functions for the Syndicate system
library SyndicateDeterministicAddresses {
    /// @notice Gas Aggregator contract address
    /// @dev TODO SEQ-2078 - change to correct address
    address public constant GAS_AGGREGATOR = 0x00000000000000000000000000000000000006A5;

    /// @notice Factory contract address
    /// @dev TODO SEQ-2078 - change to correct address
    address public constant FACTORY = 0x0000000000000000000000000000000000000fac;

    // TODO SEQ-2078 - the stub implementation can probably be a constant too.

    /// @notice Computes the deterministic address for a sequencing chain proxy
    /// @param chainId The chain ID for the sequencing chain
    /// @param proxyBytecodeHash The hash of the proxy bytecode
    /// @return The computed address of the sequencing chain proxy
    function computeSequencingChainAddress(uint256 chainId, bytes32 proxyBytecodeHash)
        internal
        pure
        returns (address)
    {
        return Create2.computeAddress(bytes32(chainId), proxyBytecodeHash, FACTORY);
    }

    /// @notice Gets the proxy bytecode for ERC1967Proxy with stub implementation
    /// @param stubImplementation The stub implementation address to use
    /// @return The proxy bytecode
    function getProxyBytecode(address stubImplementation) internal pure returns (bytes memory) {
        return abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(stubImplementation, ""));
    }

    /// @notice Gets the hash of the proxy bytecode
    /// @param stubImplementation The stub implementation address to use
    /// @return The hash of the proxy bytecode
    function getProxyBytecodeHash(address stubImplementation) internal pure returns (bytes32) {
        return keccak256(getProxyBytecode(stubImplementation));
    }
}
