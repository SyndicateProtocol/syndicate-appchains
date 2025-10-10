// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IGasDataProvider} from "src/staking/interfaces/IGasDataProvider.sol";

/// @notice Mock gas provider: programmable per-epoch fees + active IDs
contract MockGasProvider is IGasDataProvider {
    // epoch => total fees
    mapping(uint256 => uint256) public totals;
    // epoch => appchainId => fees
    mapping(uint256 => mapping(uint256 => uint256)) public fee;
    // epoch => list of appchainIds (we keep exactly what tests set)
    mapping(uint256 => uint256[]) private idsByEpoch;

    function setFees(uint256 epoch, uint256[] memory appchainIds, uint256[] memory amounts) external {
        require(appchainIds.length == amounts.length, "length mismatch");

        // reset ids list
        delete idsByEpoch[epoch];

        uint256 t;
        for (uint256 i = 0; i < appchainIds.length; i++) {
            uint256 id = appchainIds[i];
            uint256 amt = amounts[i];
            fee[epoch][id] = amt;
            idsByEpoch[epoch].push(id);
            t += amt;
        }
        totals[epoch] = t;
    }

    function setFee(uint256 epoch, uint256 appchainId, uint256 amount) external {
        // if appchainId not in ids list, push it
        bool present = false;
        uint256[] storage ids = idsByEpoch[epoch];
        for (uint256 i = 0; i < ids.length; i++) {
            if (ids[i] == appchainId) {
                present = true;
                break;
            }
        }
        if (!present) ids.push(appchainId);

        uint256 prev = fee[epoch][appchainId];
        fee[epoch][appchainId] = amount;
        totals[epoch] = totals[epoch] + amount - prev;
    }

    function getTotalGasFees(uint256 epochIndex) external view returns (uint256) {
        require(idsByEpoch[epochIndex].length > 0);
        return totals[epochIndex];
    }

    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256) {
        require(idsByEpoch[epochIndex].length > 0);
        return fee[epochIndex][appchainId];
    }

    function getAppchainIds(uint256 epochIndex) external view returns (uint256[] memory) {
        require(idsByEpoch[epochIndex].length > 0);
        return idsByEpoch[epochIndex];
    }

    function getAppchainIds(uint256 epochIndex, uint256 startIndex, uint256 pageSize)
        external
        view
        returns (uint256[] memory)
    {
        if (startIndex >= idsByEpoch[epochIndex].length) {
            return new uint256[](0);
        }

        uint256 endIndex = startIndex + pageSize;
        if (pageSize == 0 || endIndex > idsByEpoch[epochIndex].length) {
            endIndex = idsByEpoch[epochIndex].length;
        }
        uint256 actualSize = endIndex - startIndex;

        // Create the result array with the correct size
        uint256[] memory result = new uint256[](actualSize);

        // Copy the relevant slice from the full array
        for (uint256 i = 0; i < actualSize; i++) {
            result[i] = idsByEpoch[epochIndex][startIndex + i];
        }

        return result;
    }

    function getAppchainCount(uint256 epochIndex) external view returns (uint256) {
        require(idsByEpoch[epochIndex].length > 0);
        return idsByEpoch[epochIndex].length;
    }

    function getAppchainInfo(uint256 epochIndex, uint256 startIndex, uint256 count)
        external
        view
        returns (uint256[] memory chainId, uint256[] memory gasUsed)
    {
        require(idsByEpoch[epochIndex].length > 0);
        chainId = new uint256[](count);
        gasUsed = new uint256[](count);
        for (uint256 i = 0; i < count; i++) {
            chainId[i] = idsByEpoch[epochIndex][startIndex + i];
            gasUsed[i] = fee[epochIndex][chainId[i]];
        }
    }
}
