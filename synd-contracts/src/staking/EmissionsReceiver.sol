// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IEmissionsReceiver} from "./interfaces/IEmissionsReceiver.sol";

// Owner will be transferred from msg.sender to the forwarder contract after deployment
contract EmissionsReceiver is IEmissionsReceiver, Ownable(msg.sender) {
    event RewardsReceiverUpdate(uint256 indexed chainID, address receiver);

    mapping(uint256 appchainId => address receiver) public appchainEmissionsReceiver;

    // In the future, the chain registry contract will call setAppchainEmissionsReceiver()
    // when registering/creating appchains to ensure that all appchain funds are claimable.
    function setAppchainEmissionsReceiver(uint256 chainID, address receiver) external onlyOwner {
        appchainEmissionsReceiver[chainID] = receiver;
        emit RewardsReceiverUpdate(chainID, receiver);
    }
}
