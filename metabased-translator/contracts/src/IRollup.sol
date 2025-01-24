// SPDX-License-Identifier: UNLICENSED

// solhint-disable-next-line compiler-version
pragma solidity >=0.6.9 <0.9.0;

interface IRollup {
    function deliverMessage(uint8 kind, address sender, bytes memory messageData) external;
    function postBatch(uint256 afterDelayedMessagesRead, bytes calldata data) external;
}