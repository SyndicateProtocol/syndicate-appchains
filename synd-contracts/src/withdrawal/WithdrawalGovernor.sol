// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

interface IRollupAdmin {
    function pause() external;
    function unpause() external;
    function setOwner(address newOwner) external;
}

/**
 * @title WithdrawalGovernor Contract
 */
contract WithdrawalGovernor is Ownable {
    mapping(address wallet => bool isPauser) public isPauser;

    IRollupAdmin public immutable withdrawalRollup;

    event PauserAdded(address wallet);
    event PauserRemoved(address wallet);

    modifier onlyPauser() {
        require(isPauser[msg.sender], "WithdrawalGovernor: not allowed");
        _;
    }

    constructor(address admin, address withdrawalRollupAddress) Ownable(admin) {
        withdrawalRollup = IRollupAdmin(withdrawalRollupAddress);
        isPauser[admin] = true;
        emit PauserAdded(admin);
    }

    function addPauser(address wallet) public onlyOwner {
        isPauser[wallet] = true;
        emit PauserAdded(wallet);
    }

    function removePauser(address wallet) public onlyOwner {
        isPauser[wallet] = false;   
        emit PauserRemoved(wallet);
    }

    function pauseWithdrawal() public onlyPauser {
        withdrawalRollup.pause();
    }

    function unpauseWithdrawal() public onlyOwner {
        withdrawalRollup.unpause();
    }

    function transferRollupOwner(address newOwner) public onlyOwner {
        withdrawalRollup.setOwner(newOwner);
    }
}
