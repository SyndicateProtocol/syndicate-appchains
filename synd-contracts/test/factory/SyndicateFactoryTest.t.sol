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

    event NamespaceConfigUpdated(uint256 oldNamespacePrefix, uint256 newNamespacePrefix);

    event ChainIdManuallyMarked(uint256 indexed chainId);

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

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, permissionModule);

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

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(permissionModule));

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

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(permissionModule));

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

        (address sequencingChain1, uint256 actualChainId1) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule)));
        (address sequencingChain2, uint256 actualChainId2) = factory.createSyndicateSequencingChain(
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
        factory.createSyndicateSequencingChain(appchainId, address(0), IRequirementModule(address(permissionModule)));
    }

    function testRevertsOnZeroPermissionModule() public {
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(0)));
    }

    function testAutoIncrementOnZeroChainId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedChainId = (namespacePrefix * 10) + 1;

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)));

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(SyndicateSequencingChain(sequencingChainAddress).appchainId(), expectedChainId);

        // Verify next chain ID incremented
        uint256 nextExpectedChainId = (namespacePrefix * 10) + 2;
        assertEq(factory.getNextChainId(), nextExpectedChainId);
    }

    function testCreateSequencingChainAddressIsDeterministic() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        address expectedAddress = factory.computeSequencingChainAddress(appchainId);
        (address sequencingChainAddress,) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule)));

        assertEq(sequencingChainAddress, expectedAddress);
    }

    function testGetProxyBytecode() public {
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

    function testAutoIncrementChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // First auto-incremented chain ID
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)));

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Second auto-incremented chain ID
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)));
        uint256 expectedId2 = (namespacePrefix * 10) + 2;
        assertEq(id2, expectedId2);

        // Third auto-incremented chain ID
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)));
        uint256 expectedId3 = (namespacePrefix * 10) + 3;
        assertEq(id3, expectedId3);
    }

    function testMixedAutoAndManualChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // Auto chain ID
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)));

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Manual chain ID
        uint256 manualId = 42000;
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(manualId, admin, IRequirementModule(address(permissionModule2)));
        assertEq(id2, manualId);

        // Back to auto chain ID
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)));
        // Should be next auto ID since we only used one auto ID so far
        uint256 expectedId3 = (namespacePrefix * 10) + 2;
        assertEq(id3, expectedId3);
    }

    function testChainIdAlreadyExists() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create first chain
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule1)));

        // Try to create another with same chain ID
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule2)));
    }

    function testIsChainIdUsed() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appchainId), false);

        // Create chain
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule1)));

        // Now chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appchainId), true);

        // Auto-incremented ID should also be marked as used
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)));
        assertEq(factory.isChainIdUsed(id2), true);
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
        vm.expectRevert(); // AccessControl will revert
        factory.unpause();
    }

    function testCreateSequencingChainWhenPausedReverts() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create sequencing chain
        vm.expectRevert(); // Pausable will revert with "Pausable: paused"
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule)));
    }

    function testCreateSequencingChainAfterUnpauseWorks() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Pause then unpause
        vm.prank(admin);
        factory.pause();
        vm.prank(admin);
        factory.unpause();

        // Should work after unpause
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule)));

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

    function testNamespaceConfigGetters() public view {
        // Initial values
        assertEq(factory.namespacePrefix(), 510);
        // Note: namespaceMultiplier no longer exists
    }

    function testPublicVariables() public view {
        // Test that variables are publicly accessible
        assertEq(factory.namespacePrefix(), 510);
        assertEq(factory.nextAutoChainId(), 1);
        assertEq(factory.appchainContracts(appchainId), address(0));
    }

    function testUpdateNamespaceConfig() public {
        uint256 newPrefix = 511;

        // Manager can update namespace config
        vm.prank(manager);
        vm.expectEmit(true, true, false, true);
        emit NamespaceConfigUpdated(
            510, // old prefix
            newPrefix
        );
        factory.updateNamespaceConfig(newPrefix);

        // Check new values
        assertEq(factory.namespacePrefix(), newPrefix);
    }

    function testUpdateNamespaceConfigAsNonManagerReverts() public {
        uint256 newPrefix = 511;

        // Non-manager cannot update namespace config
        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert with a specific message
        factory.updateNamespaceConfig(newPrefix);
    }

    function testNamespaceUpdateAffectsAutoIncrementChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create a chain with old namespace
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)));

        // Verify chain ID follows current formula
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Update namespace config
        uint256 newPrefix = 600;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix);

        // Create a chain with new namespace
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)));

        // New namespace: (600 * 10) + 2 (counter is at 2 now)
        uint256 expectedId2 = (newPrefix * 10) + 2;
        assertEq(id2, expectedId2);
    }

    function testInitializeWithZeroAddressReverts() public {
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (address(0)));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        new ERC1967Proxy(address(implementation), initData);
    }

    function testGetNextChainIdFunction() public view {
        // Test that the public function works correctly
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 nextAutoChainId = factory.nextAutoChainId();

        uint256 expectedNextId = (namespacePrefix * 10) + nextAutoChainId;
        uint256 actualNextId = factory.getNextChainId();
        assertEq(actualNextId, expectedNextId);
    }

    function testChainIdArithmeticLogic() public {
        // Test arithmetic-based chain ID generation (replacement for old concatenation logic)
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // First: dynamic calculation
        uint256 namespacePrefix = factory.namespacePrefix();

        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)));
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Second: dynamic calculation
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)));
        uint256 expectedId2 = (namespacePrefix * 10) + 2;
        assertEq(id2, expectedId2);

        // Create several more to test sequential generation
        for (uint256 i = 3; i <= 5; i++) {
            RequireAndModule pm = new RequireAndModule(admin);
            (, uint256 id) = factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(pm)));

            // Expected: (namespacePrefix * 10) + i
            uint256 expectedId = (namespacePrefix * 10) + i;
            assertEq(id, expectedId);
        }
    }

    function testLargeChainIdNumbers() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Update to higher starting point to test larger arithmetic calculations
        vm.prank(admin);
        factory.updateNamespaceConfig(999);

        // Create auto chain with dynamic calculation
        uint256 newNamespacePrefix = factory.namespacePrefix(); // 999 after update
        uint256 expectedId = (newNamespacePrefix * 10) + 1;

        (, uint256 id) = factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)));
        assertEq(id, expectedId);
    }

    function testSequencingChainImplementationAddress() public view {
        // Test that implementation address is set and not zero
        uint256 chainId = 510_000_000_001; // Use an auto-generated ID
        address impl = factory.computeSequencingChainAddress(chainId);
        assertTrue(impl != address(0));
    }

    // Edge cases and fuzz tests
    function testFuzzManualChainIds(uint256 chainId) public {
        // Skip already used chain IDs and auto-generated ones
        vm.assume(chainId != 0);
        // Skip first auto-generated ID based on current formula
        uint256 firstAutoId = (factory.namespacePrefix() * 10) + 1;
        vm.assume(chainId != firstAutoId);
        vm.assume(factory.isChainIdUsed(chainId) == false);

        RequireAndModule permissionModule = new RequireAndModule(admin);

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(chainId, admin, IRequirementModule(address(permissionModule)));

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, chainId);
        assertEq(factory.isChainIdUsed(chainId), true);
    }

    function testNewChainIdGenerationFormat() public view {
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 nextAutoChainId = factory.nextAutoChainId();

        uint256 expectedChainId = (namespacePrefix * 10) + nextAutoChainId;
        uint256 actualChainId = factory.getNextChainId();
        assertEq(actualChainId, expectedChainId);
    }

    function testChainIdNoCollisions() public {
        // Test that different namespace/autoId combinations produce different results

        vm.startPrank(manager);

        // Test scenario 1: namespace=12, simple autoId
        factory.updateNamespaceConfig(12);
        uint256 chainId1 = factory.getNextChainId();
        // Expected: (12 * 10) + 1

        // Test scenario 2: namespace=123
        vm.expectRevert();
        factory.updateNamespaceConfig(123);

        // Test scenario 3: namespace=1
        vm.expectRevert();
        factory.updateNamespaceConfig(1);

        // Test scenario 3: namespace=133, simple autoId (same counter value)
        factory.updateNamespaceConfig(133);

        uint256 chainId2 = factory.getNextChainId();
        // Expected: (133 * 10) + 1
        // Note: nextAutoChainId is still 1 since we didn't create any chains yet

        vm.stopPrank();

        // These should be completely different, demonstrating no collision
        assertTrue(chainId1 != chainId2);
        assertEq(chainId1, (12 * 10) + 1);
        assertEq(chainId2, (133 * 10) + 1);
    }

    function testGetContractsForAppchains() public {
        // Create a few chains with non-sequential chain IDs
        RequireAndModule permissionModule = new RequireAndModule(admin);

        uint256 appchainId1 = 100;
        uint256 appchainId2 = 200;
        uint256 appchainId3 = 300;

        vm.prank(nonManager);
        (address chain1,) = factory.createSyndicateSequencingChain(appchainId1, admin, permissionModule);

        vm.prank(nonManager);
        (address chain2,) = factory.createSyndicateSequencingChain(appchainId2, admin, permissionModule);

        vm.prank(nonManager);
        (address chain3,) = factory.createSyndicateSequencingChain(appchainId3, admin, permissionModule);

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

        (address chain1Addr,) = factory.createSyndicateSequencingChain(appchainId1, admin, permissionModule);
        (address chain2Addr,) = factory.createSyndicateSequencingChain(appchainId2, admin, permissionModule);
        (address chain3Addr,) = factory.createSyndicateSequencingChain(appchainId3, admin, permissionModule);

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
        (address chainAddr, uint256 chainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, permissionModule);

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
        (address chainAddr, uint256 chainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, permissionModule);

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
        (, uint256 chainId) = factory.createSyndicateSequencingChain(appchainId, admin, permissionModule);

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
        (address chain1,) = factory.createSyndicateSequencingChain(1001, admin, permissionModule);
        // Read implementation from proxy storage (ERC1967 standard slot)
        bytes32 IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;
        address impl1 = address(uint160(uint256(vm.load(chain1, IMPLEMENTATION_SLOT))));
        assertEq(impl1, factory.syndicateChainImpl());

        // Add new implementation and make it default
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();
        vm.prank(admin);
        factory.addAllowedImplementation(address(newImpl), true); // makeDefault = true

        // Deploy second chain - should use new implementation
        (address chain2,) = factory.createSyndicateSequencingChain(1002, admin, permissionModule);
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
        (address chainAddr, uint256 chainId) = factory.createSyndicateSequencingChain(2001, admin, permissionModule);

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
        (address goodChainAddr, uint256 goodChainId) =
            factory.createSyndicateSequencingChain(2002, admin, permissionModule);

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
}
