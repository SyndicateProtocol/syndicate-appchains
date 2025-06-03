// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

// XERC20 Interface (ERC-7281) - Sovereign Bridged Tokens
interface IXERC20 {
    /**
     * @notice Mints tokens for a user
     * @dev Can only be called by a bridge with sufficient minting limit
     * @param _user The address of the user who needs tokens minted
     * @param _amount The amount of tokens being minted
     */
    function mint(address _user, uint256 _amount) external;

    /**
     * @notice Burns tokens from a user
     * @dev Can only be called by a bridge with sufficient burning limit
     * @param _user The address of the user who needs tokens burned
     * @param _amount The amount of tokens being burned
     */
    function burn(address _user, uint256 _amount) external;

    /**
     * @notice Updates the limits of any bridge
     * @dev Can only be called by the owner
     * @param _bridge The address of the bridge we are setting the limits to
     * @param _mintingLimit The updated minting limit we are setting to the bridge
     * @param _burningLimit The updated burning limit we are setting to the bridge
     */
    function setLimits(address _bridge, uint256 _mintingLimit, uint256 _burningLimit) external;

    /**
     * @notice Returns the current minting limit of a bridge
     * @param _bridge The bridge we are viewing the limits of
     * @return _limit The limit the bridge has
     */
    function mintingCurrentLimitOf(address _bridge) external view returns (uint256 _limit);

    /**
     * @notice Returns the current burning limit of a bridge
     * @param _bridge The bridge we are viewing the limits of
     * @return _limit The limit the bridge has
     */
    function burningCurrentLimitOf(address _bridge) external view returns (uint256 _limit);

    /**
     * @notice Returns the maximum minting limit of a bridge
     * @param _bridge The bridge we are viewing the limits of
     * @return _limit The limit the bridge has
     */
    function mintingMaxLimitOf(address _bridge) external view returns (uint256 _limit);

    /**
     * @notice Returns the maximum burning limit of a bridge
     * @param _bridge The bridge we are viewing the limits of
     * @return _limit The limit the bridge has
     */
    function burningMaxLimitOf(address _bridge) external view returns (uint256 _limit);
}
