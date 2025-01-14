pragma solidity >= 0.8.0;

import {PermissionModule} from "src/interfaces/PermissionModule.sol";

contract PermissionModuleBasic is PermissionModule {
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
