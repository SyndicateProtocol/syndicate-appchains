// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain, SequencingModuleChecker} from "src/SyndicateSequencingChain.sol";
import {SyndicateFactory, IGasAggregator as IFactoryGasAggregator} from "src/factory/SyndicateFactory.sol";
import {
    SyndicateSequencingChain,
    L2MessageType_SignedTx,
    SequencingModuleChecker,
    IGasAggregator
} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract MockIsAllowed is IPermissionModule {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address, address, bytes calldata) external view override returns (bool) {
        return allowed;
    }
}

contract MockIsAllowedWithInvalidData is IPermissionModule {
    function isAllowed(address, address, bytes calldata data) external pure override returns (bool) {
        return keccak256(data) != keccak256(abi.encodePacked(L2MessageType_SignedTx, "invalid"));
    }
}

contract DirectMockModule is IPermissionModule {
    mapping(bytes => bool) public allowedData;

    function setAllowed(bytes memory data, bool allowed) external {
        allowedData[data] = allowed;
    }

    function isAllowed(address, address, bytes calldata data) external view override returns (bool) {
        return allowedData[data];
    }
}

contract SyndicateSequencingChainTestSetUp is Test {
    SyndicateSequencingChain public chain;
    SyndicateFactory public factory;
    RequireAndModule public permissionModule;
    RequireOrModule public permissionModuleAny;
    address public admin;

    function deployFromFactory(RequireAndModule _permissionModule) public returns (SyndicateSequencingChain) {
        uint256 appchainId = 10042001;
        vm.startPrank(admin);
        SyndicateFactory implementation = new SyndicateFactory();
        bytes memory initData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        factory = SyndicateFactory(address(proxy));
        (address chainAddress,) =
            factory.createSyndicateSequencingChainWithCustomId(appchainId, admin, _permissionModule);
        vm.stopPrank();
        return SyndicateSequencingChain(chainAddress);
    }

    function setUp() public virtual {
        // Warp to START_TIMESTAMP to avoid underflow in epoch calculations
        vm.warp(1754089200); // START_TIMESTAMP from EpochTracker.sol

        admin = address(0x1);
        permissionModule = new RequireAndModule(admin);
        permissionModuleAny = new RequireOrModule(admin);
        chain = deployFromFactory(permissionModule);
    }
}

contract SyndicateSequencingChainTest is SyndicateSequencingChainTestSetUp {
    mapping(address => bool) public mockAllowedImplementations;
    mapping(uint256 => bool) public mockBannedChains;

    function isImplementationAllowed(address implementation) external view returns (bool) {
        return mockAllowedImplementations[implementation];
    }

    function notifyChainUpgrade(uint256 chainID, address impl) external {
        if (!mockAllowedImplementations[impl]) mockBannedChains[chainID] = true;
    }

    function allowedImplementations(address implementation) external view returns (bool) {
        return mockAllowedImplementations[implementation];
    }

    function notifyNewImplementation(address implementation) external {
        mockAllowedImplementations[implementation] = true;
    }

    // Helper function to manually set allowed implementations for testing
    function setMockAllowedImplementation(address implementation, bool allowed) public {
        mockAllowedImplementations[implementation] = allowed;
    }

    function testProcessRawTransaction() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit SyndicateSequencingChain.TransactionProcessed(
            address(this), abi.encodePacked(L2MessageType_SignedTx, validTxn)
        );

        chain.processTransaction(validTxn);
    }

    function testProcessTransactionRequireAllFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");
        address mockRequireAll = address(new MockIsAllowed(false));

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(mockRequireAll, false);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(
                RequireAndModule.AndPermissionCheckFailed.selector,
                mockRequireAll,
                address(this),
                abi.encodePacked(L2MessageType_SignedTx, validTxn)
            )
        );
        chain.processTransaction(validTxn);
    }

    function testProcessTransactionRequireAnyFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        chain.updateRequirementModule(address(permissionModuleAny));
        permissionModuleAny.addPermissionCheck(address(new MockIsAllowed(false)), false);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(
                RequireOrModule.AllOrPermissionChecksFailed.selector,
                address(this),
                abi.encodePacked(L2MessageType_SignedTx, validTxn)
            )
        );
        chain.processTransaction(validTxn);
    }

    function testProcessTransaction() public {
        bytes memory data = abi.encode("raw transaction");

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit SyndicateSequencingChain.TransactionProcessed(
            address(this), abi.encodePacked(L2MessageType_SignedTx, data)
        );

        chain.processTransaction(data);
    }

    function testProcessTransactionsBulk() public {
        bytes[] memory validTxns = new bytes[](3);
        validTxns[0] = abi.encode("transaction 1");
        validTxns[1] = abi.encode("transaction 2");
        validTxns[2] = abi.encode("transaction 3");

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        for (uint256 i = 0; i < validTxns.length; i++) {
            vm.expectEmit(true, false, false, true);

            emit SyndicateSequencingChain.TransactionProcessed(
                address(this), abi.encodePacked(L2MessageType_SignedTx, validTxns[i])
            );
        }

        chain.processTransactionsBulk(validTxns);
    }

    function testConstructorWithZeroAppChainId() public {
        address chainImpl = address(new SyndicateSequencingChain());
        address chainProxy = address(new ERC1967Proxy(chainImpl, bytes("")));

        vm.expectRevert("App chain ID cannot be 0");
        SyndicateSequencingChain(chainProxy).initialize(admin, address(permissionModule), 0);
    }

    function testUpgradeBadguy() public {
        address chainImpl = address(new SyndicateSequencingChain());
        address chainProxy = address(new ERC1967Proxy(chainImpl, bytes("")));
        SyndicateSequencingChain(chainProxy).initialize(admin, address(permissionModule), 1);

        // Set up gas aggregator
        vm.prank(admin);
        SyndicateSequencingChain(chainProxy).setGasAggregator(IGasAggregator(address(this)));

        address badguy = makeAddr("badguy");
        vm.prank(badguy);
        vm.expectRevert();
        UUPSUpgradeable(chainProxy).upgradeToAndCall(chainImpl, bytes(""));
    }

    function testUpgradeOwner() public {
        address chainImpl = address(new SyndicateSequencingChain());
        address chainProxy = address(new ERC1967Proxy(chainImpl, bytes("")));
        SyndicateSequencingChain(chainProxy).initialize(admin, address(permissionModule), 1);

        // Set up gas aggregator
        vm.prank(admin);
        SyndicateSequencingChain(chainProxy).setGasAggregator(IGasAggregator(address(this)));

        // Allow the implementation
        setMockAllowedImplementation(chainImpl, true);

        vm.prank(admin);
        UUPSUpgradeable(chainProxy).upgradeToAndCall(chainImpl, bytes(""));
    }

    function testUpgradeWithAllowedImplementation() public {
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        // Deploy chain through factory
        RequireAndModule testPermissionModule = new RequireAndModule(admin);
        SyndicateFactory implementation2 = new SyndicateFactory();
        bytes memory initData2 = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy2 = new ERC1967Proxy(address(implementation2), initData2);
        SyndicateFactory testFactory = SyndicateFactory(address(proxy2));

        vm.startPrank(admin);
        (address chainAddr,) = testFactory.createSyndicateSequencingChainWithCustomId(123, admin, testPermissionModule);
        // Set up gas aggregator and add implementation to allowlist
        testFactory.setGasAggregator(IFactoryGasAggregator(address(this)));
        SyndicateSequencingChain(chainAddr).setGasAggregator(IGasAggregator(address(this)));
        vm.stopPrank();

        // Allow the new implementation
        setMockAllowedImplementation(address(newImpl), true);

        // Upgrade should succeed since we mock the allowlist check
        // Set allowGasTrackingBanOnUpgrade to false (default is true)
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).setAllowGasTrackingBanOnUpgrade(false);

        // Perform the upgrade
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).upgradeToAndCall(address(newImpl), "");
    }

    function testUpgradeWithDisallowedImplementationAllowBan() public {
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        // Deploy chain through factory
        RequireAndModule testPermissionModule = new RequireAndModule(admin);
        SyndicateFactory implementation2 = new SyndicateFactory();
        bytes memory initData2 = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy2 = new ERC1967Proxy(address(implementation2), initData2);
        SyndicateFactory testFactory = SyndicateFactory(address(proxy2));

        vm.startPrank(admin);
        (address chainAddr, uint256 chainId) =
            testFactory.createSyndicateSequencingChainWithCustomId(123, admin, testPermissionModule);
        // Set up gas aggregator on the chain
        SyndicateSequencingChain(chainAddr).setGasAggregator(IGasAggregator(address(this)));
        vm.stopPrank();

        // Upgrade should succeed with allowGasTrackingBan=true even though implementation not allowed
        // Set allowGasTrackingBanOnUpgrade to true (this is the default)
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).setAllowGasTrackingBanOnUpgrade(true);

        // Perform the upgrade
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).upgradeToAndCall(address(newImpl), "");
    }

    function testUpgradeWithDisallowedImplementationRevertsBan() public {
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        // Deploy chain through factory
        RequireAndModule testPermissionModule = new RequireAndModule(admin);
        SyndicateFactory implementation2 = new SyndicateFactory();
        bytes memory initData2 = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy2 = new ERC1967Proxy(address(implementation2), initData2);
        SyndicateFactory testFactory = SyndicateFactory(address(proxy2));

        vm.startPrank(admin);
        (address chainAddr,) = testFactory.createSyndicateSequencingChainWithCustomId(123, admin, testPermissionModule);
        // Set up gas aggregator on the chain
        SyndicateSequencingChain(chainAddr).setGasAggregator(IGasAggregator(address(this)));
        vm.stopPrank();

        // Make sure newImpl is NOT allowed
        setMockAllowedImplementation(address(newImpl), false);

        // Upgrade should revert with allowGasTrackingBan=false
        // Set allowGasTrackingBanOnUpgrade to false
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).setAllowGasTrackingBanOnUpgrade(false);

        // Attempt upgrade - should fail
        vm.prank(admin);
        vm.expectRevert("Upgrade would result in gas tracking ban");
        SyndicateSequencingChain(chainAddr).upgradeToAndCall(address(newImpl), "");
    }

    function testUpgradeAuthorizationOnlyOwner() public {
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        // Deploy chain through factory
        RequireAndModule testPermissionModule = new RequireAndModule(admin);
        SyndicateFactory implementation2 = new SyndicateFactory();
        bytes memory initData2 = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy2 = new ERC1967Proxy(address(implementation2), initData2);
        SyndicateFactory testFactory = SyndicateFactory(address(proxy2));

        vm.startPrank(admin);
        (address chainAddr,) = testFactory.createSyndicateSequencingChainWithCustomId(123, admin, testPermissionModule);
        testFactory.setGasAggregator(IFactoryGasAggregator(address(this)));
        SyndicateSequencingChain(chainAddr).setGasAggregator(IGasAggregator(address(this)));
        vm.stopPrank();

        address nonOwner = makeAddr("nonOwner");

        // Non-owner should not be able to perform upgrade
        vm.prank(nonOwner);
        vm.expectRevert(); // Ownable revert from _authorizeUpgrade
        SyndicateSequencingChain(chainAddr).upgradeToAndCall(address(newImpl), "");
    }

    function testUpgradeChecksImplementationCorrectly() public {
        // Deploy chain through factory
        RequireAndModule testPermissionModule = new RequireAndModule(admin);
        SyndicateFactory implementation2 = new SyndicateFactory();
        bytes memory initData2 = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy proxy2 = new ERC1967Proxy(address(implementation2), initData2);
        SyndicateFactory testFactory = SyndicateFactory(address(proxy2));

        vm.startPrank(admin);
        (address chainAddr,) = testFactory.createSyndicateSequencingChainWithCustomId(123, admin, testPermissionModule);
        vm.stopPrank();

        // Create two different implementations
        SyndicateSequencingChain impl1 = new SyndicateSequencingChain();
        SyndicateSequencingChain impl2 = new SyndicateSequencingChain();

        // Set up gas aggregator to mock implementation checking
        vm.prank(admin);
        testFactory.setGasAggregator(IFactoryGasAggregator(address(this)));
        // Also set the gas aggregator on the chain itself
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).setGasAggregator(IGasAggregator(address(this)));

        // Set up the mock: allow only impl1
        setMockAllowedImplementation(address(impl1), true);

        // Verify impl1 upgrade works
        // Set allowGasTrackingBanOnUpgrade to false first
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).setAllowGasTrackingBanOnUpgrade(false);

        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).upgradeToAndCall(address(impl1), "");

        // Verify impl2 upgrade fails with allowGasTrackingBan=false
        vm.prank(admin);
        vm.expectRevert("Upgrade would result in gas tracking ban");
        SyndicateSequencingChain(chainAddr).upgradeToAndCall(address(impl2), "");

        // Verify that the chain is NOT blacklisted on the gas aggregator
        assertFalse(mockBannedChains[123]);

        // Set allowGasTrackingBanOnUpgrade to true
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).setAllowGasTrackingBanOnUpgrade(true);

        // Verify impl2 upgrade succeds, but blacklists the chain on the gas aggregator
        vm.prank(admin);
        SyndicateSequencingChain(chainAddr).upgradeToAndCall(address(impl2), "");
        assertTrue(mockBannedChains[123]);
    }

    function testProcessTransactionsBulkAllAllowed() public {
        // Deploy a module we can directly control
        DirectMockModule directMock = new DirectMockModule();

        // Set up the chain with our custom module
        vm.startPrank(admin);
        chain.updateRequirementModule(address(directMock));
        vm.stopPrank();

        // Prepare test data
        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("transaction 1");
        txns[1] = abi.encode("transaction 2");
        txns[2] = abi.encode("transaction 3");

        // Configure mock to allow all transactions
        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, txns[0]), true);
        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, txns[1]), true);
        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, txns[2]), true);

        // Expect events for all transactions
        for (uint256 i = 0; i < txns.length; i++) {
            vm.expectEmit(true, false, false, true);
            emit SyndicateSequencingChain.TransactionProcessed(
                address(this), abi.encodePacked(L2MessageType_SignedTx, txns[i])
            );
        }

        // Process all transactions
        chain.processTransactionsBulk(txns);
    }

    function testProcessTransactionsBulkBranchCoverage() public {
        // Deploy a module we can directly control
        DirectMockModule directMock = new DirectMockModule();

        // Set up the chain with our custom module
        vm.startPrank(admin);
        chain.updateRequirementModule(address(directMock));
        vm.stopPrank();

        // Part 1: Test the failure branch
        bytes[] memory failingTxns = new bytes[](2);
        failingTxns[0] = abi.encode("allowed tx");
        failingTxns[1] = abi.encode("disallowed tx");

        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, failingTxns[0]), true);
        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, failingTxns[1]), false);

        chain.processTransactionsBulk(failingTxns);

        // Part 2: Test the success branch
        bytes[] memory successTxns = new bytes[](2);
        successTxns[0] = abi.encode("allowed tx 1");
        successTxns[1] = abi.encode("allowed tx 2");

        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, successTxns[0]), true);
        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, successTxns[1]), true);

        // Expect events for successful transactions
        for (uint256 i = 0; i < successTxns.length; i++) {
            vm.expectEmit(true, false, false, true);
            emit SyndicateSequencingChain.TransactionProcessed(
                address(this), abi.encodePacked(L2MessageType_SignedTx, successTxns[i])
            );
        }

        chain.processTransactionsBulk(successTxns);
    }

    function testProcessTransactionsBulkOnlyEmitsValidTransactionsAsEvents() public {
        chain = deployFromFactory(RequireAndModule(address(new MockIsAllowedWithInvalidData())));

        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encodePacked("valid");
        txns[1] = abi.encodePacked("invalid");
        txns[2] = abi.encodePacked("valid");

        vm.recordLogs();
        chain.processTransactionsBulk(txns);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bytes32 expectedSig = keccak256("TransactionProcessed(address,bytes)");

        uint256 validEventCount = 0;
        uint256 expectedValidEventCount = 2;

        for (uint256 i = 0; i < logs.length; i++) {
            Vm.Log memory log = logs[i];

            if (log.topics.length > 0 && log.topics[0] == expectedSig) {
                if (keccak256(log.data) == keccak256(abi.encodePacked(L2MessageType_SignedTx, "invalid"))) {
                    fail();
                }

                validEventCount++;
            }
        }

        assertEq(validEventCount, expectedValidEventCount, "Wrong amount of valid transaction events emitted");
    }

    function testOnlyWhenAllowedModifierBranches() public {
        // Deploy a module we can directly control
        DirectMockModule directMock = new DirectMockModule();

        // Set up the chain with our custom module
        vm.startPrank(admin);
        chain.updateRequirementModule(address(directMock));
        vm.stopPrank();

        bytes memory allowedData = abi.encode("allowed data");
        bytes memory disallowedData = abi.encode("disallowed data");

        // Configure permissions
        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, allowedData), true);
        directMock.setAllowed(abi.encodePacked(L2MessageType_SignedTx, disallowedData), false);

        // Test 1: Failure path of onlyWhenAllowed (processTransaction)
        vm.expectRevert(SyndicateSequencingChain.TransactionOrSenderNotAllowed.selector);
        chain.processTransaction(disallowedData);

        // Test 2: Success path of onlyWhenAllowed (processTransaction)
        vm.expectEmit(true, false, false, true);
        emit SyndicateSequencingChain.TransactionProcessed(
            address(this), abi.encodePacked(L2MessageType_SignedTx, allowedData)
        );
        chain.processTransaction(allowedData);
    }

    function testProcessTransactionsBulkWithEmptyArray() public {
        bytes[] memory emptyArray = new bytes[](0);
        vm.expectRevert(SyndicateSequencingChain.NoTxData.selector);
        chain.processTransactionsBulk(emptyArray);
    }

    function testEmissionsReceiver() public {
        // Test defaults to owner
        assertEq(chain.getEmissionsReceiver(), admin);

        // Test only owner can set it
        address newReceiver = address(0x999);
        address nonOwner = address(0x123);
        vm.prank(nonOwner);
        vm.expectRevert(abi.encodeWithSignature("OwnableUnauthorizedAccount(address)", nonOwner));
        chain.setEmissionsReceiver(newReceiver);

        // Test owner can set it and it returns correct value with proper event
        vm.prank(admin);
        vm.expectEmit(true, true, false, false);
        emit SyndicateSequencingChain.EmissionsReceiverUpdated(address(0), newReceiver);
        chain.setEmissionsReceiver(newReceiver);
        assertEq(chain.getEmissionsReceiver(), newReceiver);

        // falls back to owner if emissionsReceiver is set to address(0)
        vm.prank(admin);
        vm.expectEmit(true, true, false, false);
        emit SyndicateSequencingChain.EmissionsReceiverUpdated(newReceiver, admin);
        chain.setEmissionsReceiver(address(0));
        assertEq(chain.getEmissionsReceiver(), admin);
    }

    function testTransferOwnershipEmitsEmissionsReceiverUpdated() public {
        // Test that transferOwnership emits EmissionsReceiverUpdated when emissionsReceiver is not set
        address newOwner = address(0x888);

        vm.prank(admin);
        vm.expectEmit(true, true, false, false);
        emit SyndicateSequencingChain.EmissionsReceiverUpdated(admin, newOwner);
        chain.transferOwnership(newOwner);

        // Verify the emissions receiver changed
        assertEq(chain.getEmissionsReceiver(), newOwner);
        assertEq(chain.owner(), newOwner);

        // Test that transferOwnership does NOT emit EmissionsReceiverUpdated when emissionsReceiver is explicitly set
        address explicitReceiver = address(0x777);
        vm.prank(newOwner);
        chain.setEmissionsReceiver(explicitReceiver);

        address anotherNewOwner = address(0x666);
        vm.prank(newOwner);
        // Should NOT emit EmissionsReceiverUpdated
        vm.recordLogs();
        chain.transferOwnership(anotherNewOwner);

        Vm.Log[] memory logs = vm.getRecordedLogs();
        // Should only have OwnershipTransferred event, not EmissionsReceiverUpdated
        bool foundEmissionsEvent = false;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == keccak256("EmissionsReceiverUpdated(address,address)")) {
                foundEmissionsEvent = true;
                break;
            }
        }
        assertFalse(foundEmissionsEvent, "Should not emit EmissionsReceiverUpdated when explicit receiver is set");

        // Verify emissions receiver stayed the same
        assertEq(chain.getEmissionsReceiver(), explicitReceiver);
        assertEq(chain.owner(), anotherNewOwner);
    }
}

contract SyndicateSequencingChainViewRequireAllTest is SyndicateSequencingChainTestSetUp {
    MockIsAllowed mockRequireAll1;
    MockIsAllowed mockRequireAll2;

    function setUp() public override {
        super.setUp();
        mockRequireAll1 = new MockIsAllowed(true);
        mockRequireAll2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(mockRequireAll1), false);
        permissionModule.addPermissionCheck(address(mockRequireAll2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAll() public view {
        address[] memory allChecks = permissionModule.getAllPermissionChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAll1));
        assertEq(allChecks[1], address(mockRequireAll2));
    }
}

contract SyndicateSequencingChainViewRequireAnyTest is SyndicateSequencingChainTestSetUp {
    MockIsAllowed mockRequireAny1;
    MockIsAllowed mockRequireAny2;

    function setUp() public override {
        super.setUp();

        mockRequireAny1 = new MockIsAllowed(false);
        mockRequireAny2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.updateRequirementModule(address(permissionModuleAny));

        permissionModuleAny.addPermissionCheck(address(mockRequireAny1), false);
        permissionModuleAny.addPermissionCheck(address(mockRequireAny2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAny() public view {
        address[] memory allChecks = permissionModuleAny.getAllPermissionChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAny1));
        assertEq(allChecks[1], address(mockRequireAny2));
    }

    // ================== VERSION TRACKING TESTS ==================

    function testInitialVersionInSyndicateSequencingChain() public view {
        assertEq(chain.version(), "1.0.0", "Initial version should be 1.0.0");
    }

    function testUpdateVersionInSyndicateSequencingChain() public {
        vm.prank(admin);
        chain.updateVersion("1.5.0");

        assertEq(chain.version(), "1.5.0", "Version should be updated to 1.5.0");
    }

    function testUpdateVersionOnlyOwner() public {
        address nonOwner = address(999);

        vm.prank(nonOwner);
        vm.expectRevert(); // Ownable error
        chain.updateVersion("1.1.0");
    }

    function testVersionPersistsAfterOperations() public {
        // Update version
        vm.prank(admin);
        chain.updateVersion("2.1.0");

        // Perform chain operations
        vm.prank(admin);
        chain.setEmissionsReceiver(address(0x1234));

        // Version should still be the same
        assertEq(chain.version(), "2.1.0", "Version should persist after operations");
    }

    function testVersionUsesNamespacedStorage() public {
        // Test that version is properly stored in namespaced storage
        // and doesn't interfere with other storage variables
        vm.prank(admin);
        chain.updateVersion("3.2.1");

        // Other storage should remain intact
        assertEq(chain.appchainId(), 10042001, "AppchainId should remain intact");
        assertEq(chain.factory(), address(factory), "Factory should remain intact");
        assertEq(chain.version(), "3.2.1", "Version should be correctly stored");
    }
}
