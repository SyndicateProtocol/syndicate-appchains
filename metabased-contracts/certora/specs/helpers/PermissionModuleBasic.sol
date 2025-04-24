pragma solidity >= 0.8.0;

import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";

contract PermissionModuleBasic is IPermissionModule {
    mapping(address => bool) allowed;

    constructor() {
        allowed[msg.sender] = true;
    }

    function isAllowed(address proposer, address, bytes calldata) external view override returns (bool) {
        return allowed[proposer];
    }

    function setAllowed(address proposer, bool status) external {
        allowed[proposer] = status;
    }
}
