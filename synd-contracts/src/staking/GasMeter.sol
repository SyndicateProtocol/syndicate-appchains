// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {EpochTracker} from "./EpochTracker.sol";
import {ReentrancyGuardTransientUpgradeable} from
    "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardTransientUpgradeable.sol";

struct GasMeterStorage {
    /// @notice Mapping of epoch to gas data
    mapping(uint256 epoch => mapping(address chainAddress => uint256 gasUsed)) gasUsed;
}

/// @title GasMeter
/// @notice Tracks gas usage for sequencing chains
/// @dev This contract is used to track gas usage for sequencing chains per epoch
contract GasMeter is
    Initializable,
    OwnableUpgradeable,
    UUPSUpgradeable,
    ReentrancyGuardTransientUpgradeable,
    EpochTracker
{
    uint256 public constant VERSION = 1_000_000; // 1.0.0

    /*//////////////////////////////////////////////////////////////
                            STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice Storage slot for GasMeter-specific data
    /// @dev Generated using: cast index-erc7201 syndicate.storage.GasMeter
    bytes32 public constant GAS_METER_STORAGE_LOCATION =
        0xfc98281d044415bf020b282d0d9074ae05a385f11f6d4a56281e2a89efbc8900;

    /// @notice Internal function to access the ERC-7201 namespaced storage
    /// @dev Uses inline assembly to access the specific storage slot for this contract's data
    /// @return $ Storage pointer to the GasMeterStorage struct
    function _getGasMeterStorage() private pure returns (GasMeterStorage storage $) {
        assembly {
            $.slot := GAS_METER_STORAGE_LOCATION
        }
    }

    /// @notice Get the gas used for a given epoch and chain address
    /// @param epoch The epoch to get the gas used for
    /// @param chainAddress The address of the chain to get the gas used for
    /// @return The gas used for the given epoch and chain address
    function gasUsed(uint256 epoch, address chainAddress) public view returns (uint256) {
        GasMeterStorage storage $ = _getGasMeterStorage();
        return $.gasUsed[epoch][chainAddress];
    }

    constructor() {
        _disableInitializers();
    }

    /// @notice Initialize the GasMeter contract
    /// @dev Initializes the GasMeter contract and sets the owner
    function initialize() external initializer {
        __Ownable_init(msg.sender);
    }

    /// @notice Meter a call and track the gas used
    /// @dev Meters the gas used for a call and tracks it in the gas used mapping
    /// @param meteredCall The call to track gas for
    function meterCall(bytes calldata meteredCall) public nonReentrant {
        uint256 startGas = gasleft();
        (bool success, bytes memory result) = address(msg.sender).call(meteredCall);
        if (!success) {
            assembly {
                revert(add(result, 32), mload(result))
            }
        }

        uint256 gasPrice = tx.gasprice == 0 ? 1 : tx.gasprice;
        _getGasMeterStorage().gasUsed[getCurrentEpoch()][msg.sender] += (startGas - gasleft()) * gasPrice;
    }

    /// @notice Authorizes the upgrade of the GasMeter contract
    /// @dev Authorizes the upgrade of the GasMeter contract only if the caller is the owner
    /// @param newImplementation The address of the new implementation contract
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
