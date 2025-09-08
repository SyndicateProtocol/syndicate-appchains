// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "src/requirement-modules/RequireCompositeModule.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract SyndicateFactoryTest is Test {
    SyndicateFactory public factory;
    address public admin;
    address public manager;
    address public nonManager;
    uint256 public appchainId = 10042001;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    // Events
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    event ChainIdManuallyMarked(uint256 indexed chainId);

    event DeterministicChainIdGenerated(address indexed sender, uint256 indexed nonce, uint256 indexed chainId);

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        nonManager = address(0x3);
        // Deploy implementation and proxy
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        factory = SyndicateFactory(address(proxy));

        // Grant manager role to the manager address
        vm.prank(admin);
        factory.grantRole(MANAGER_ROLE, manager);
    }

    function testCreateSequencingChainWithRequireAndModule() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address permissionModuleAddress = address(permissionModule);

        address expectedAddress = factory.computeSequencingChainAddress(appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, permissionModule);

        assertTrue(sequencingChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appchainId);
        assertEq(sequencingChainAddress, expectedAddress);

        SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(sequencingChainAddress);

        // Verify sequencer setup
        assertEq(address(sequencingChain), sequencingChainAddress);
        assertEq(sequencingChain.appchainId(), appchainId);

        // Verify permission module setup
        assertEq(address(sequencingChain.permissionRequirementModule()), permissionModuleAddress);
        assertEq(permissionModule.owner(), admin);
    }

    function testCreateSequencingChainWithRequireOrModule() public {
        RequireOrModule permissionModule = new RequireOrModule(admin);
        address permissionModuleAddress = address(permissionModule);

        address expectedAddress = factory.computeSequencingChainAddress(appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, IRequirementModule(permissionModule));

        assertTrue(sequencingChainAddress != address(0));
        assertTrue(permissionModuleAddress != address(0));
        assertEq(actualChainId, appchainId);
        assertEq(sequencingChainAddress, expectedAddress);

        SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(sequencingChainAddress);

        // Verify sequencer setup
        assertEq(address(sequencingChain), sequencingChainAddress);
        assertEq(sequencingChain.appchainId(), appchainId);

        // Verify permission module setup
        assertEq(address(sequencingChain.permissionRequirementModule()), permissionModuleAddress);
        assertEq(permissionModule.owner(), admin);
    }

    function testCreateSequencingChainWithRequireCompositeModule() public {
        RequireCompositeModule permissionModule = new RequireCompositeModule(admin);
        address permissionModuleAddress = address(permissionModule);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, IRequirementModule(permissionModule));

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, appchainId);

        SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(sequencingChainAddress);
        assertEq(sequencingChain.appchainId(), appchainId);
        assertEq(address(sequencingChain.permissionRequirementModule()), permissionModuleAddress);
        assertEq(permissionModule.owner(), admin);
    }

    function testCorrectAppChainIdAssignment() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        uint256 differentChainId = 10042002;

        vm.prank(admin);
        (address sequencingChain1, uint256 actualChainId1) = factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );
        vm.prank(admin);
        (address sequencingChain2, uint256 actualChainId2) = factory.createSyndicateSequencingChainWithCustomId(
            differentChainId, admin, IRequirementModule(address(permissionModule2))
        );

        assertEq(SyndicateSequencingChain(sequencingChain1).appchainId(), appchainId);
        assertEq(SyndicateSequencingChain(sequencingChain2).appchainId(), differentChainId);
        assertEq(actualChainId1, appchainId);
        assertEq(actualChainId2, differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, address(0), IRequirementModule(address(permissionModule))
        );
    }

    function testRevertsOnZeroPermissionModule() public {
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, IRequirementModule(address(0)));
    }

    function testCreateSequencingChainWithCustomId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        uint256 customChainId = 1001;

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChainWithCustomId(
            customChainId, admin, IRequirementModule(address(permissionModule))
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, customChainId);
        assertEq(SyndicateSequencingChain(sequencingChainAddress).appchainId(), customChainId);
    }

    function testCreateSequencingChainAddressIsDeterministic() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        address expectedAddress = factory.computeSequencingChainAddress(appchainId);
        vm.prank(admin);
        (address sequencingChainAddress,) = factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );

        assertEq(sequencingChainAddress, expectedAddress);
    }

    function testGetProxyBytecode() public view {
        bytes memory bytecode = factory.getProxyBytecode();
        bytes memory expectedBytecode =
            abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(factory.stubImplementation(), ""));
        assertEq(bytecode, expectedBytecode);
    }

    function testGetImplBytecode() public {
        address impl = address(new SyndicateSequencingChain());
        bytes memory bytecode = factory.getImplBytecode(impl);
        bytes memory expectedBytecode = abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(impl, ""));
        assertEq(bytecode, expectedBytecode);
    }

    function testCreateMultipleSequencingChainsWithCustomIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        uint256 chainId1 = 1001;
        uint256 chainId2 = 1002;
        uint256 chainId3 = 1003;

        // First chain
        vm.prank(admin);
        (, uint256 id1) = factory.createSyndicateSequencingChainWithCustomId(
            chainId1, admin, IRequirementModule(address(permissionModule1))
        );
        assertEq(id1, chainId1);

        // Second chain
        vm.prank(admin);
        (, uint256 id2) = factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );
        assertEq(id2, chainId2);

        // Third chain
        vm.prank(admin);
        (, uint256 id3) = factory.createSyndicateSequencingChainWithCustomId(
            chainId3, admin, IRequirementModule(address(permissionModule3))
        );
        assertEq(id3, chainId3);
    }

    function testCreateSequencingChainsWithMixedChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // First custom chain ID
        uint256 chainId1 = 2001;
        vm.prank(admin);
        (, uint256 id1) = factory.createSyndicateSequencingChainWithCustomId(
            chainId1, admin, IRequirementModule(address(permissionModule1))
        );
        assertEq(id1, chainId1);

        // Different custom chain ID
        uint256 chainId2 = 42000;
        vm.prank(admin);
        (, uint256 id2) = factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );
        assertEq(id2, chainId2);

        // Another custom chain ID
        uint256 chainId3 = 3001;
        vm.prank(admin);
        (, uint256 id3) = factory.createSyndicateSequencingChainWithCustomId(
            chainId3, admin, IRequirementModule(address(permissionModule3))
        );
        assertEq(id3, chainId3);
    }

    function testChainIdAlreadyExists() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create first chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule1))
        );

        // Try to create another with same chain ID
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule2))
        );
    }

    function testIsChainIdUsed() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        uint256 chainId2 = 2002;

        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appchainId), false);
        assertEq(factory.isChainIdUsed(chainId2), false);

        // Create first chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule1))
        );

        // Now first chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appchainId), true);
        assertEq(factory.isChainIdUsed(chainId2), false);

        // Create second chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );

        // Now both chain IDs should be marked as used
        assertEq(factory.isChainIdUsed(appchainId), true);
        assertEq(factory.isChainIdUsed(chainId2), true);
    }

    // Pausability Tests
    function testPauseUnpause() public {
        // Initially not paused
        assertFalse(factory.paused());

        // Admin can pause
        vm.prank(admin);
        factory.pause();
        assertTrue(factory.paused());

        // Admin can unpause
        vm.prank(admin);
        factory.unpause();
        assertFalse(factory.paused());
    }

    function testPauseNonAdminReverts() public {
        vm.prank(manager);
        vm.expectRevert(); // AccessControl will revert
        factory.pause();

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert
        factory.pause();
    }

    function testUnpauseNonAdminReverts() public {
        // Pause first
        vm.prank(admin);
        factory.pause();

        vm.prank(manager);
        vm.expectRevert(); // AccessControl will revert
        factory.unpause();

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl revert
        factory.unpause();
    }

    function testCreateSequencingChainWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create sequencing chain
        vm.expectRevert(); // Pausable will revert with "Pausable: paused"
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );
    }

    function testCreateSequencingChainAfterUnpauseWorks() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Pause then unpause
        vm.prank(admin);
        factory.pause();
        vm.prank(admin);
        factory.unpause();

        // Should work after unpause
        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChainWithCustomId(
            appchainId, admin, IRequirementModule(address(permissionModule))
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, appchainId);
    }

    // Access Control tests
    function testRoleSetup() public view {
        // Admin should have the default admin role
        assertTrue(factory.hasRole(DEFAULT_ADMIN_ROLE, admin));

        // Admin should have the manager role
        assertTrue(factory.hasRole(MANAGER_ROLE, admin));

        // Manager should have the manager role
        assertTrue(factory.hasRole(MANAGER_ROLE, manager));

        // Non-manager should not have any roles
        assertFalse(factory.hasRole(DEFAULT_ADMIN_ROLE, nonManager));
        assertFalse(factory.hasRole(MANAGER_ROLE, nonManager));
    }

    function testPublicVariables() public view {
        // Test that variables are publicly accessible
        assertEq(factory.appchainContracts(appchainId), address(0));
    }

    function testInitializeWithZeroAddressReverts() public {
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (address(0)));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        new ERC1967Proxy(address(implementation), initData);
    }

    function testLargeChainIdNumbers() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Test with a large custom chain ID
        uint256 largeChainId = 999999999;

        vm.prank(admin);
        (, uint256 id) = factory.createSyndicateSequencingChainWithCustomId(
            largeChainId, admin, IRequirementModule(address(permissionModule))
        );
        assertEq(id, largeChainId);
    }

    function testSequencingChainImplementationAddress() public view {
        // Test that implementation address is set and not zero
        uint256 chainId = 1001; // Use a test chain ID
        address impl = factory.computeSequencingChainAddress(chainId);
        assertTrue(impl != address(0));
    }

    // Edge cases and fuzz tests
    function testFuzzManualChainIds(uint256 chainId) public {
        // Skip invalid chain IDs
        vm.assume(chainId != 0);
        vm.assume(chainId < type(uint256).max / 2); // Avoid overflow
        vm.assume(factory.isChainIdUsed(chainId) == false);

        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.prank(admin);
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChainWithCustomId(
            chainId, admin, IRequirementModule(address(permissionModule))
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, chainId);
        assertEq(factory.isChainIdUsed(chainId), true);
    }

    function testChainIdNoCollisions() public {
        // Test that different custom chain IDs produce different results
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireAndModule permissionModule2 = new RequireAndModule(admin);

        uint256 chainId1 = 1001;
        uint256 chainId2 = 2001;

        vm.prank(admin);
        (, uint256 actualId1) = factory.createSyndicateSequencingChainWithCustomId(
            chainId1, admin, IRequirementModule(address(permissionModule1))
        );

        vm.prank(admin);
        (, uint256 actualId2) = factory.createSyndicateSequencingChainWithCustomId(
            chainId2, admin, IRequirementModule(address(permissionModule2))
        );

        // These should be completely different
        assertTrue(actualId1 != actualId2);
        assertEq(actualId1, chainId1);
        assertEq(actualId2, chainId2);
    }

    function testGetContractsForAppchains() public {
        // Create a few chains with non-sequential chain IDs
        RequireAndModule permissionModule = new RequireAndModule(admin);

        uint256 appchainId1 = 100;
        uint256 appchainId2 = 200;
        uint256 appchainId3 = 300;

        vm.prank(admin);
        (address chain1,) = factory.createSyndicateSequencingChainWithCustomId(appchainId1, admin, permissionModule);

        vm.prank(admin);
        (address chain2,) = factory.createSyndicateSequencingChainWithCustomId(appchainId2, admin, permissionModule);

        vm.prank(admin);
        (address chain3,) = factory.createSyndicateSequencingChainWithCustomId(appchainId3, admin, permissionModule);

        // Test getContractsForGasTracking with specific chain IDs
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = appchainId2; // 200
        chainIDs[1] = appchainId1; // 100

        address[] memory contracts = factory.getContractsForGasTracking(chainIDs);

        // Verify that the correct contracts are returned for each chain ID
        // This is the regression test for the bug where it was using the loop index instead of the chain ID
        assertEq(contracts.length, 2);
        assertEq(contracts[0], chain2); // Contract for chain ID 200
        assertEq(contracts[1], chain1); // Contract for chain ID 100

        // Test with a single chain ID
        uint256[] memory singleChainID = new uint256[](1);
        singleChainID[0] = appchainId3; // 300

        address[] memory singleContract = factory.getContractsForGasTracking(singleChainID);

        assertEq(singleContract.length, 1);
        assertEq(singleContract[0], chain3); // Contract for chain ID 300
    }

    function testGetAppchainsAndContracts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        uint256 appchainId1 = 1000;
        uint256 appchainId2 = 2000;
        uint256 appchainId3 = 3000;

        vm.prank(admin);
        (address chain1Addr,) = factory.createSyndicateSequencingChainWithCustomId(appchainId1, admin, permissionModule);
        vm.prank(admin);
        (address chain2Addr,) = factory.createSyndicateSequencingChainWithCustomId(appchainId2, admin, permissionModule);
        vm.prank(admin);
        (address chain3Addr,) = factory.createSyndicateSequencingChainWithCustomId(appchainId3, admin, permissionModule);

        (uint256[] memory chainIDs, address[] memory contracts) = factory.getAppchainsAndContractsForGasTracking();
        assertEq(chainIDs.length, 3);
        assertEq(contracts.length, 3);
        assertEq(contracts[0], chain1Addr);
        assertEq(contracts[1], chain2Addr);
        assertEq(contracts[2], chain3Addr);

        // ensure the mapping is consistent
        for (uint256 i = 0; i < chainIDs.length; i++) {
            assertEq(contracts[i], factory.appchainContracts(chainIDs[i]));
        }
    }

    // Implementation Allowlist Tests
    function testAddAllowedImplementation() public {
        SyndicateSequencingChain mockImpl = new SyndicateSequencingChain();

        // Initially not allowed
        assertFalse(factory.isImplementationAllowed(address(mockImpl)));

        vm.expectEmit(true, false, false, false);
        emit SyndicateFactory.ImplementationAdded(address(mockImpl));

        vm.prank(admin);
        factory.addAllowedImplementation(address(mockImpl), false);

        // Now should be allowed
        assertTrue(factory.isImplementationAllowed(address(mockImpl)));

        // Check that it was added to the array
        address[] memory allowedImplementations = factory.getAllowedImplementations();
        bool found = false;
        for (uint256 i = 0; i < allowedImplementations.length; i++) {
            if (allowedImplementations[i] == address(mockImpl)) {
                found = true;
                break;
            }
        }
        assertTrue(found);
    }

    function testAddAllowedImplementationNonAdminReverts() public {
        SyndicateSequencingChain mockImpl = new SyndicateSequencingChain();

        vm.prank(manager);
        vm.expectRevert(); // AccessControl revert
        factory.addAllowedImplementation(address(mockImpl), false);

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl revert
        factory.addAllowedImplementation(address(mockImpl), false);
    }

    function testAddDuplicateImplementationReverts() public {
        SyndicateSequencingChain mockImpl = new SyndicateSequencingChain();

        vm.prank(admin);
        factory.addAllowedImplementation(address(mockImpl), false);

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ImplementationAlreadyAllowed.selector);
        factory.addAllowedImplementation(address(mockImpl), false);
    }

    function testNotifyChainUpgradeWithAllowedImplementation() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Create a chain
        vm.prank(admin);
        (address chainAddr, uint256 chainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, permissionModule);

        // Add allowed implementation
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();
        vm.prank(admin);
        factory.addAllowedImplementation(address(newImpl), false);

        // Chain should be able to notify about upgrade with allowed implementation
        vm.prank(chainAddr);
        factory.notifyChainUpgrade(chainId, address(newImpl));

        // Chain should not be banned
        assertFalse(factory.isChainBannedFromGasTracking(chainId));
    }

    function testNotifyChainUpgradeWithDisallowedImplementation() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Create a chain
        vm.prank(admin);
        (address chainAddr, uint256 chainId) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, permissionModule);

        // Create implementation that is not allowed
        SyndicateSequencingChain disallowedImpl = new SyndicateSequencingChain();

        vm.expectEmit(true, true, false, false);
        emit SyndicateFactory.ChainBannedFromGasTracking(chainId, address(disallowedImpl));

        // Chain notifies about upgrade with disallowed implementation
        vm.prank(chainAddr);
        factory.notifyChainUpgrade(chainId, address(disallowedImpl));

        // Chain should be banned from gas tracking
        assertTrue(factory.isChainBannedFromGasTracking(chainId));
    }

    function testNotifyChainUpgradeUnauthorizedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Create a chain
        vm.prank(admin);
        (, uint256 chainId) = factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, permissionModule);

        SyndicateSequencingChain someImpl = new SyndicateSequencingChain();

        // Non-chain address cannot notify
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.OnlyChainCanNotifyUpgrade.selector);
        factory.notifyChainUpgrade(chainId, address(someImpl));
    }

    function testBanChainFromGasTracking() public {
        SyndicateSequencingChain notAllowedImpl = new SyndicateSequencingChain();

        vm.expectEmit(true, true, false, false);
        emit SyndicateFactory.ChainBannedFromGasTracking(appchainId, address(notAllowedImpl));

        vm.prank(admin);
        factory.banChainFromGasTracking(appchainId, address(notAllowedImpl));

        assertTrue(factory.isChainBannedFromGasTracking(appchainId));
    }

    function testBanChainFromGasTrackingNonAdminReverts() public {
        SyndicateSequencingChain notAllowedImpl = new SyndicateSequencingChain();

        vm.prank(manager);
        vm.expectRevert(); // AccessControl revert
        factory.banChainFromGasTracking(appchainId, address(notAllowedImpl));

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl revert
        factory.banChainFromGasTracking(appchainId, address(notAllowedImpl));
    }

    function testGetAllowedImplementations() public {
        SyndicateSequencingChain impl1 = new SyndicateSequencingChain();
        SyndicateSequencingChain impl2 = new SyndicateSequencingChain();

        // Initially has one implementation (from constructor)
        address[] memory allowedImplementations = factory.getAllowedImplementations();
        assertEq(allowedImplementations.length, 1);

        // Add first implementation
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl1), false);

        allowedImplementations = factory.getAllowedImplementations();
        assertEq(allowedImplementations.length, 2);

        // Add second implementation
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl2), false);

        allowedImplementations = factory.getAllowedImplementations();
        assertEq(allowedImplementations.length, 3);
    }

    function testNewChainsUseLatestImplementation() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Deploy first chain - should use current default implementation
        vm.prank(admin);
        (address chain1,) = factory.createSyndicateSequencingChainWithCustomId(1001, admin, permissionModule);
        // Read implementation from proxy storage (ERC1967 standard slot)
        bytes32 IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;
        address impl1 = address(uint160(uint256(vm.load(chain1, IMPLEMENTATION_SLOT))));
        assertEq(impl1, factory.syndicateChainImpl());

        // Add new implementation and make it default
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();
        vm.prank(admin);
        factory.addAllowedImplementation(address(newImpl), true); // makeDefault = true

        // Deploy second chain - should use new implementation
        vm.prank(admin);
        (address chain2,) = factory.createSyndicateSequencingChainWithCustomId(1002, admin, permissionModule);
        address impl2 = address(uint160(uint256(vm.load(chain2, IMPLEMENTATION_SLOT))));
        assertEq(impl2, address(newImpl));

        // Both chains should have same predictable addresses (same bytecode template)
        address expectedAddr1 = factory.computeSequencingChainAddress(1001);
        address expectedAddr2 = factory.computeSequencingChainAddress(1002);
        assertEq(chain1, expectedAddr1);
        assertEq(chain2, expectedAddr2);
    }

    function testEndToEndUpgradeFlow() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Deploy a chain using the factory
        vm.prank(admin);
        (address chainAddr, uint256 chainId) =
            factory.createSyndicateSequencingChainWithCustomId(2001, admin, permissionModule);

        // Verify chain was deployed with current default implementation
        bytes32 IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;
        address initialImpl = address(uint160(uint256(vm.load(chainAddr, IMPLEMENTATION_SLOT))));
        assertEq(initialImpl, factory.syndicateChainImpl());

        // Create new implementations
        SyndicateSequencingChain goodImpl = new SyndicateSequencingChain();
        SyndicateSequencingChain badImpl = new SyndicateSequencingChain();

        // Add the good implementation to allowed list
        vm.prank(admin);
        factory.addAllowedImplementation(address(goodImpl), false);

        // Note: badImpl is NOT added to allowed list, making it "bad"

        SyndicateSequencingChain chain = SyndicateSequencingChain(chainAddr);

        // Test 1: Upgrade to good (allowed) implementation should succeed
        // First set allowGasTrackingBanOnUpgrade to false
        vm.prank(admin);
        chain.setAllowGasTrackingBanOnUpgrade(false);

        // Now perform the upgrade (authorization happens internally)
        vm.prank(admin);
        chain.upgradeToAndCall(address(goodImpl), "");

        // Verify upgrade succeeded
        address currentImpl = address(uint160(uint256(vm.load(chainAddr, IMPLEMENTATION_SLOT))));
        assertEq(currentImpl, address(goodImpl));

        // Chain should not be banned from gas tracking
        assertFalse(factory.isChainBannedFromGasTracking(chainId));

        // Test 2: Try to upgrade to bad (not allowed) implementation with allowGasTrackingBan = false
        // allowGasTrackingBanOnUpgrade is still false from previous test
        vm.prank(admin);
        vm.expectRevert("Upgrade would result in gas tracking ban");
        chain.upgradeToAndCall(address(badImpl), ""); // Should fail because allowGasTrackingBanOnUpgrade = false

        // Test 3: Upgrade to bad implementation with allowGasTrackingBan = true should succeed but ban the chain
        // First set allowGasTrackingBanOnUpgrade to true
        vm.prank(admin);
        chain.setAllowGasTrackingBanOnUpgrade(true);

        vm.expectEmit(true, true, false, false);
        emit SyndicateFactory.ChainBannedFromGasTracking(chainId, address(badImpl));

        // Perform the actual upgrade (should succeed now)
        vm.prank(admin);
        chain.upgradeToAndCall(address(badImpl), "");

        // Verify upgrade succeeded
        currentImpl = address(uint160(uint256(vm.load(chainAddr, IMPLEMENTATION_SLOT))));
        assertEq(currentImpl, address(badImpl));

        // Chain should now be banned from gas tracking
        assertTrue(factory.isChainBannedFromGasTracking(chainId));

        // Test 4: Check that gas tracking methods exclude banned chains
        uint256 totalBefore = factory.getTotalAppchainsForGasTracking();

        // Create another chain that won't be banned
        vm.prank(admin);
        (address goodChainAddr, uint256 goodChainId) =
            factory.createSyndicateSequencingChainWithCustomId(2002, admin, permissionModule);

        uint256 totalAfter = factory.getTotalAppchainsForGasTracking();

        // Only the good chain should be counted (bad chain is banned)
        assertEq(totalAfter, totalBefore + 1);

        // Test getContractsForGasTracking with banned chain
        uint256[] memory testChainIds = new uint256[](2);
        testChainIds[0] = chainId; // banned chain
        testChainIds[1] = goodChainId; // good chain

        address[] memory contracts = factory.getContractsForGasTracking(testChainIds);
        assertEq(contracts.length, 2);
        assertEq(contracts[0], address(0)); // banned chain returns zero address
        assertEq(contracts[1], goodChainAddr); // good chain returns actual address
    }

    function testRemoveAllowedImplementation() public {
        SyndicateSequencingChain impl1 = new SyndicateSequencingChain();
        SyndicateSequencingChain impl2 = new SyndicateSequencingChain();

        // Add implementations
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl1), false);
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl2), false);

        // Verify they are allowed
        assertTrue(factory.isImplementationAllowed(address(impl1)));
        assertTrue(factory.isImplementationAllowed(address(impl2)));

        address[] memory allowedBefore = factory.getAllowedImplementations();
        uint256 lengthBefore = allowedBefore.length;

        // Remove impl1
        vm.prank(admin);
        factory.removeAllowedImplementation(address(impl1));

        // Verify impl1 is no longer allowed
        assertFalse(factory.isImplementationAllowed(address(impl1)));
        // Verify impl2 is still allowed
        assertTrue(factory.isImplementationAllowed(address(impl2)));

        // Verify array length decreased
        address[] memory allowedAfter = factory.getAllowedImplementations();
        assertEq(allowedAfter.length, lengthBefore - 1);

        // Verify impl1 is not in the array
        bool found = false;
        for (uint256 i = 0; i < allowedAfter.length; i++) {
            if (allowedAfter[i] == address(impl1)) {
                found = true;
                break;
            }
        }
        assertFalse(found);
    }

    function testRemoveAllowedImplementationNonAdminReverts() public {
        SyndicateSequencingChain impl = new SyndicateSequencingChain();

        // Add implementation first
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl), false);

        // Non-admin cannot remove
        vm.prank(manager);
        vm.expectRevert(); // AccessControl revert
        factory.removeAllowedImplementation(address(impl));

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl revert
        factory.removeAllowedImplementation(address(impl));
    }

    function testRemoveNonAllowedImplementationReverts() public {
        SyndicateSequencingChain impl = new SyndicateSequencingChain();

        // Try to remove implementation that was never added
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ImplementationNotAllowed.selector);
        factory.removeAllowedImplementation(address(impl));
    }

    function testRemoveDefaultImplementationReverts() public {
        // Get the current default implementation (stubImplementation from constructor)
        address defaultImpl = factory.syndicateChainImpl();
        assertTrue(factory.isImplementationAllowed(defaultImpl));

        // Try to remove the default implementation
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.CannotRemoveDefaultImplementation.selector);
        factory.removeAllowedImplementation(defaultImpl);

        // Verify it's still allowed and still the default
        assertTrue(factory.isImplementationAllowed(defaultImpl));
        assertEq(factory.syndicateChainImpl(), defaultImpl);
    }

    function testRemoveImplementationFromMiddleOfArray() public {
        SyndicateSequencingChain impl1 = new SyndicateSequencingChain();
        SyndicateSequencingChain impl2 = new SyndicateSequencingChain();
        SyndicateSequencingChain impl3 = new SyndicateSequencingChain();

        // Add three implementations
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl1), false);
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl2), false);
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl3), false);

        address[] memory beforeRemoval = factory.getAllowedImplementations();
        assertEq(beforeRemoval.length, 4); // 1 from constructor + 3 added

        // Remove impl2 (middle element)
        vm.prank(admin);
        factory.removeAllowedImplementation(address(impl2));

        // Verify array integrity
        address[] memory afterRemoval = factory.getAllowedImplementations();
        assertEq(afterRemoval.length, 3);

        // Verify impl2 is not in array
        bool foundImpl2 = false;
        for (uint256 i = 0; i < afterRemoval.length; i++) {
            if (afterRemoval[i] == address(impl2)) {
                foundImpl2 = true;
                break;
            }
        }
        assertFalse(foundImpl2);

        // Verify other implementations are still there
        assertTrue(factory.isImplementationAllowed(address(impl1)));
        assertFalse(factory.isImplementationAllowed(address(impl2)));
        assertTrue(factory.isImplementationAllowed(address(impl3)));
    }

    function testSetDefaultImplementation() public {
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        // Add implementation first
        vm.prank(admin);
        factory.addAllowedImplementation(address(newImpl), false);

        // Set as default
        vm.prank(admin);
        factory.setDefaultImplementation(address(newImpl));

        // Verify it's the current implementation
        assertEq(factory.syndicateChainImpl(), address(newImpl));
    }

    function testSetDefaultImplementationNotAllowedReverts() public {
        SyndicateSequencingChain impl = new SyndicateSequencingChain();

        // Try to set as default without adding to allowed list first
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ImplementationNotAllowed.selector);
        factory.setDefaultImplementation(address(impl));
    }

    function testSetDefaultImplementationNonAdminReverts() public {
        SyndicateSequencingChain impl = new SyndicateSequencingChain();

        // Add implementation first
        vm.prank(admin);
        factory.addAllowedImplementation(address(impl), false);

        // Non-admin cannot set default
        vm.prank(manager);
        vm.expectRevert(); // AccessControl revert
        factory.setDefaultImplementation(address(impl));

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl revert
        factory.setDefaultImplementation(address(impl));
    }

    // ================== DETERMINISTIC CHAIN ID TESTS ==================

    function testGenerateDeterministicChainId() public view {
        address sender = address(0x123);
        uint256 nonce = 0;

        uint256 chainId = factory.generateDeterministicChainId(sender, nonce);

        // Verify chainID is deterministic (same inputs -> same output)
        uint256 chainId2 = factory.generateDeterministicChainId(sender, nonce);
        assertEq(chainId, chainId2);

        // Verify chainID is never 0
        assertTrue(chainId > 0);

        // Verify different sender produces different chainID
        uint256 chainId3 = factory.generateDeterministicChainId(address(0x456), nonce);
        assertTrue(chainId != chainId3);

        // Verify different nonce produces different chainID
        uint256 chainId4 = factory.generateDeterministicChainId(sender, 1);
        assertTrue(chainId != chainId4);
    }

    function testCreateSequencingChainDeterministicWithAutoIncrement() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Check initial nonce is 0
        assertEq(factory.getNextNonceForSender(admin), 0);

        // Create sequencing chain with auto-increment nonce (nonce = 0)
        vm.prank(admin);

        (address sequencingChain, uint256 chainId) =
            factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        // Verify the chain was deployed
        assertTrue(sequencingChain != address(0));
        assertTrue(chainId > 0);

        // Verify nonce incremented
        assertEq(factory.getNextNonceForSender(admin), 1);

        // Verify deterministic generation
        uint256 expectedChainId = factory.generateDeterministicChainId(admin, 0);
        assertEq(chainId, expectedChainId);

        // Verify chain is marked as used
        assertTrue(factory.isChainIdUsed(chainId));
    }

    function testCreateSequencingChainDeterministicWithSpecificNonce() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Create sequencing chain deterministically
        vm.prank(admin);

        (address sequencingChain, uint256 chainId) =
            factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        // Verify the chain was deployed
        assertTrue(sequencingChain != address(0));
        assertTrue(chainId > 0);

        // Verify deterministic generation works
        assertTrue(factory.isChainIdUsed(chainId));
    }

    function testCreateSequencingChainDeterministicIncreasesNonce() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Create first chain deterministically (uses nonce 0)
        vm.prank(admin);
        (, uint256 chainId1) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        // Create second chain deterministically (uses nonce 1) - should succeed with different chain ID
        vm.prank(admin);
        (, uint256 chainId2) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        // Chain IDs should be different
        assertTrue(chainId1 != chainId2);

        // Both should be marked as used
        assertTrue(factory.isChainIdUsed(chainId1));
        assertTrue(factory.isChainIdUsed(chainId2));

        // Nonce should be incremented to 2
        assertEq(factory.getNextNonceForSender(admin), 2);
    }

    function testCreateSequencingChainDeterministicDifferentSendersGetDifferentChainIds() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        address sender1 = address(0x111);
        address sender2 = address(0x222);

        // Both senders use nonce 0
        vm.prank(sender1);
        (, uint256 chainId1) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        vm.prank(sender2);
        (, uint256 chainId2) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        // Chain IDs should be different
        assertTrue(chainId1 != chainId2);

        // Both nonces should be 1 now
        assertEq(factory.getNextNonceForSender(sender1), 1);
        assertEq(factory.getNextNonceForSender(sender2), 1);
    }

    function testCreateSequencingChainDeterministicWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create deterministic sequencing chain
        vm.prank(admin);
        vm.expectRevert(); // Pausable will revert
        factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);
    }

    function testCreateSequencingChainDeterministicRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainDeterministic(address(0), permissionModule);
    }

    function testCreateSequencingChainDeterministicRevertsOnZeroPermissionModule() public {
        address chainAdmin = address(0x789);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainDeterministic(chainAdmin, IRequirementModule(address(0)));
    }

    function testCreateSyndicateSequencingChainWithCustomIdAdminOnly() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        uint256 customChainId = 999999;

        // Admin can create with custom ID
        vm.prank(admin);
        (address sequencingChain, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);

        assertTrue(sequencingChain != address(0));
        assertEq(actualChainId, customChainId);
        assertTrue(factory.isChainIdUsed(customChainId));

        // Non-admin cannot create with custom ID
        vm.prank(manager);
        vm.expectRevert(); // AccessControl revert
        factory.createSyndicateSequencingChainWithCustomId(customChainId + 1, chainAdmin, permissionModule);

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl revert
        factory.createSyndicateSequencingChainWithCustomId(customChainId + 2, chainAdmin, permissionModule);
    }

    function testCreateCustomIdCannotReuseChainId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        uint256 customChainId = 888888;

        // Create first chain
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);

        // Try to reuse same chain ID - should fail
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);
    }

    function testCreateCustomIdCannotUseZeroChainId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(0, chainAdmin, permissionModule);
    }

    function testCreateCustomIdWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        uint256 customChainId = 777777;

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create custom ID chain
        vm.prank(admin);
        vm.expectRevert(); // Pausable will revert
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, permissionModule);
    }

    function testCreateCustomIdRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        uint256 customChainId = 666666;

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, address(0), permissionModule);
    }

    function testCreateCustomIdRevertsOnZeroPermissionModule() public {
        address chainAdmin = address(0x789);
        uint256 customChainId = 555555;

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(customChainId, chainAdmin, IRequirementModule(address(0)));
    }

    function testAntiSquattingAcrossChains() public {
        // This test demonstrates that the same sender will generate deterministic chainIDs
        // preventing squatting across different deployments

        address sender = address(0x555);

        // Deploy a chain - this will use nonce 0 for the sender
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Generate expected chainID for sender with nonce 0 (first deployment)
        uint256 expectedChainId = factory.generateDeterministicChainId(sender, 0);

        vm.prank(sender);
        (, uint256 actualChainId) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        // ChainID should match expected
        assertEq(actualChainId, expectedChainId);

        // Now simulate trying to deploy again with the same sender
        // This should generate a different chain ID (nonce 1) but should not revert
        uint256 expectedChainId2 = factory.generateDeterministicChainId(sender, 1);
        vm.prank(sender);
        (, uint256 actualChainId2) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        assertEq(actualChainId2, expectedChainId2);
        assertTrue(actualChainId != actualChainId2);
    }

    function testFuzzDeterministicChainIdGeneration(address sender, uint256 nonce) public view {
        // Ensure we have a valid sender (not zero address since our contract checks for this)
        vm.assume(sender != address(0));

        uint256 chainId = factory.generateDeterministicChainId(sender, nonce);

        // Chain ID should never be 0
        assertTrue(chainId > 0);

        // Chain ID should be deterministic
        uint256 chainId2 = factory.generateDeterministicChainId(sender, nonce);
        assertEq(chainId, chainId2);

        // Chain ID should be within reasonable bounds (less than 10^18)
        assertTrue(chainId < 10 ** 18);
    }

    function testMultipleSequentialDeterministicDeployments() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);
        address sender = address(0x666);

        uint256[] memory chainIds = new uint256[](5);

        // Deploy 5 sequential chains using auto-increment nonces
        for (uint256 i = 0; i < 5; i++) {
            vm.prank(sender);
            (, chainIds[i]) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

            // Verify nonce incremented
            assertEq(factory.getNextNonceForSender(sender), i + 1);
        }

        // All chain IDs should be different
        for (uint256 i = 0; i < 5; i++) {
            for (uint256 j = i + 1; j < 5; j++) {
                assertTrue(chainIds[i] != chainIds[j]);
            }
        }

        // All should be marked as used
        for (uint256 i = 0; i < 5; i++) {
            assertTrue(factory.isChainIdUsed(chainIds[i]));
        }
    }

    // ================== STUB IMPLEMENTATION TESTS ==================

    function testStubImplementationConsistency() public view {
        // Test that stub implementation address is deterministic
        address computedStub = factory.computeStubImplementationAddress();
        address actualStub = factory.stubImplementation();
        assertEq(computedStub, actualStub);
    }

    // ================== COMPREHENSIVE GAS TRACKING TESTS ==================

    function testGasTrackingWithBannedChains() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Create 3 chains
        uint256 chainId1 = 1001;
        uint256 chainId2 = 1002;
        uint256 chainId3 = 1003;

        vm.prank(admin);
        (address chain1,) = factory.createSyndicateSequencingChainWithCustomId(chainId1, admin, permissionModule);
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(chainId2, admin, permissionModule);
        vm.prank(admin);
        (address chain3,) = factory.createSyndicateSequencingChainWithCustomId(chainId3, admin, permissionModule);

        // Initially all chains should be included
        assertEq(factory.getTotalAppchainsForGasTracking(), 3);
        assertEq(factory.numberOfChainsBannedFromGasTracking(), 0);

        // Ban chain 2
        vm.prank(admin);
        factory.banChainFromGasTracking(chainId2, address(0x123));

        // Now only 2 chains should be counted
        assertEq(factory.getTotalAppchainsForGasTracking(), 2);
        assertEq(factory.numberOfChainsBannedFromGasTracking(), 1);

        // Test getAppchainsAndContractsForGasTracking
        (uint256[] memory chainIds, address[] memory contracts) = factory.getAppchainsAndContractsForGasTracking();
        assertEq(chainIds.length, 2);
        assertEq(contracts.length, 2);

        // Should contain chains 1 and 3, but not chain 2
        bool foundChain1 = false;
        bool foundChain3 = false;
        bool foundChain2 = false;

        for (uint256 i = 0; i < chainIds.length; i++) {
            if (chainIds[i] == chainId1) foundChain1 = true;
            if (chainIds[i] == chainId2) foundChain2 = true;
            if (chainIds[i] == chainId3) foundChain3 = true;
        }

        assertTrue(foundChain1);
        assertTrue(foundChain3);
        assertFalse(foundChain2);

        // Test getContractsForGasTracking with banned chain
        uint256[] memory requestedChains = new uint256[](3);
        requestedChains[0] = chainId1;
        requestedChains[1] = chainId2; // banned
        requestedChains[2] = chainId3;

        address[] memory returnedContracts = factory.getContractsForGasTracking(requestedChains);
        assertEq(returnedContracts.length, 3);
        assertEq(returnedContracts[0], chain1); // chain1 should be returned
        assertEq(returnedContracts[1], address(0)); // chain2 should return zero address (banned)
        assertEq(returnedContracts[2], chain3); // chain3 should be returned
    }

    function testBanningAlreadyBannedChainDoesntDoubleCount() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        uint256 chainId = 2001;
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(chainId, admin, permissionModule);

        // Ban the chain
        vm.prank(admin);
        factory.banChainFromGasTracking(chainId, address(0x123));
        assertEq(factory.numberOfChainsBannedFromGasTracking(), 1);

        // Ban it again - count should not increase
        vm.prank(admin);
        factory.banChainFromGasTracking(chainId, address(0x456));
        assertEq(factory.numberOfChainsBannedFromGasTracking(), 1); // Still 1, not 2
    }

    // ================== COMPREHENSIVE INTEGRATION TESTS ==================

    function testFullFactoryLifecycle() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Test 1: Regular chain creation
        vm.prank(admin);
        (address regularChain, uint256 regularChainId) =
            factory.createSyndicateSequencingChainWithCustomId(5001, admin, permissionModule);
        assertTrue(regularChain != address(0));
        assertEq(regularChainId, 5001);

        // Test 2: Another custom chain creation
        vm.prank(admin);
        (, uint256 customChainId1) = factory.createSyndicateSequencingChainWithCustomId(6001, admin, permissionModule);
        assertEq(customChainId1, 6001);

        // Test 3: Deterministic chain creation
        (, uint256 detChainId) = factory.createSyndicateSequencingChainDeterministic(admin, permissionModule);
        assertTrue(detChainId > 0);

        // Test 4: Another custom chain creation
        vm.prank(admin);
        (, uint256 customChainId2) = factory.createSyndicateSequencingChainWithCustomId(9999, admin, permissionModule);
        assertEq(customChainId2, 9999);

        // Test 5: All chains should be tracked
        assertEq(factory.getTotalAppchainsForGasTracking(), 4);

        // Test 6: Verify all chain IDs are different
        assertTrue(regularChainId != customChainId1);
        assertTrue(regularChainId != detChainId);
        assertTrue(regularChainId != customChainId2);
        assertTrue(customChainId1 != detChainId);
        assertTrue(customChainId1 != customChainId2);
        assertTrue(detChainId != customChainId2);
    }

    function testMixedCreationMethodsWithCollisions() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Create regular chain
        uint256 regularId = 1000;
        vm.prank(admin);
        factory.createSyndicateSequencingChainWithCustomId(regularId, admin, permissionModule);

        // Try to create deterministic chain that might collide
        vm.prank(admin);
        (, uint256 detId) = factory.createSyndicateSequencingChainDeterministic(admin, permissionModule);

        // Try to create custom chain with same ID as regular - should fail
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChainWithCustomId(regularId, admin, permissionModule);

        // Try to create regular chain with same ID as deterministic - should fail
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChainWithCustomId(detId, admin, permissionModule);

        // All methods should respect the shared chainId space
        assertTrue(factory.isChainIdUsed(regularId));
        assertTrue(factory.isChainIdUsed(detId));
    }

    // ================== ERROR HANDLING TESTS ==================

    function testZeroAddressValidation() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Test all creation methods with zero admin
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1001, address(0), permissionModule);

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainDeterministic(address(0), permissionModule);

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1002, address(0), permissionModule);

        // Test all creation methods with zero permission module
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1003, admin, IRequirementModule(address(0)));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainDeterministic(admin, IRequirementModule(address(0)));

        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChainWithCustomId(1004, admin, IRequirementModule(address(0)));
    }

    function testImplementationManagementErrors() public {
        SyndicateSequencingChain impl = new SyndicateSequencingChain();

        // Test adding zero address implementation
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.addAllowedImplementation(address(0), false);

        // Test removing implementation not in list
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ImplementationNotAllowed.selector);
        factory.removeAllowedImplementation(address(impl));

        // Test setting non-allowed implementation as default
        vm.prank(admin);
        vm.expectRevert(SyndicateFactory.ImplementationNotAllowed.selector);
        factory.setDefaultImplementation(address(impl));
    }

    // ================== EDGE CASE TESTS ==================

    function testLargeNonceValues() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address chainAdmin = address(0x789);

        // Should handle normal nonce values (avoiding overflow issues)
        vm.prank(admin);
        (, uint256 chainId) = factory.createSyndicateSequencingChainDeterministic(chainAdmin, permissionModule);

        assertTrue(chainId > 0);
        assertEq(factory.getNextNonceForSender(admin), 1);

        // Verify deterministic generation still works
        uint256 expectedId = factory.generateDeterministicChainId(admin, 0);
        assertEq(chainId, expectedId);
    }

    function testMaxChainIdValue() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        uint256 maxChainId = type(uint256).max - 1;

        // Should handle very large chain ID values
        vm.prank(admin);
        (, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(maxChainId, admin, permissionModule);

        assertEq(actualChainId, maxChainId);
        assertTrue(factory.isChainIdUsed(maxChainId));
    }

    // ================== FUZZ TESTS ==================

    function testFuzzCreateRegularChain(uint256 chainId, address chainAdmin) public {
        vm.assume(chainId != 0);
        vm.assume(chainAdmin != address(0));
        vm.assume(!factory.isChainIdUsed(chainId));

        RequireAndModule permissionModule = new RequireAndModule(admin);

        vm.prank(admin);
        (, uint256 actualChainId) =
            factory.createSyndicateSequencingChainWithCustomId(chainId, chainAdmin, permissionModule);
        assertEq(actualChainId, chainId);
        assertTrue(factory.isChainIdUsed(chainId));
    }

    function testFuzzDeterministicChainGeneration(address sender, uint256 nonce) public view {
        vm.assume(sender != address(0));

        uint256 chainId1 = factory.generateDeterministicChainId(sender, nonce);
        uint256 chainId2 = factory.generateDeterministicChainId(sender, nonce);

        // Should be deterministic
        assertEq(chainId1, chainId2);
        assertTrue(chainId1 > 0);
        assertTrue(chainId1 < 10 ** 18); // Within reasonable bounds
    }
}
