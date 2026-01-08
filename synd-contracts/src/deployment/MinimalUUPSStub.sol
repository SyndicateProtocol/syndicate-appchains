// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

/// @title MinimalUUPSStub
/// @notice Minimal UUPS implementation stub for deterministic proxy deployments
/// @dev This contract will NEVER change to ensure deterministic CREATE2 addresses across all deployments.
///      It serves as a temporary implementation that is immediately upgraded after proxy deployment.
///      The stub has no functionality except for UUPS upgrade capability and security measures.
contract MinimalUUPSStub is UUPSUpgradeable {
    /// @notice this is only used to get a reliably deterministic address, the proxy will immediately be upgraded
    function _authorizeUpgrade(address) internal view override {}

    /// @notice Receive function that reverts - this stub should not receive ETH
    receive() external payable {
        revert("Stub: ETH not accepted");
    }

    /// @notice Fallback that reverts - this stub has no logic
    fallback() external payable {
        revert("Stub: no logic implemented");
    }
}
