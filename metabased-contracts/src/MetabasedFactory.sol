// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "./MetabasedSequencerChain.sol";
import {RequireAllModule} from "./requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "./requirement-modules/RequireAnyModule.sol";
import {IRequirementModule} from "./interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

/// @title MetabasedFactory
/// @notice Factory contract for creating MetabasedSequencerChain and related contracts
contract MetabasedFactory {
    /// @notice Emitted when a new MetabasedSequencerChain is created
    event MetabasedSequencerChainCreated(
        uint256 indexed appChainId,
        address indexed metabasedSequencerChainAddress,
        address indexed permissionModuleAddress
    );

    error ZeroAddress();
    error ZeroValue();

    modifier zeroValuesChainAndTwoAddressesNotAllowed(
        uint256 appChainId,
        address firstAddrCheck,
        address secondAddrCheck
    ) {
        if (appChainId == 0) {
            revert ZeroValue();
        }
        if (firstAddrCheck == address(0) || secondAddrCheck == address(0)) {
            revert ZeroAddress();
        }
        _;
    }

    modifier zeroValuesChainAndAddressNotAllowed(uint256 appChainId, address addrCheck) {
        if (appChainId == 0) {
            revert ZeroValue();
        }
        if (addrCheck == address(0)) {
            revert ZeroAddress();
        }
        _;
    }

    /// @notice Creates a new MetabasedSequencerChain contract with a permission module
    /// @param appChainId the app chain the contract refers to
    /// @param admin The address that will be the admin
    /// @param permissionModule The address of the permission module
    /// @param salt The salt to use for the deployment, this should be the appChainId if it has not been previously used
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    function createMetabasedSequencerChain(
        uint256 appChainId,
        address admin,
        IRequirementModule permissionModule,
        bytes32 salt
    )
        public
        zeroValuesChainAndTwoAddressesNotAllowed(appChainId, admin, address(permissionModule))
        returns (address sequencerChain)
    {
        bytes memory bytecode = getBytecode(appChainId);
        address deployedAddress = Create2.deploy(0, salt, bytecode);

        MetabasedSequencerChain newSequencerChain = MetabasedSequencerChain(deployedAddress);
        newSequencerChain.initialize(admin, address(permissionModule));
        emit MetabasedSequencerChainCreated(appChainId, deployedAddress, address(permissionModule));
        return deployedAddress;
    }

    /// @notice Creates MetabasedSequencerChain with RequireAllModule
    /// @param admin The address that will be the default admin role
    /// @param appChainId The L3 chain ID
    /// @param salt The salt to use for the deployment
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return permissionModule The address of the newly created RequireAllModule
    function createMetabasedSequencerChainWithRequireAllModule(address admin, uint256 appChainId, bytes32 salt)
        public
        zeroValuesChainAndAddressNotAllowed(appChainId, admin)
        returns (address sequencerChain, IRequirementModule permissionModule)
    {
        permissionModule = IRequirementModule(new RequireAllModule(admin));
        (sequencerChain) = createMetabasedSequencerChain(appChainId, admin, permissionModule, salt);

        emit MetabasedSequencerChainCreated(appChainId, sequencerChain, address(permissionModule));

        return (sequencerChain, permissionModule);
    }

    /// @notice Creates MetabasedSequencerChain with RequireAnyModule
    /// @param admin The address that will be the default admin role
    /// @param appChainId The L3 chain ID
    /// @param salt The salt to use for the deployment
    /// @return sequencerChain The address of the newly created MetabasedSequencerChain
    /// @return permissionModule The address of the newly created RequireAnyModule
    function createMetabasedSequencerChainWithRequireAnyModule(address admin, uint256 appChainId, bytes32 salt)
        public
        zeroValuesChainAndAddressNotAllowed(appChainId, admin)
        returns (address sequencerChain, IRequirementModule permissionModule)
    {
        permissionModule = IRequirementModule(new RequireAnyModule(admin));
        (sequencerChain) = createMetabasedSequencerChain(appChainId, admin, permissionModule, salt);

        emit MetabasedSequencerChainCreated(appChainId, sequencerChain, address(permissionModule));

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
