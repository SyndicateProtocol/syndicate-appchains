// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract SimpleERC20 is ERC20 {
    constructor() ERC20("SimpleERC20", "SMPL") {}

    function mint(address to, uint256 amount) public {
        require(amount != 0, "Amount cannot be zero");
        _mint(to, amount);
    }
}
