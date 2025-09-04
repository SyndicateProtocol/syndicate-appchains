// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/// @notice Minimal interface for the L1Block precompile on Base/Optimism stack
interface IL1Block {
    function hash() external view returns (bytes32);
}

/// @notice Minimal interface for ArbSys on Arbitrum
interface IArbSys {
    function sendTxToL2(address destination, uint256 l2CallValue, bytes calldata data)
        external
        payable
        returns (uint256);
}

// TODO untested
contract BlockHashMessenger {
    address public constant L1_BLOCK_ADDRESS = 0x4200000000000000000000000000000000000015;
    address public constant ARBSYS_ADDRESS = 0x0000000000000000000000000000000000000064;

    address public gasArchive;

    constructor(address _l3Target) {
        gasArchive = _l3Target;
    }

    /// @notice Sends Ethereum and Base block hashes to the L3 contract
    function sendBlockHashes() external payable {
        bytes32 ethBlockHash = IL1Block(L1_BLOCK_ADDRESS).hash();
        bytes32 baseBlockHash = blockhash(block.number - 1);

        // Encode call to L3 contract
        bytes memory callData =
            abi.encodeWithSignature("setLastKnownBlockHashes(bytes32,bytes32)", ethBlockHash, baseBlockHash);

        // Send to L3 via ArbSys
        IArbSys(ARBSYS_ADDRESS).sendTxToL2(
            gasArchive,
            0, // no callvalue on L3
            callData
        );
    }
}
