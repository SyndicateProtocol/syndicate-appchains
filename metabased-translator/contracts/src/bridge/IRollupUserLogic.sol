
pragma solidity ^0.8.0;

import "../rollup/Assertion.sol";
import "./IRollupCore.sol";

interface IRollupUser is IRollupCore {
    function fastConfirmNewAssertion(
        AssertionInputs calldata assertion,
        bytes32 expectedAssertionHash
    ) external;

    function computeAssertionHash(
        bytes32 prevAssertionHash,
        AssertionState calldata state,
        bytes32 inboxAcc
    ) external returns (bytes32);

}
