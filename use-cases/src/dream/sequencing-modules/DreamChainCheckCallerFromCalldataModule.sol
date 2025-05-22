// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RLPTxDecoder} from "../RLP/RLPTxDecoder.sol";

import {IAgentApplication} from "../interface/IAgentApplication.sol";

interface CalldataPermissionModule {
    /**
     * @notice Checks if the calldata is allowed.
     * @param data The calldata to be checked.
     * @return bool indicating if the calldata is allowed.
     */
    function isCalldataAllowed(bytes calldata data) external view returns (bool);
}

/**
 * @title DreamChainCheckCallerFromCalldataModule
 * @dev it checks whether caller is allowed from calldata.
 */
contract DreamChainCheckCallerFromCalldataModule is CalldataPermissionModule {
    /// @notice The AgentApplication contract for checking permissions
    IAgentApplication public immutable agentApplication;

    /// @notice Error for empty address
    error EmptyAddress();

    /// @notice Construct the DreamChainCheckCallerFromCalldataModule
    /// @param _agentApplication The address of the AgentApplication contract
    constructor(IAgentApplication _agentApplication) {
        if (address(_agentApplication) == address(0)) revert EmptyAddress();

        agentApplication = _agentApplication;
    }

    /**
     * @notice checks if calldata caller is allowed.
     * @return bool indicating whether is allowed.
     */
    function isCalldataAllowed(bytes calldata data) external view override returns (bool) {
        address from = RLPTxDecoder.decodeTx(data);

        // Check if the from address is permitted
        if (!agentApplication.isPermittedByAddress(from)) {
            // then must be owner of the AgentApplication contract, otherwise revert
            bytes32 adminRole = agentApplication.ADMIN_ROLE();
            if (!agentApplication.hasRole(adminRole, from)) {
                return false;
            }
        }

        return true;
    }
}
