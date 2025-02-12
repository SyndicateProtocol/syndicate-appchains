// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract SlotTracker {
    uint256 public slotNumber;
    uint256[] public sequencingChainBlockNumbers;
    uint256[] public settlementChainBlockNumbers;

    function setSlotInfo(uint256 _slotNumber, uint256[] memory _sequencingChainBlockNumbers, uint256[] memory _settlementChainBlockNumbers) public {
        slotNumber = _slotNumber;
        sequencingChainBlockNumbers = _sequencingChainBlockNumbers;
        settlementChainBlockNumbers = _settlementChainBlockNumbers;
    }

    function getSlotInfo() public view returns (uint256, uint256[] memory, uint256[] memory) {
        return (slotNumber, sequencingChainBlockNumbers, settlementChainBlockNumbers);
    }
}
