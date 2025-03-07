
pragma solidity ^0.8.0;

import "./Assertion.sol";

interface IRollupUser {
    function fastConfirmNewAssertion(
        AssertionInputs calldata assertion,
        bytes32 expectedAssertionHash
    ) external;
}
