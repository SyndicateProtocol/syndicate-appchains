// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MinimalUUPSStub} from "./MinimalUUPSStub.sol";

/// @title SyndForwarder
/// @notice WIP
/// @dev WIP
contract SyndForwarder {
    address public immutable allowedSender;
    address public immutable stubImplementation;

    error NotAllowedSender();

    /// @notice Constructor
    /// @param _sourceSender The source address that is allowed to call the base forwarder on ETH mainnet
    /// @param _sourceChainId The chain ID of the source chain
    constructor(address _sourceSender, uint256 _sourceChainId) {
        if (block.chainid == _sourceChainId) {
            allowedSender = _sourceSender;
        } else {
            // alias the contract address to receive messages from the same contract on the parent chain
            allowedSender = address(uint160(address(this)) + uint160(0x1111000000000000000000000000000000001111));
        }

        bytes memory stubBytecode = abi.encodePacked(type(MinimalUUPSStub).creationCode);
        stubImplementation = Create2.deploy(0, bytes32("SYND_FORWARDER_STUB_V1"), stubBytecode);
    }

    modifier onlyAllowedSender() {
        if (msg.sender != allowedSender) revert NotAllowedSender();
        _;
    }

    function call(address dest, bytes calldata data) external payable onlyAllowedSender {
        (bool success, bytes memory result) = payable(dest).call{value: msg.value}(data);
        if (!success) {
            assembly {
                revert(add(result, 32), mload(result))
            }
        }
    }

    function deploy(bytes32 salt, address impl, bytes calldata init) external onlyAllowedSender returns (address) {
        address deployAddress = Create2.deploy(0, salt, getProxyBytecode());
        (bool upgradeSuccess, bytes memory result) =
            deployAddress.call(abi.encodeWithSignature("upgradeToAndCall(address,bytes)", impl, init));
        if (!upgradeSuccess) {
            assembly {
                revert(add(result, 32), mload(result))
            }
        }
        return deployAddress;
    }

    /// @notice Returns the consistent proxy bytecode used for all deployments
    /// @dev Always returns the same bytecode for predictable CREATE2 addresses
    /// @return The bytecode to be used for deployment
    function getProxyBytecode() public view returns (bytes memory) {
        return abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(stubImplementation, ""));
    }
}
