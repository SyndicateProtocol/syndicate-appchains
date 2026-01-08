// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

interface IGasAggregator {
    function notifyNewImplementation(address newImplementation) external;
    function notifyChainUpgrade(uint256 chainId, address newImplementation) external;
    function allowedImplementations(address implementation) external view returns (bool);
}
