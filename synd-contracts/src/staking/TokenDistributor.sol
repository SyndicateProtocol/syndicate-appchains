// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract TokenDistributor is Ownable, Pausable {
    IERC20 public immutable token;

    mapping(address sender => bool allowed) public isAllowedSender;

    modifier onlyAllowedSenders() {
        require(isAllowedSender[msg.sender], "Must be an allowed sender");
        _;
    }

    constructor(address _token, address _admin) Ownable(_admin) {
        token = IERC20(_token);
    }

    function updateSender(address sender, bool isAllowed) external onlyOwner {
        isAllowedSender[sender] = isAllowed;
    }

    function transfer(address account, uint256 amount) external whenNotPaused onlyAllowedSenders returns (bool) {
        return IERC20(token).transfer(account, amount);
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }
}
