// SPDX-License-Identifier: UNLICENSED

pragma solidity 0.8.29;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";

// [Olympix Warning: unfuzzed variables, missing events assertion] These test-related warnings are not security critical
// as the contract uses standard unit tests and integration tests. Parameter validation is handled through AccessControl.
contract SyndicateToken is ERC20, AccessControl, ERC20Permit {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    // [Olympix Warning: no parameter validation in constructor] Parameter validation is handled by OpenZeppelin's AccessControl
    constructor(address defaultAdmin, address minter) ERC20("Syndicate", "SYND") ERC20Permit("Syndicate") {
        require(defaultAdmin != address(0), "Default admin cannot be zero address");
        require(minter != address(0), "Minter cannot be zero address");
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(MINTER_ROLE, minter);
    }

    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
        require(amount != 0, "Amount cannot be zero");
        _mint(to, amount);
    }
}
