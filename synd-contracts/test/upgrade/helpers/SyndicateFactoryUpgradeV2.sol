// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";

/// @title SyndicateFactoryUpgradeV2
/// @notice V2 upgrade for SyndicateFactory - adds new functionality safely
/// @dev This demonstrates safe upgrade patterns:
///      - New storage variables appended at end
///      - No existing storage modified
///      - New functions added
contract SyndicateFactoryUpgradeV2 is SyndicateFactory {
    /*//////////////////////////////////////////////////////////////
                    NEW V2 STORAGE - APPENDED SAFELY
    //////////////////////////////////////////////////////////////*/

    /// @notice V2: Track total number of chains created
    uint256 public totalChainsCreated;

    /// @notice V2: Mapping of chain address to creation timestamp
    mapping(address chain => uint256 timestamp) public chainCreationTimestamp;

    /// @notice V2: Enable/disable chain creation
    bool public chainCreationEnabled;

    /// @notice V2: Minimum time between chain creations
    uint256 public minTimeBetweenCreations;

    /// @notice V2: Last chain creation timestamp
    uint256 public lastChainCreationTime;

    /*//////////////////////////////////////////////////////////////
                            NEW V2 EVENTS
    //////////////////////////////////////////////////////////////*/

    event ChainCreationToggled(bool enabled);
    event MinTimeBetweenCreationsUpdated(uint256 newMinTime);
    event ChainCreationThrottled(address indexed creator, uint256 waitTime);

    /*//////////////////////////////////////////////////////////////
                        NEW V2 FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Toggle chain creation on/off
    function toggleChainCreation() external onlyRole(DEFAULT_ADMIN_ROLE) {
        chainCreationEnabled = !chainCreationEnabled;
        emit ChainCreationToggled(chainCreationEnabled);
    }

    /// @notice Set minimum time between chain creations
    /// @param _minTime Minimum time in seconds
    function setMinTimeBetweenCreations(uint256 _minTime) external onlyRole(DEFAULT_ADMIN_ROLE) {
        minTimeBetweenCreations = _minTime;
        emit MinTimeBetweenCreationsUpdated(_minTime);
    }

    /// @notice Get total chains created (V2 feature)
    /// @return Total number of chains
    function getTotalChainsCreated() external view returns (uint256) {
        return totalChainsCreated;
    }

    /// @notice Get chain creation timestamp (V2 feature)
    /// @param chainAddress Address of the chain
    /// @return Timestamp when chain was created
    function getChainCreationTimestamp(address chainAddress) external view returns (uint256) {
        return chainCreationTimestamp[chainAddress];
    }

    /// @notice Check if chain creation is allowed based on rate limiting
    /// @return bool Whether creation is currently allowed
    function isChainCreationAllowed() external view returns (bool) {
        if (!chainCreationEnabled) {
            return false;
        }

        if (minTimeBetweenCreations == 0) {
            return true;
        }

        return block.timestamp >= lastChainCreationTime + minTimeBetweenCreations;
    }

    /// @notice Get factory version (V2)
    /// @return Version string
    function factoryVersion() external pure returns (string memory) {
        return "2.0.0";
    }

    /// @notice Internal function to track chain creation (called during createSyndicateSequencingChain)
    /// @dev This would be integrated into the existing createSyndicateSequencingChain function
    function _trackChainCreation(address chainAddress) internal {
        totalChainsCreated++;
        chainCreationTimestamp[chainAddress] = block.timestamp;
        lastChainCreationTime = block.timestamp;
    }
}
