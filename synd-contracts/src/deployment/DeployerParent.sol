// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {SyndForwarder} from "./SyndForwarder.sol";

/// @title DeployerParent
/// @notice Temporary contract for controlling forwarders until we have the chain registry
contract DeployerParent is UUPSUpgradeable {
    address public immutable owner;
    SyndForwarder public immutable forwarder;

    error NotOwner();

    constructor(address owner_, address forwarder_) {
        owner = owner_;
        forwarder = SyndForwarder(forwarder_);
        _disableInitializers();
    }

    modifier onlyOwner() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    function call(address dest, bytes calldata data) external payable onlyOwner {
        forwarder.call{value: msg.value}(dest, data);
    }

    function deploy(bytes32 salt, address impl, bytes calldata init) external onlyOwner returns (address) {
        return forwarder.deploy(salt, impl, init);
    }
}
