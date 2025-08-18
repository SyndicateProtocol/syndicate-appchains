// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {RequireCompositeModule} from "src/requirement-modules/RequireCompositeModule.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";

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
        factory = new SyndicateFactory(admin, 0);

        // Grant manager role to the manager address
        vm.prank(admin);
        factory.grantRole(MANAGER_ROLE, manager);
    }

    function testCreateSequencingChainWithRequireAndModule() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        address permissionModuleAddress = address(permissionModule);

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-1"));
        address expectedAddress = factory.computeSequencingChainAddress(salt, appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, permissionModule, salt);

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

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-2"));
        address expectedAddress = factory.computeSequencingChainAddress(salt, appchainId);

        vm.expectEmit(true, true, true, true);
        emit SyndicateSequencingChainCreated(appchainId, expectedAddress, permissionModuleAddress);

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(permissionModule), salt);

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

        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-composite"));
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(permissionModule), salt);

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

        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-test-7"));
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-test-8"));

        (address sequencingChain1, uint256 actualChainId1) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt1
        );
        (address sequencingChain2, uint256 actualChainId2) = factory.createSyndicateSequencingChain(
            differentChainId, admin, IRequirementModule(address(permissionModule2)), salt2
        );

        assertEq(SyndicateSequencingChain(sequencingChain1).appchainId(), appchainId);
        assertEq(SyndicateSequencingChain(sequencingChain2).appchainId(), differentChainId);
        assertEq(actualChainId1, appchainId);
        assertEq(actualChainId2, differentChainId);
    }

    function testRevertsOnZeroAdmin() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-9"));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(
            appchainId, address(0), IRequirementModule(address(permissionModule)), salt
        );
    }

    function testRevertsOnZeroPermissionModule() public {
        bytes32 salt = keccak256(abi.encodePacked("salt-for-test-10"));

        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(0)), salt);
    }

    function testAutoIncrementOnZeroChainId() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedChainId = (namespacePrefix * 10) + 1;

        bytes32 salt = keccak256(abi.encodePacked("salt-for-auto-increment"));
        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, expectedChainId);
        assertEq(SyndicateSequencingChain(sequencingChainAddress).appchainId(), expectedChainId);

        // Verify next chain ID incremented
        uint256 nextExpectedChainId = (namespacePrefix * 10) + 2;
        assertEq(factory.getNextChainId(), nextExpectedChainId);
    }

    function testCreateSequencingChainAddressIsDeterministic() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-deterministic"));

        address expectedAddress = factory.computeSequencingChainAddress(salt, appchainId);
        (address sequencingChainAddress,) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt
        );

        assertEq(sequencingChainAddress, expectedAddress);
    }

    function testGetBytecode() public view {
        bytes memory bytecode = factory.getBytecode(appchainId);
        bytes memory expectedBytecode =
            abi.encodePacked(type(SyndicateSequencingChain).creationCode, abi.encode(appchainId, 0));
        assertEq(bytecode, expectedBytecode);
    }

    function testAutoIncrementChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // First auto-incremented chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-auto-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Second auto-incremented chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-auto-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        uint256 expectedId2 = (namespacePrefix * 10) + 2;
        assertEq(id2, expectedId2);

        // Third auto-incremented chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-auto-3"));
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)), salt3);
        uint256 expectedId3 = (namespacePrefix * 10) + 3;
        assertEq(id3, expectedId3);
    }

    function testMixedAutoAndManualChainIds() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);
        RequireCompositeModule permissionModule3 = new RequireCompositeModule(admin);

        // Auto chain ID
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-mixed-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Fetch dynamic values from contract
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Manual chain ID
        uint256 manualId = 42000;
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-mixed-2"));
        (, uint256 id2) = factory.createSyndicateSequencingChain(
            manualId, admin, IRequirementModule(address(permissionModule2)), salt2
        );
        assertEq(id2, manualId);

        // Back to auto chain ID
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-mixed-3"));
        (, uint256 id3) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule3)), salt3);
        // Should be next auto ID since we only used one auto ID so far
        uint256 expectedId3 = (namespacePrefix * 10) + 2;
        assertEq(id3, expectedId3);
    }

    function testChainIdAlreadyExists() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Create first chain
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-duplicate-1"));
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Try to create another with same chain ID
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-duplicate-2"));
        vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule2)), salt2);
    }

    function testIsChainIdUsed() public {
        RequireAndModule permissionModule1 = new RequireAndModule(admin);
        RequireOrModule permissionModule2 = new RequireOrModule(admin);

        // Initially no chain IDs used
        assertEq(factory.isChainIdUsed(appchainId), false);

        // Create chain
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-used-1"));
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Now chain ID should be marked as used
        assertEq(factory.isChainIdUsed(appchainId), true);

        // Auto-incremented ID should also be marked as used
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-used-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
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
        bytes32 salt = keccak256(abi.encodePacked("salt-for-paused"));

        // Pause the factory
        vm.prank(admin);
        factory.pause();

        // Try to create sequencing chain
        vm.expectRevert(); // Pausable will revert with "Pausable: paused"
        factory.createSyndicateSequencingChain(appchainId, admin, IRequirementModule(address(permissionModule)), salt);
    }

    function testCreateSequencingChainAfterUnpauseWorks() public {
        RequireAndModule permissionModule = new RequireAndModule(admin);
        bytes32 salt = keccak256(abi.encodePacked("salt-for-unpause"));

        // Pause then unpause
        vm.prank(admin);
        factory.pause();
        vm.prank(admin);
        factory.unpause();

        // Should work after unpause
        (address sequencingChainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt
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
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-namespace-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);

        // Verify chain ID follows current formula
        uint256 namespacePrefix = factory.namespacePrefix();
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Update namespace config
        uint256 newPrefix = 600;

        vm.prank(admin);
        factory.updateNamespaceConfig(newPrefix);

        // Create a chain with new namespace
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-namespace-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);

        // New namespace: (600 * 10) + 2 (counter is at 2 now)
        uint256 expectedId2 = (newPrefix * 10) + 2;
        assertEq(id2, expectedId2);
    }

    function testConstructorWithZeroAddressReverts() public {
        vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
        new SyndicateFactory(address(0), 0);
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

        bytes32 salt1 = keccak256(abi.encodePacked("arithmetic-1"));
        (, uint256 id1) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule1)), salt1);
        uint256 expectedId1 = (namespacePrefix * 10) + 1;
        assertEq(id1, expectedId1);

        // Second: dynamic calculation
        bytes32 salt2 = keccak256(abi.encodePacked("arithmetic-2"));
        (, uint256 id2) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule2)), salt2);
        uint256 expectedId2 = (namespacePrefix * 10) + 2;
        assertEq(id2, expectedId2);

        // Create several more to test sequential generation
        for (uint256 i = 3; i <= 5; i++) {
            RequireAndModule pm = new RequireAndModule(admin);
            bytes32 salt = keccak256(abi.encodePacked("arithmetic-loop", i));
            (, uint256 id) = factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(pm)), salt);

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

        bytes32 salt = keccak256(abi.encodePacked("large-number"));
        (, uint256 id) =
            factory.createSyndicateSequencingChain(0, admin, IRequirementModule(address(permissionModule)), salt);
        assertEq(id, expectedId);
    }

    function testSequencingChainImplementationAddress() public view {
        // Test that implementation address is set and not zero
        bytes32 salt = keccak256(abi.encodePacked("impl-test"));
        uint256 chainId = 510_000_000_001; // Use an auto-generated ID
        address impl = factory.computeSequencingChainAddress(salt, chainId);
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
        bytes32 salt = keccak256(abi.encodePacked("fuzz-test", chainId));

        (address sequencingChainAddress, uint256 actualChainId) =
            factory.createSyndicateSequencingChain(chainId, admin, IRequirementModule(address(permissionModule)), salt);

        assertTrue(sequencingChainAddress != address(0));
        assertEq(actualChainId, chainId);
        assertEq(factory.isChainIdUsed(chainId), true);
    }

    function testFuzzSalts(bytes32 salt) public {
        RequireAndModule permissionModule = new RequireAndModule(admin);

        (address sequencingChainAddress,) = factory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(address(permissionModule)), salt
        );

        assertTrue(sequencingChainAddress != address(0));
        assertEq(sequencingChainAddress, factory.computeSequencingChainAddress(salt, appchainId));
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
        bytes32 salt1 = keccak256(abi.encodePacked("salt-for-test-9a"));
        bytes32 salt2 = keccak256(abi.encodePacked("salt-for-test-9b"));
        bytes32 salt3 = keccak256(abi.encodePacked("salt-for-test-9c"));

        uint256 appchainId1 = 100;
        uint256 appchainId2 = 200;
        uint256 appchainId3 = 300;

        vm.prank(nonManager);
        (address chain1,) = factory.createSyndicateSequencingChain(appchainId1, admin, permissionModule, salt1);

        vm.prank(nonManager);
        (address chain2,) = factory.createSyndicateSequencingChain(appchainId2, admin, permissionModule, salt2);

        vm.prank(nonManager);
        (address chain3,) = factory.createSyndicateSequencingChain(appchainId3, admin, permissionModule, salt3);

        // Test getContractsForAppchains with specific chain IDs
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = appchainId2; // 200
        chainIDs[1] = appchainId1; // 100

        address[] memory contracts = factory.getContractsForAppchains(chainIDs);

        // Verify that the correct contracts are returned for each chain ID
        // This is the regression test for the bug where it was using the loop index instead of the chain ID
        assertEq(contracts.length, 2);
        assertEq(contracts[0], chain2); // Contract for chain ID 200
        assertEq(contracts[1], chain1); // Contract for chain ID 100

        // Test with a single chain ID
        uint256[] memory singleChainID = new uint256[](1);
        singleChainID[0] = appchainId3; // 300

        address[] memory singleContract = factory.getContractsForAppchains(singleChainID);

        assertEq(singleContract.length, 1);
        assertEq(singleContract[0], chain3); // Contract for chain ID 300
    }
}
