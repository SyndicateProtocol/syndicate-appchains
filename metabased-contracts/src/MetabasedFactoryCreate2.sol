// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChainInitable} from "./MetabasedSequencerChainInitable.sol";
import {MetafillerStorage} from "./backfill/MetafillerStorage.sol";
import {RequireAllModule} from "./requirement-modules/RequireAllModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

/// @title MetabasedFactory
/// @notice Factory contract for creating MetabasedSequencerChainInitable

contract MetabasedFactoryCreate2 {
    function createSequencerChainInitable(bytes32 salt, uint256 chainId, address admin, address permissionModule)
        external
        returns (address)
    {
        bytes memory bytecode = getBytecode(chainId);
        address deployedAddress = Create2.deploy(0, salt, bytecode);

        MetabasedSequencerChainInitable sequencerChain = MetabasedSequencerChainInitable(deployedAddress);
        sequencerChain.init(admin, permissionModule);
        return deployedAddress;
    }

    function computeSequencerChainAddress(bytes32 salt, uint256 chainId) external view returns (address) {
        return Create2.computeAddress(salt, keccak256(getBytecode(chainId)));
    }

    function getBytecode(uint256 chainId) internal pure returns (bytes memory) {
        return abi.encodePacked(type(MetabasedSequencerChainInitable).creationCode, abi.encode(chainId));
    }
}
