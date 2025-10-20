// // SPDX-License-Identifier: UNLICENSED
// pragma solidity 0.8.28;

// import {Test} from "forge-std/Test.sol";
// import {SyndicateFactory, ILegacyAppchain} from "src/factory/SyndicateFactory.sol";
// import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
// import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
// import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
// import {ERC1967Proxy} from "src/factory/SyndicateFactory.sol";
// import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
// import {GasAggregator} from "src/staking/GasAggregator.sol";
// import {IGasAggregator} from "src/interfaces/IGasAggregator.sol";
// import {MinimalUUPSStub} from "src/factory/MinimalUUPSStub.sol";

// contract MockLegacyAppchain {
//     mapping(uint256 => uint256) public tokensPerEpoch;
//     bool public shouldRevert;

//     function setTokensForEpoch(uint256 epoch, uint256 tokens) external {
//         tokensPerEpoch[epoch] = tokens;
//     }

//     function getTokensForEpoch(uint256 epoch) external view returns (uint256) {
//         if (shouldRevert) {
//             revert("Mock revert");
//         }
//         return tokensPerEpoch[epoch];
//     }

//     function setShouldRevert(bool _shouldRevert) external {
//         shouldRevert = _shouldRevert;
//     }
// }

// contract SyndicateFactoryMigrationTest is Test {
//     SyndicateFactory public factory;
//     MockLegacyAppchain public mockLegacyAppchain;
//     RequireAndModule public permissionModule;

//     address public admin;
//     address public nonAdmin;
//     uint256 public appchainId = 10042001;
//     uint256 public legacyGasTokens = 1000 ether;

//     // Constants for role checking
//     bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;

//     // Events copied from SyndicateFactory for testing
//     // Note: Solidity requires local event definitions for vm.expectEmit() testing
//     event AppchainMigrated(
//         address indexed oldAppchainContract,
//         address indexed newAppchainContract,
//         uint256 indexed appchainId,
//         uint256 epoch,
//         uint256 migratedGasTokensUsedForCurrentEpoch
//     );

//     event SyndicateSequencingChainCreated(
//         uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
//     );

//     function setUp() public {
//         vm.warp(1754089200 + 1 days); // after epoch start

//         admin = address(0x1);
//         nonAdmin = address(0x3);

//         // Deploy factory
//         SyndicateFactory implementation = new SyndicateFactory();
//         bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (admin));
//         ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
//         factory = SyndicateFactory(address(proxy));
//         // Deploy and set GasAggregator (non-upgradeable)
//         uint256 startEpoch = 1;
//         uint256 addChainFee = 5 ether;
//         uint256 maxAppchainsToQuery = 100;
//         GasAggregator gasAggregator = new GasAggregator(startEpoch, addChainFee, maxAppchainsToQuery);

//         vm.prank(admin);
//         factory.setGasAggregator(IGasAggregator(address(gasAggregator)));

//         // Deploy mock legacy appchain with code
//         mockLegacyAppchain = new MockLegacyAppchain();

//         // Set up permission module
//         permissionModule = new RequireAndModule(admin);
//     }

//     function _testMigrateLegacyAppchain(uint256 gasTokens) internal {
//         uint256 currentEpoch = factory.getCurrentEpoch();
//         mockLegacyAppchain.setTokensForEpoch(currentEpoch, gasTokens);

//         address expectedAddress = factory.computeSequencingChainAddress(appchainId);

//         // Expect both migration and creation events
//         vm.expectEmit(true, true, true, true);
//         emit SyndicateSequencingChainCreated(appchainId, expectedAddress, address(permissionModule));

//         vm.expectEmit(true, true, true, true);
//         emit AppchainMigrated(address(mockLegacyAppchain), expectedAddress, appchainId, currentEpoch, gasTokens);

//         vm.prank(admin);
//         address newSyndicateChain = factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, permissionModule
//         );

//         // Verify deployment
//         assertTrue(newSyndicateChain != address(0));
//         assertEq(newSyndicateChain, expectedAddress);
//         assertEq(factory.appchainContracts(appchainId), newSyndicateChain);
//         assertTrue(factory.isChainIdUsed(appchainId));

//         // Verify the new sequencing chain has correct setup
//         SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(newSyndicateChain);
//         assertEq(sequencingChain.appchainId(), appchainId);
//         assertEq(address(sequencingChain.permissionRequirementModule()), address(permissionModule));

//         // Verify gas tokens were migrated
//         assertEq(sequencingChain.getTokensForEpoch(currentEpoch), gasTokens);
//     }

//     function testMigrateLegacyAppchainSuccess() public {
//         _testMigrateLegacyAppchain(legacyGasTokens);
//     }

//     function testMigrateLegacyAppchainWithZeroGasTokens() public {
//         _testMigrateLegacyAppchain(0);
//     }

//     function testMigrateLegacyAppchainWithLargeGasTokens() public {
//         _testMigrateLegacyAppchain(type(uint256).max / 2);
//     }

//     function testMigrateLegacyAppchainMultipleChains() public {
//         uint256 currentEpoch = factory.getCurrentEpoch();

//         // Set up multiple legacy appchains
//         MockLegacyAppchain legacyAppchain1 = new MockLegacyAppchain();
//         MockLegacyAppchain legacyAppchain2 = new MockLegacyAppchain();

//         uint256 appchainId1 = 10001;
//         uint256 appchainId2 = 10002;
//         uint256 gasTokens1 = 500 ether;
//         uint256 gasTokens2 = 1500 ether;

//         legacyAppchain1.setTokensForEpoch(currentEpoch, gasTokens1);
//         legacyAppchain2.setTokensForEpoch(currentEpoch, gasTokens2);

//         RequireAndModule permissionModule2 = new RequireAndModule(admin);

//         // Migrate first chain
//         vm.prank(admin);
//         address newChain1 = factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(legacyAppchain1)), appchainId1, admin, permissionModule
//         );

//         // Migrate second chain
//         vm.prank(admin);
//         address newChain2 = factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(legacyAppchain2)), appchainId2, admin, permissionModule2
//         );

//         // Verify both migrations
//         assertTrue(newChain1 != address(0));
//         assertTrue(newChain2 != address(0));
//         assertTrue(newChain1 != newChain2);

//         SyndicateSequencingChain chain1 = SyndicateSequencingChain(newChain1);
//         SyndicateSequencingChain chain2 = SyndicateSequencingChain(newChain2);

//         assertEq(chain1.getTokensForEpoch(currentEpoch), gasTokens1);
//         assertEq(chain2.getTokensForEpoch(currentEpoch), gasTokens2);
//         assertEq(chain1.appchainId(), appchainId1);
//         assertEq(chain2.appchainId(), appchainId2);
//     }

//     function testMigrateLegacyAppchainPreservesGasCounterAcrossEpochs() public {
//         uint256 currentEpoch = factory.getCurrentEpoch();
//         uint256 previousEpoch = currentEpoch - 1;

//         // Set tokens for both current and previous epochs
//         mockLegacyAppchain.setTokensForEpoch(currentEpoch, legacyGasTokens);
//         mockLegacyAppchain.setTokensForEpoch(previousEpoch, 500 ether);

//         vm.prank(admin);
//         address newSyndicateChain = factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, permissionModule
//         );

//         // Verify only current epoch tokens are migrated
//         SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(newSyndicateChain);
//         assertEq(sequencingChain.getTokensForEpoch(currentEpoch), legacyGasTokens);
//     }

//     // Error condition tests
//     function testMigrateLegacyAppchainRevertsWithZeroLegacyAddress() public {
//         vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(address(0)), appchainId, admin, permissionModule);
//     }

//     function testMigrateLegacyAppchainRevertsWithZeroAdmin() public {
//         vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(mockLegacyAppchain)), appchainId, address(0), permissionModule
//         );
//     }

//     function testMigrateLegacyAppchainRevertsWithZeroPermissionModule() public {
//         vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, IRequirementModule(address(0))
//         );
//     }

//     function testMigrateLegacyAppchainRevertsWithZeroChainId() public {
//         vm.expectRevert(SyndicateFactory.ZeroAddress.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(address(mockLegacyAppchain)), 0, admin, permissionModule);
//     }

//     function testMigrateLegacyAppchainRevertsWithExistingChainId() public {
//         // First create a regular chain with the same ID
//         vm.prank(admin);
//         factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, permissionModule);

//         // Now try to migrate to the same ID - should fail
//         vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, permissionModule);
//     }

//     function testMigrateLegacyAppchainRevertsWithInvalidAppchainAddress() public {
//         // Create an EOA (no code) to simulate invalid appchain address
//         address invalidAppchain = address(0x999);

//         vm.expectRevert(SyndicateFactory.InvalidAppchainAddress.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(invalidAppchain), appchainId, admin, permissionModule);
//     }

//     function testMigrateLegacyAppchainRevertsWhenPaused() public {
//         uint256 currentEpoch = factory.getCurrentEpoch();
//         mockLegacyAppchain.setTokensForEpoch(currentEpoch, legacyGasTokens);

//         // Pause the factory
//         vm.prank(admin);
//         factory.pause();

//         vm.expectRevert(); // Pausable will revert
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, permissionModule);
//     }

//     function testMigrateLegacyAppchainRevertsWithNonAdmin() public {
//         uint256 currentEpoch = factory.getCurrentEpoch();
//         mockLegacyAppchain.setTokensForEpoch(currentEpoch, legacyGasTokens);

//         vm.expectRevert(); // AccessControl will revert
//         vm.prank(nonAdmin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, permissionModule);
//     }

//     function testMigrateLegacyAppchainHandlesGasTokensCallFailure() public {
//         // Set up the mock to revert when getTokensForEpoch is called
//         mockLegacyAppchain.setShouldRevert(true);

//         // The migration should fail when trying to get gas tokens
//         vm.expectRevert("Mock revert");
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, permissionModule);
//     }

//     function testMigrateLegacyAppchainCannotReuseAfterMigration() public {
//         uint256 currentEpoch = factory.getCurrentEpoch();
//         mockLegacyAppchain.setTokensForEpoch(currentEpoch, legacyGasTokens);

//         // First migration should succeed
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(ILegacyAppchain(address(mockLegacyAppchain)), appchainId, admin, permissionModule);

//         // Trying to migrate again to same chain ID should fail
//         uint256 appchainId2 = appchainId; // Same ID
//         vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(mockLegacyAppchain)), appchainId2, admin, permissionModule
//         );
//     }

//     function testMigrateLegacyAppchainDifferentAdminsAndPermissionModules() public {
//         uint256 currentEpoch = factory.getCurrentEpoch();
//         mockLegacyAppchain.setTokensForEpoch(currentEpoch, legacyGasTokens);

//         address differentAdmin = address(0x789);
//         RequireAndModule differentPermissionModule = new RequireAndModule(differentAdmin);

//         vm.prank(admin);
//         address newSyndicateChain = factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(mockLegacyAppchain)), appchainId, differentAdmin, differentPermissionModule
//         );

//         SyndicateSequencingChain sequencingChain = SyndicateSequencingChain(newSyndicateChain);
//         assertEq(address(sequencingChain.permissionRequirementModule()), address(differentPermissionModule));
//         assertEq(differentPermissionModule.owner(), differentAdmin);
//     }

//     function testMigrateLegacyAppchainConflict() public {
//         uint256 currentEpoch = factory.getCurrentEpoch();
//         mockLegacyAppchain.setTokensForEpoch(currentEpoch, legacyGasTokens);

//         // Create a chain first with a deterministic chainID
//         address newChainAdmin = address(0x555);
//         RequireAndModule perm = new RequireAndModule(admin);

//         vm.prank(newChainAdmin);
//         (, uint256 deterministicChainId) = factory.createSyndicateSequencingChain(0, admin, perm);

//         // Try to migrate to the same chain ID that was created deterministically - should fail
//         vm.expectRevert(SyndicateFactory.ChainIdAlreadyExists.selector);
//         vm.prank(admin);
//         factory.migrateLegacyAppchain(
//             ILegacyAppchain(address(mockLegacyAppchain)), deterministicChainId, admin, permissionModule
//         );
//     }
// }
