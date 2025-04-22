pragma solidity >= 0.8.0;

import {ProposerPermissionModule} from "src/interfaces/ProposerPermissionModule.sol";

contract ProposerPermissionModuleBasic is ProposerPermissionModule {
    mapping(address => bool) allowed;

    constructor() {
        allowed[msg.sender] = true;
    }

    function isAllowed(address proposer) external view override returns (bool) {
        return allowed[proposer];
    }

    function setAllowed(address proposer, bool status) external {
        allowed[proposer] = status;
    }
}
