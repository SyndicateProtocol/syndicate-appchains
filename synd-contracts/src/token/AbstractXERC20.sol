// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit, Nonces} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ERC20Votes} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";

import {IXERC20} from "./interfaces/IXERC20.sol";

/**
 * @title AbstractXERC20
 * @notice Abstract contract implementing XERC20 (ERC-7281) functionality
 * @dev This contract provides the complete XERC20 implementation that can be inherited
 *      by any token contract that needs cross-chain bridging capabilities
 */
abstract contract AbstractXERC20 is ERC20, AccessControl, ERC20Permit, ERC20Votes, IXERC20 {
    error ZeroAddress();
    error ZeroAmount();
    error InsufficientLimit();
    error BridgeNotAuthorized();

    bytes32 public constant BRIDGE_MANAGER_ROLE = keccak256("BRIDGE_MANAGER_ROLE");

    uint256 public constant BRIDGE_LIMIT_DURATION = 1 days; // Bridge limits reset daily

    // Bridge limit tracking structure
    struct BridgeLimits {
        uint256 mintingMaxLimit; // Maximum minting limit per duration
        uint256 mintingCurrentLimit; // Current available minting limit
        uint256 burningMaxLimit; // Maximum burning limit per duration
        uint256 burningCurrentLimit; // Current available burning limit
        uint256 lastUpdate; // Last time limits were updated
    }

    // State variables
    mapping(address => BridgeLimits) public bridgeLimits;
    mapping(address => bool) public authorizedBridges;

    // Events
    event BridgeLimitsSet(address indexed bridge, uint256 mintingLimit, uint256 burningLimit);
    event BridgeAuthorized(address indexed bridge);
    event BridgeDeauthorized(address indexed bridge);

    /**
     * @dev Constructor for AbstractXERC20
     * @param name_ Token name
     * @param symbol_ Token symbol
     */
    //#olympix-ignore-no-parameter-validation-in-constructor
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) ERC20Permit(name_) ERC20Votes() {
        // Abstract contract - roles will be set in implementing contracts
    }

    // =============================================================================
    // XERC20 INTERFACE IMPLEMENTATION
    // =============================================================================

    /**
     * @notice XERC20 mint function - Called by authorized bridges
     * @param _user The address to mint tokens to
     * @param _amount The amount of tokens to mint
     */
    function mint(address _user, uint256 _amount) external virtual override {
        if (_user == address(0)) revert ZeroAddress();
        if (_amount == 0) revert ZeroAmount();
        if (!authorizedBridges[msg.sender]) revert BridgeNotAuthorized();

        _updateBridgeLimits(msg.sender);

        if (bridgeLimits[msg.sender].mintingCurrentLimit < _amount) {
            revert InsufficientLimit();
        }

        // Reduce the current minting limit
        bridgeLimits[msg.sender].mintingCurrentLimit -= _amount;

        _mint(_user, _amount);
    }

    /**
     * @notice XERC20 burn function - Called by authorized bridges
     * @param _user The address to burn tokens from
     * @param _amount The amount of tokens to burn
     */
    function burn(address _user, uint256 _amount) external virtual override {
        if (_user == address(0)) revert ZeroAddress();
        if (_amount == 0) revert ZeroAmount();
        if (!authorizedBridges[msg.sender]) revert BridgeNotAuthorized();

        _updateBridgeLimits(msg.sender);

        if (bridgeLimits[msg.sender].burningCurrentLimit < _amount) {
            revert InsufficientLimit();
        }

        // Reduce the current burning limit
        bridgeLimits[msg.sender].burningCurrentLimit -= _amount;

        _burn(_user, _amount);
    }

    /**
     * @notice Set the limits of a bridge
     * @param _bridge The bridge address
     * @param _mintingLimit The minting limit per duration
     * @param _burningLimit The burning limit per duration
     */
    function setLimits(address _bridge, uint256 _mintingLimit, uint256 _burningLimit)
        external
        virtual
        override
        onlyRole(BRIDGE_MANAGER_ROLE)
    {
        if (_bridge == address(0)) revert ZeroAddress();

        // If bridge wasn't authorized before and has non-zero limits, authorize it
        if (!authorizedBridges[_bridge] && (_mintingLimit > 0 || _burningLimit > 0)) {
            authorizedBridges[_bridge] = true;
            emit BridgeAuthorized(_bridge);
        }

        // If both limits are zero, deauthorize the bridge
        if (_mintingLimit == 0 && _burningLimit == 0 && authorizedBridges[_bridge]) {
            authorizedBridges[_bridge] = false;
            emit BridgeDeauthorized(_bridge);
        }

        _updateBridgeLimits(_bridge);

        bridgeLimits[_bridge].mintingMaxLimit = _mintingLimit;
        bridgeLimits[_bridge].burningMaxLimit = _burningLimit;

        // Reset current limits to max when setting new limits
        bridgeLimits[_bridge].mintingCurrentLimit = _mintingLimit;
        bridgeLimits[_bridge].burningCurrentLimit = _burningLimit;

        emit BridgeLimitsSet(_bridge, _mintingLimit, _burningLimit);
    }

    /**
     * @notice Get the current minting limit of a bridge
     */
    function mintingCurrentLimitOf(address _bridge) external view virtual override returns (uint256) {
        return _calculateCurrentLimit(
            bridgeLimits[_bridge].mintingCurrentLimit,
            bridgeLimits[_bridge].mintingMaxLimit,
            bridgeLimits[_bridge].lastUpdate
        );
    }

    /**
     * @notice Get the current burning limit of a bridge
     */
    function burningCurrentLimitOf(address _bridge) external view virtual override returns (uint256) {
        return _calculateCurrentLimit(
            bridgeLimits[_bridge].burningCurrentLimit,
            bridgeLimits[_bridge].burningMaxLimit,
            bridgeLimits[_bridge].lastUpdate
        );
    }

    /**
     * @notice Get the maximum minting limit of a bridge
     */
    function mintingMaxLimitOf(address _bridge) external view virtual override returns (uint256) {
        return bridgeLimits[_bridge].mintingMaxLimit;
    }

    /**
     * @notice Get the maximum burning limit of a bridge
     */
    function burningMaxLimitOf(address _bridge) external view virtual override returns (uint256) {
        return bridgeLimits[_bridge].burningMaxLimit;
    }

    // =============================================================================
    // BRIDGE MANAGEMENT FUNCTIONS
    // =============================================================================

    /**
     * @notice Check if an address is an authorized bridge
     * @param bridge The bridge address to check
     * @return True if the bridge is authorized
     */
    function isBridge(address bridge) external view returns (bool) {
        return authorizedBridges[bridge];
    }

    /**
     * @notice Get comprehensive bridge information
     * @param bridge The bridge address
     * @return authorized Whether the bridge is authorized
     * @return mintingMax Maximum minting limit
     * @return mintingCurrent Current minting limit (calculated)
     * @return burningMax Maximum burning limit
     * @return burningCurrent Current burning limit (calculated)
     */
    function getBridgeInfo(address bridge)
        external
        view
        returns (
            bool authorized,
            uint256 mintingMax,
            uint256 mintingCurrent,
            uint256 burningMax,
            uint256 burningCurrent
        )
    {
        authorized = authorizedBridges[bridge];
        mintingMax = bridgeLimits[bridge].mintingMaxLimit;
        burningMax = bridgeLimits[bridge].burningMaxLimit;
        mintingCurrent = this.mintingCurrentLimitOf(bridge);
        burningCurrent = this.burningCurrentLimitOf(bridge);
    }

    /**
     * @notice Get bridge capacity utilization
     * @param bridge The bridge address
     * @return mintingUtilization Percentage of minting capacity used (in basis points)
     * @return burningUtilization Percentage of burning capacity used (in basis points)
     */
    function getBridgeUtilization(address bridge)
        external
        view
        returns (uint256 mintingUtilization, uint256 burningUtilization)
    {
        uint256 mintingMax = bridgeLimits[bridge].mintingMaxLimit;
        uint256 burningMax = bridgeLimits[bridge].burningMaxLimit;
        uint256 mintingCurrent = this.mintingCurrentLimitOf(bridge);
        uint256 burningCurrent = this.burningCurrentLimitOf(bridge);

        if (mintingMax > 0) {
            mintingUtilization = ((mintingMax - mintingCurrent) * 10000) / mintingMax;
        }

        if (burningMax > 0) {
            burningUtilization = ((burningMax - burningCurrent) * 10000) / burningMax;
        }
    }

    // =============================================================================
    // INTERNAL FUNCTIONS
    // =============================================================================

    /**
     * @dev Update bridge limits based on time elapsed
     * @param _bridge The bridge to update limits for
     */
    function _updateBridgeLimits(address _bridge) internal {
        BridgeLimits storage limits = bridgeLimits[_bridge];

        if (limits.lastUpdate == 0) {
            limits.lastUpdate = block.timestamp;
            return;
        }

        uint256 elapsed = block.timestamp - limits.lastUpdate;
        if (elapsed >= BRIDGE_LIMIT_DURATION) {
            // Full duration has passed, reset to max limits
            limits.mintingCurrentLimit = limits.mintingMaxLimit;
            limits.burningCurrentLimit = limits.burningMaxLimit;
        } else if (elapsed > 0) {
            // Partial replenishment based on time elapsed
            uint256 mintingReplenished = (limits.mintingMaxLimit * elapsed) / BRIDGE_LIMIT_DURATION;
            uint256 burningReplenished = (limits.burningMaxLimit * elapsed) / BRIDGE_LIMIT_DURATION;

            limits.mintingCurrentLimit = _min(limits.mintingCurrentLimit + mintingReplenished, limits.mintingMaxLimit);
            limits.burningCurrentLimit = _min(limits.burningCurrentLimit + burningReplenished, limits.burningMaxLimit);
        }

        limits.lastUpdate = block.timestamp;
    }

    /**
     * @dev Calculate current limit based on time elapsed (view function)
     */
    function _calculateCurrentLimit(uint256 currentLimit, uint256 maxLimit, uint256 lastUpdate)
        internal
        view
        returns (uint256)
    {
        if (lastUpdate == 0) return 0;

        uint256 elapsed = block.timestamp - lastUpdate;
        if (elapsed >= BRIDGE_LIMIT_DURATION) return maxLimit;

        uint256 replenished = (maxLimit * elapsed) / BRIDGE_LIMIT_DURATION;
        uint256 newLimit = currentLimit + replenished;

        return newLimit > maxLimit ? maxLimit : newLimit;
    }

    /**
     * @dev Internal helper function to get minimum of two values
     */
    function _min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    // =============================================================================
    // GOVERNANCE HELPER FUNCTIONS
    // =============================================================================

    /**
     * @notice Get voting power of an account at current block
     * @param account The account to check
     * @return The current voting power
     */
    function getVotingPower(address account) external view returns (uint256) {
        return getVotes(account);
    }

    /**
     * @notice Get past voting power of an account at a specific block
     * @param account The account to check
     * @param blockNumber The block number to check
     * @return The voting power at that block
     */
    function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256) {
        return getPastVotes(account, blockNumber);
    }

    /**
     * @notice Get total supply at current block
     * @return Current total supply
     */
    function getCurrentTotalSupply() external view returns (uint256) {
        return totalSupply();
    }

    // =============================================================================
    // REQUIRED OVERRIDES FOR MULTIPLE INHERITANCE
    // =============================================================================

    /**
     * @dev Override required by Solidity for multiple inheritance
     */
    function _update(address from, address to, uint256 value) internal virtual override(ERC20, ERC20Votes) {
        super._update(from, to, value);
    }

    /**
     * @dev Override required by Solidity for multiple inheritance
     */
    function nonces(address owner) public view virtual override(ERC20Permit, Nonces) returns (uint256) {
        return super.nonces(owner);
    }
}
