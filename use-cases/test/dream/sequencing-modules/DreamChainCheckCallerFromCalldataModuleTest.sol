// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.29;

import {Test} from "forge-std/Test.sol";
import {DreamChainCheckCallerFromCalldataModule} from
    "src/dream/sequencing-modules/DreamChainCheckCallerFromCalldataModule.sol";
import {IAgentApplication} from "src/dream/interface/IAgentApplication.sol";
import {RLPTxDecoder} from "src/dream/RLP/RLPTxDecoder.sol";

// Mock AgentApplication contract
contract MockAgentApplication is IAgentApplication {
    mapping(address => bool) private _permittedAddresses;
    mapping(address => bool) private _adminAddresses;
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    function setPermitted(address account, bool status) external {
        _permittedAddresses[account] = status;
    }

    function setAdmin(address account, bool status) external {
        _adminAddresses[account] = status;
    }

    function isPermittedByAddress(address account) external view override returns (bool) {
        return _permittedAddresses[account];
    }

    function hasRole(bytes32 role, address account) external view override returns (bool) {
        if (role == ADMIN_ROLE) {
            return _adminAddresses[account];
        }
        return false;
    }
}

contract DreamChainCheckCallerFromCalldataModuleTest is Test {
    DreamChainCheckCallerFromCalldataModule public module;
    MockAgentApplication public mockAgentApp;
    address public permittedUser;
    address public adminUser;
    address public unpermittedUser;

    // Set up sample RLP encoded transaction data (fixed-length valid hex string)
    bytes internal sampleTxData =
        hex"f86b808504a817c80082520894b3f9704360110f807adac5c95577d4bcd58cd2ce8803ba1e47a5620aa80802ca03d60ccc517e3a8cc8da56e97f5c2bb5df825df76f4c5b62186f5a201bbb2eb0ea026ffccafd0f4dbf3a1f1e535899d2e34d21ac33fbaad84914de92a2f1d9c2dc";

    function setUp() public {
        permittedUser = address(0x1);
        adminUser = address(0x2);
        unpermittedUser = address(0x3);

        // Create and configure mock agent application
        mockAgentApp = new MockAgentApplication();
        mockAgentApp.setPermitted(permittedUser, true);
        mockAgentApp.setAdmin(adminUser, true);

        // Create the module
        module = new DreamChainCheckCallerFromCalldataModule(mockAgentApp);
    }

    function testConstructorSuccess() public {
        // Test successful construction
        DreamChainCheckCallerFromCalldataModule newModule = new DreamChainCheckCallerFromCalldataModule(mockAgentApp);
        assertEq(address(newModule.agentApplication()), address(mockAgentApp));
    }

    function testConstructorRevertOnZeroAddress() public {
        // Test construction with zero address
        vm.expectRevert(DreamChainCheckCallerFromCalldataModule.EmptyAddress.selector);
        new DreamChainCheckCallerFromCalldataModule(IAgentApplication(address(0)));
    }

    function testCalldataAllowedForPermittedUser() public {
        // Create a transaction that will decode to permittedUser
        bytes memory txData = _createSampleTxDataFrom(permittedUser);

        // Check if the calldata is allowed
        bool isAllowed = module.isCalldataAllowed(txData);
        assertTrue(isAllowed, "Permitted user's transaction should be allowed");
    }

    function testCalldataAllowedForAdminUser() public {
        // Create a transaction that will decode to adminUser
        bytes memory txData = _createSampleTxDataFrom(adminUser);

        // Check if the calldata is allowed
        bool isAllowed = module.isCalldataAllowed(txData);
        assertTrue(isAllowed, "Admin user's transaction should be allowed");
    }

    function testCalldataNotAllowedForUnpermittedUser() public {
        // Create a transaction that will decode to unpermittedUser
        bytes memory txData = _createSampleTxDataFrom(unpermittedUser);

        // Check if the calldata is allowed
        bool isAllowed = module.isCalldataAllowed(txData);
        assertFalse(isAllowed, "Unpermitted user's transaction should not be allowed");
    }

    function testPermissionChangesAffectResult() public {
        // Create a transaction that will decode to unpermittedUser
        bytes memory txData = _createSampleTxDataFrom(unpermittedUser);

        // Initially unpermitted
        bool isAllowed = module.isCalldataAllowed(txData);
        assertFalse(isAllowed, "Should not be allowed before permission change");

        // Grant permission
        mockAgentApp.setPermitted(unpermittedUser, true);

        // Now should be permitted
        isAllowed = module.isCalldataAllowed(txData);
        assertTrue(isAllowed, "Should be allowed after permission change");
    }

    function testAdminRoleChangesAffectResult() public {
        // Create a transaction that will decode to unpermittedUser
        bytes memory txData = _createSampleTxDataFrom(unpermittedUser);

        // Initially unpermitted
        bool isAllowed = module.isCalldataAllowed(txData);
        assertFalse(isAllowed, "Should not be allowed before admin role change");

        // Grant admin role
        mockAgentApp.setAdmin(unpermittedUser, true);

        // Now should be permitted due to admin role
        isAllowed = module.isCalldataAllowed(txData);
        assertTrue(isAllowed, "Should be allowed after admin role change");
    }

    // Helper function to create a transaction that will decode to a specific address
    function _createSampleTxDataFrom(address from) internal returns (bytes memory) {
        // Mock the RLPTxDecoder.decodeTx call to return our desired address
        vm.mockCall(address(RLPTxDecoder), abi.encodeWithSignature("decodeTx(bytes)"), abi.encode(from));

        return sampleTxData;
    }
}
