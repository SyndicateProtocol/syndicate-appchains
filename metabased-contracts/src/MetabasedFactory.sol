// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "./MetabasedSequencerChain.sol";
import {MetafillerStorage} from "./backfill/MetafillerStorage.sol";
import {RequireAllModule} from "./requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "./requirement-modules/RequireAnyModule.sol";
import {IRequirementModule} from "./interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

/// @title MetabasedFactory
/// @notice Factory contract for creating MetabasedSequencerChain and related contracts
contract MetabasedFactory {
    /// @notice Emitted when a new MetabasedSequencerChain is created
    event MetabasedSequencerChainCreated(
        uint256 indexed l3ChainId,
        address indexed metabasedSequencerChainAddress,
        address indexed permissionModuleAddress
    );

    error ZeroAddress();
    error ZeroValue();

    modifier zeroValuesChainAndTwoAddressesNotAllowed(
        uint256 l3ChainId,
        address firstAddrCheck,
        address secondAddrCheck
    ) {
        if (l3ChainId == 0) {
            revert ZeroValue();
        }
        if (firstAddrCheck == address(0) || secondAddrCheck == address(0)) {
            revert ZeroAddress();
        }
        _;
    }

    modifier zeroValuesChainAndAddressNotAllowed(uint256 l3ChainId, address addrCheck) {
        if (l3ChainId == 0) {
            revert ZeroValue();
        }
        if (addrCheck == address(0)) {
            revert ZeroAddress();
        }
        _;
    }

    /// @notice Creates a new MetabasedSequencerChain contract with a permission module
    /// @param l3ChainId the l3 chain the contract refers to
    /// @param admin The address that will be the admin
    /// @param permissionModule The address of the permission module
    /// @param salt The salt to use for the deployment, this should be the l3ChainId if it has not been previously used
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    function createMetabasedSequencerChain(
        uint256 l3ChainId,
        address admin,
        IRequirementModule permissionModule,
        bytes32 salt
    )
        public
        zeroValuesChainAndTwoAddressesNotAllowed(l3ChainId, admin, address(permissionModule))
        returns (address sequencerChain)
    {
        bytes memory bytecode = getBytecode(l3ChainId);
        address deployedAddress = Create2.deploy(0, salt, bytecode);

        MetabasedSequencerChain newSequencerChain = MetabasedSequencerChain(deployedAddress);
        newSequencerChain.initialize(admin, address(permissionModule));
        emit MetabasedSequencerChainCreated(l3ChainId, deployedAddress, address(permissionModule));
        return deployedAddress;
    }

    /// @notice Creates MetabasedSequencerChain with RequireAllModule
    /// @param admin The address that will be the default admin role
    /// @param l3ChainId The L3 chain ID
    /// @param salt The salt to use for the deployment
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return permissionModule The address of the newly created RequireAllModule
    function createMetabasedSequencerChainWithRequireAllModule(address admin, uint256 l3ChainId, bytes32 salt)
        public
        zeroValuesChainAndAddressNotAllowed(l3ChainId, admin)
        returns (address sequencerChain, IRequirementModule permissionModule)
    {
        permissionModule = IRequirementModule(new RequireAllModule(admin));
        (sequencerChain) = createMetabasedSequencerChain(l3ChainId, admin, permissionModule, salt);

        emit MetabasedSequencerChainCreated(l3ChainId, sequencerChain, address(permissionModule));

        return (sequencerChain, permissionModule);
    }

    /// @notice Creates MetabasedSequencerChain with RequireAnyModule
    /// @param admin The address that will be the default admin role
    /// @param l3ChainId The L3 chain ID
    /// @param salt The salt to use for the deployment
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return permissionModule The address of the newly created RequireAnyModule
    function createMetabasedSequencerChainWithRequireAnyModule(address admin, uint256 l3ChainId, bytes32 salt)
        public
        zeroValuesChainAndAddressNotAllowed(l3ChainId, admin)
        returns (address sequencerChain, IRequirementModule permissionModule)
    {
        permissionModule = IRequirementModule(new RequireAnyModule(admin));
        (sequencerChain) = createMetabasedSequencerChain(l3ChainId, admin, permissionModule, salt);

        emit MetabasedSequencerChainCreated(l3ChainId, sequencerChain, address(permissionModule));

        return (sequencerChain, permissionModule);
    }

    /// @notice Computes the address of the MetabasedSequencerChain contract
    /// @param salt The salt to use for the deployment
    /// @param chainId The ID of the L3 chain
    /// @return The address of the MetabasedSequencerChain contract
    function computeSequencerChainAddress(bytes32 salt, uint256 chainId) public view returns (address) {
        return Create2.computeAddress(salt, keccak256(getBytecode(chainId)));
    }

    /// @notice Returns the bytecode of the MetabasedSequencerChain contract
    /// @param chainId The ID of the L3 chain
    /// @return The bytecode of the MetabasedSequencerChain contract
    function getBytecode(uint256 chainId) public pure returns (bytes memory) {
        return abi.encodePacked(type(MetabasedSequencerChain).creationCode, abi.encode(chainId));
    }
}
