// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/**
 * @title Airdrop
 * @notice Turbo gas optimized bulk transfers of ERC20
 * @dev Inspired by the author Harrison (@PopPunkOnChain) contract GasliteDrop at https://github.com/PopPunkLLC/GasliteDrop/blob/main/contracts/src/GasliteDrop.sol
 */
contract Airdrop {
    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when arrays length mismatch
    error ArrayLengthMismatch();

    /// @notice Thrown when token transfer fails
    error TransferFailed();

    /*//////////////////////////////////////////////////////////////
                            AIRDROP FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Airdrop ERC20 tokens to a list of addresses
     * @param _token The address of the ERC20 contract
     * @param _addresses The addresses to airdrop to
     * @param _amounts The amounts to airdrop
     * @param _totalAmount The total amount to airdrop
     */
    function airdropERC20(
        address _token,
        address[] calldata _addresses,
        uint256[] calldata _amounts,
        uint256 _totalAmount
    ) external payable {
        assembly {
            // Check that the number of addresses matches the number of amounts
            if iszero(eq(_amounts.length, _addresses.length)) { revert(0, 0) }

            // transferFrom(address from, address to, uint256 amount)
            mstore(0x00, hex"23b872dd")
            // from address
            mstore(0x04, caller())
            // to address (this contract)
            mstore(0x24, address())
            // total amount
            mstore(0x44, _totalAmount)

            // transfer total amount to this contract
            if iszero(call(gas(), _token, 0, 0x00, 0x64, 0, 0)) { revert(0, 0) }

            // transfer(address to, uint256 value)
            mstore(0x00, hex"a9059cbb")

            // end of array
            let end := add(_addresses.offset, shl(5, _addresses.length))
            // diff = _addresses.offset - _amounts.offset
            let diff := sub(_addresses.offset, _amounts.offset)

            // Loop through the addresses
            for { let addressOffset := _addresses.offset } 1 {} {
                // to address
                mstore(0x04, calldataload(addressOffset))
                // amount
                mstore(0x24, calldataload(sub(addressOffset, diff)))
                // transfer the tokens
                if iszero(call(gas(), _token, 0, 0x00, 0x64, 0, 0)) { revert(0, 0) }
                // increment the address offset
                addressOffset := add(addressOffset, 0x20)
                // if addressOffset >= end, break
                if iszero(lt(addressOffset, end)) { break }
            }
        }
    }
}
