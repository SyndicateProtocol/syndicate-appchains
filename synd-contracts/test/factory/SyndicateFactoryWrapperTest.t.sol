// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateFactoryWrapper} from "src/factory/SyndicateFactoryWrapper.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {RequireAndModuleFactory, RequireOrModuleFactory} from "src/factory/PermissionModuleFactories.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";

contract SyndicateFactoryWrapperTest is Test {
    SyndicateFactoryWrapper public wrapper;
    SyndicateFactory public syndicateFactory;
    RequireAndModuleFactory public andFactory;
    RequireOrModuleFactory public orFactory;

    address public admin;
    address public manager;
    address public user1;
    address public user2;
    address public nonManager;

    uint256 public appchainId = 12345;

    // Constants for role checking
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    // Events to test
    event CompleteSyndicateDeployed(
        uint256 indexed chainId,
        address indexed sequencingChain,
        address indexed permissionModule,
        SyndicateFactoryWrapper.ModuleType moduleType,
        address admin
    );

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        user1 = address(0x3);
        user2 = address(0x4);
        nonManager = address(0x5);

        // Deploy individual factories
        syndicateFactory = new SyndicateFactory(admin);
        andFactory = new RequireAndModuleFactory(admin);
        orFactory = new RequireOrModuleFactory(admin);

        // Deploy wrapper
        wrapper = new SyndicateFactoryWrapper(admin, address(syndicateFactory), address(andFactory), address(orFactory));

        // Grant manager roles
        vm.startPrank(admin);
        wrapper.grantRole(MANAGER_ROLE, manager);
        vm.stopPrank();
    }

    // Basic deployment tests
    function testDeployCompleteSyndicateWithRequireAndModule() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("module-salt-and"));
        bytes32 chainSalt = keccak256(abi.encodePacked("chain-salt-and"));

        // Compute expected addresses
        address expectedModuleAddr = andFactory.computeModuleAddress(user1, moduleSalt);
        address expectedChainAddr = syndicateFactory.computeSequencingChainAddress(chainSalt, appchainId);

        vm.expectEmit(true, true, true, true);
        emit CompleteSyndicateDeployed(
            appchainId, expectedChainAddr, expectedModuleAddr, SyndicateFactoryWrapper.ModuleType.RequireAnd, user1
        );

        (address sequencingChain, address permissionModule, uint256 actualChainId) = wrapper.deployCompleteSyndicate(
            appchainId, user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt
        );

        // Verify addresses match expectations
        assertEq(sequencingChain, expectedChainAddr);
        assertEq(permissionModule, expectedModuleAddr);
        assertEq(actualChainId, appchainId);

        // Verify the deployed contracts work
        RequireAndModule module = RequireAndModule(permissionModule);
        assertEq(module.owner(), user1);

        SyndicateSequencingChain chain = SyndicateSequencingChain(sequencingChain);
        assertEq(chain.appchainId(), appchainId);
        assertEq(address(chain.permissionRequirementModule()), permissionModule);
    }

    function testDeployCompleteSyndicateWithRequireOrModule() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("module-salt-or"));
        bytes32 chainSalt = keccak256(abi.encodePacked("chain-salt-or"));

        // Compute expected addresses
        address expectedModuleAddr = orFactory.computeModuleAddress(user1, moduleSalt);
        address expectedChainAddr = syndicateFactory.computeSequencingChainAddress(chainSalt, appchainId);

        vm.expectEmit(true, true, true, true);
        emit CompleteSyndicateDeployed(
            appchainId, expectedChainAddr, expectedModuleAddr, SyndicateFactoryWrapper.ModuleType.RequireOr, user1
        );

        (address sequencingChain, address permissionModule, uint256 actualChainId) = wrapper.deployCompleteSyndicate(
            appchainId, user1, SyndicateFactoryWrapper.ModuleType.RequireOr, moduleSalt, chainSalt
        );

        // Verify addresses match expectations
        assertEq(sequencingChain, expectedChainAddr);
        assertEq(permissionModule, expectedModuleAddr);
        assertEq(actualChainId, appchainId);

        // Verify the deployed contracts work
        RequireOrModule module = RequireOrModule(permissionModule);
        assertEq(module.owner(), user1);

        SyndicateSequencingChain chain = SyndicateSequencingChain(sequencingChain);
        assertEq(chain.appchainId(), appchainId);
        assertEq(address(chain.permissionRequirementModule()), permissionModule);
    }

    // Convenience function tests
    function testDeployWithRequireAndModule() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("convenience-and-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("convenience-and-chain"));

        (address sequencingChain, address permissionModule, uint256 actualChainId) =
            wrapper.deployWithRequireAndModule(appchainId, user1, moduleSalt, chainSalt);

        assertTrue(sequencingChain != address(0));
        assertTrue(permissionModule != address(0));
        assertEq(actualChainId, appchainId);

        // Verify it's a RequireAndModule
        RequireAndModule module = RequireAndModule(permissionModule);
        assertEq(module.owner(), user1);
    }

    function testDeployWithRequireOrModule() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("convenience-or-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("convenience-or-chain"));

        (address sequencingChain, address permissionModule, uint256 actualChainId) =
            wrapper.deployWithRequireOrModule(appchainId, user1, moduleSalt, chainSalt);

        assertTrue(sequencingChain != address(0));
        assertTrue(permissionModule != address(0));
        assertEq(actualChainId, appchainId);

        // Verify it's a RequireOrModule
        RequireOrModule module = RequireOrModule(permissionModule);
        assertEq(module.owner(), user1);
    }

    // Auto-increment chain ID tests
    function testDeployWithAutoIncrementChainId() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("auto-increment-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("auto-increment-chain"));

        uint256 expectedChainId = wrapper.getNextAutoChainId();

        (address sequencingChain, address permissionModule, uint256 actualChainId) = wrapper.deployCompleteSyndicate(
            0, // Auto-increment
            user1,
            SyndicateFactoryWrapper.ModuleType.RequireAnd,
            moduleSalt,
            chainSalt
        );

        assertTrue(sequencingChain != address(0));
        assertTrue(permissionModule != address(0));
        assertEq(actualChainId, expectedChainId);

        // Verify chain ID is marked as used
        assertEq(wrapper.isChainIdUsed(actualChainId), 1);
    }

    // Address computation tests
    function testComputeCompleteSyndicateAddresses() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("compute-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("compute-chain"));

        (address expectedModuleAddr, address expectedChainAddr) = wrapper.computeCompleteSyndicateAddresses(
            user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt, appchainId
        );

        // Deploy and verify addresses match
        (address sequencingChain, address permissionModule,) = wrapper.deployCompleteSyndicate(
            appchainId, user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt
        );

        assertEq(permissionModule, expectedModuleAddr);
        assertEq(sequencingChain, expectedChainAddr);
    }

    function testComputeAddressesForDifferentModuleTypes() public view {
        bytes32 moduleSalt = keccak256(abi.encodePacked("diff-types-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("diff-types-chain"));

        (address andModuleAddr, address chainAddr1) = wrapper.computeCompleteSyndicateAddresses(
            user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt, appchainId
        );

        (address orModuleAddr, address chainAddr2) = wrapper.computeCompleteSyndicateAddresses(
            user1, SyndicateFactoryWrapper.ModuleType.RequireOr, moduleSalt, chainSalt, appchainId
        );

        // Module addresses should be different (different contract types)
        assertTrue(andModuleAddr != orModuleAddr);
        // Chain addresses should be the same (same salt and chain ID)
        assertEq(chainAddr1, chainAddr2);
    }

    // Error handling tests
    function testDeployWithZeroAddressReverts() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("zero-addr-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("zero-addr-chain"));

        vm.expectRevert(SyndicateFactoryWrapper.ZeroAddress.selector);
        wrapper.deployCompleteSyndicate(
            appchainId, address(0), SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt
        );
    }

    // Pausability tests
    function testPauseUnpause() public {
        // Initially not paused
        assertFalse(wrapper.paused());

        // Admin can pause
        vm.prank(admin);
        wrapper.pause();
        assertTrue(wrapper.paused());

        // Admin can unpause
        vm.prank(admin);
        wrapper.unpause();
        assertFalse(wrapper.paused());
    }

    function testPauseNonAdminReverts() public {
        vm.prank(manager);
        vm.expectRevert(); // AccessControl will revert
        wrapper.pause();

        vm.prank(nonManager);
        vm.expectRevert(); // AccessControl will revert
        wrapper.pause();
    }

    function testDeployWhenPausedReverts() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("paused-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("paused-chain"));

        // Pause the wrapper
        vm.prank(admin);
        wrapper.pause();

        // Try to deploy
        vm.expectRevert(); // Pausable will revert
        wrapper.deployCompleteSyndicate(
            appchainId, user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt
        );
    }

    function testDeployAfterUnpauseWorks() public {
        bytes32 moduleSalt = keccak256(abi.encodePacked("unpause-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("unpause-chain"));

        // Pause then unpause
        vm.prank(admin);
        wrapper.pause();
        vm.prank(admin);
        wrapper.unpause();

        // Should work after unpause
        (address sequencingChain, address permissionModule, uint256 actualChainId) = wrapper.deployCompleteSyndicate(
            appchainId, user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt
        );

        assertTrue(sequencingChain != address(0));
        assertTrue(permissionModule != address(0));
        assertEq(actualChainId, appchainId);
    }

    // Access control tests
    function testRoleSetup() public view {
        // Admin should have both roles
        assertTrue(wrapper.hasRole(DEFAULT_ADMIN_ROLE, admin));
        assertTrue(wrapper.hasRole(MANAGER_ROLE, admin));

        // Manager should have manager role
        assertTrue(wrapper.hasRole(MANAGER_ROLE, manager));

        // Non-manager should not have any roles
        assertFalse(wrapper.hasRole(DEFAULT_ADMIN_ROLE, nonManager));
        assertFalse(wrapper.hasRole(MANAGER_ROLE, nonManager));
    }

    // Helper function tests
    function testGetNextAutoChainId() public view {
        uint256 nextId = wrapper.getNextAutoChainId();
        assertEq(nextId, 5101); // Should match syndicate factory default
    }

    function testGetNamespacePrefix() public view {
        uint256 prefix = wrapper.getNamespacePrefix();
        assertEq(prefix, 510); // Should match syndicate factory default
    }

    function testIsChainIdUsed() public {
        // Initially not used
        assertEq(wrapper.isChainIdUsed(appchainId), 0);

        // Deploy something
        bytes32 moduleSalt = keccak256(abi.encodePacked("used-module"));
        bytes32 chainSalt = keccak256(abi.encodePacked("used-chain"));

        wrapper.deployCompleteSyndicate(
            appchainId, user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt, chainSalt
        );

        // Now should be used
        assertEq(wrapper.isChainIdUsed(appchainId), 1);
    }

    // Constructor tests
    function testConstructorWithZeroAddressReverts() public {
        vm.expectRevert(SyndicateFactoryWrapper.ZeroAddress.selector);
        new SyndicateFactoryWrapper(
            address(0), // zero admin
            address(syndicateFactory),
            address(andFactory),
            address(orFactory)
        );

        vm.expectRevert(SyndicateFactoryWrapper.ZeroAddress.selector);
        new SyndicateFactoryWrapper(
            admin,
            address(0), // zero syndicate factory
            address(andFactory),
            address(orFactory)
        );

        vm.expectRevert(SyndicateFactoryWrapper.ZeroAddress.selector);
        new SyndicateFactoryWrapper(
            admin,
            address(syndicateFactory),
            address(0), // zero and factory
            address(orFactory)
        );

        vm.expectRevert(SyndicateFactoryWrapper.ZeroAddress.selector);
        new SyndicateFactoryWrapper(
            admin,
            address(syndicateFactory),
            address(andFactory),
            address(0) // zero or factory
        );
    }

    // Integration tests
    function testMultipleDeploymentsDifferentTypes() public {
        bytes32 moduleSalt1 = keccak256(abi.encodePacked("multi-module-1"));
        bytes32 chainSalt1 = keccak256(abi.encodePacked("multi-chain-1"));
        bytes32 moduleSalt2 = keccak256(abi.encodePacked("multi-module-2"));
        bytes32 chainSalt2 = keccak256(abi.encodePacked("multi-chain-2"));

        // Deploy with RequireAndModule
        (address chain1, address module1, uint256 chainId1) = wrapper.deployCompleteSyndicate(
            12001, user1, SyndicateFactoryWrapper.ModuleType.RequireAnd, moduleSalt1, chainSalt1
        );

        // Deploy with RequireOrModule
        (address chain2, address module2, uint256 chainId2) = wrapper.deployCompleteSyndicate(
            12002, user2, SyndicateFactoryWrapper.ModuleType.RequireOr, moduleSalt2, chainSalt2
        );

        // All addresses should be different
        assertTrue(chain1 != chain2);
        assertTrue(module1 != module2);
        assertTrue(chainId1 != chainId2);

        // Verify each module type
        RequireAndModule andModule = RequireAndModule(module1);
        RequireOrModule orModule = RequireOrModule(module2);

        assertEq(andModule.owner(), user1);
        assertEq(orModule.owner(), user2);
    }

    // Fuzz testing
    function testFuzzDeployWithDifferentParameters(
        address _admin,
        uint256 _chainId,
        bytes32 _moduleSalt,
        bytes32 _chainSalt
    ) public {
        vm.assume(_admin != address(0));
        vm.assume(_chainId != 0);
        vm.assume(_chainId < type(uint256).max / 2); // Avoid overflow in chain ID logic
        vm.assume(wrapper.isChainIdUsed(_chainId) == 0); // Not already used

        (address sequencingChain, address permissionModule, uint256 actualChainId) = wrapper.deployCompleteSyndicate(
            _chainId, _admin, SyndicateFactoryWrapper.ModuleType.RequireAnd, _moduleSalt, _chainSalt
        );

        assertTrue(sequencingChain != address(0));
        assertTrue(permissionModule != address(0));
        assertEq(actualChainId, _chainId);

        RequireAndModule module = RequireAndModule(permissionModule);
        assertEq(module.owner(), _admin);
    }
}
