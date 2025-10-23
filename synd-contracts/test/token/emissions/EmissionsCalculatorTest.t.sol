// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {EmissionsCalculator} from "src/token/emissions/EmissionsCalculator.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";

contract EmissionsCalculatorTest is Test {
    EmissionsCalculator public calculator;
    SyndicateToken public token;

    address public admin = address(0x1234);
    address public changeFactorManager = address(0x5678);
    address public treasury = address(0x9ABC);
    address public user = address(0x1111);

    uint256 public constant SCALE = 1e18;
    uint256 public constant EMISSIONS_CAP = 80_000_000 * 10 ** 18;
    uint256 public constant TOTAL_EPOCHS = 48;

    event ChangeFactorSet(uint256 indexed epoch, uint256 changeFactor, address indexed setter);
    event EmissionMinted(uint256 indexed epoch, uint256 amount, uint256 remainingSupply, address indexed to);
    event EmissionsInitialized(uint256 defaultChangeFactor);

    function setUp() public {
        // Deploy token
        token = new SyndicateToken(admin, treasury);

        // Deploy calculator
        calculator = new EmissionsCalculator(address(token), admin, changeFactorManager);

        // Grant emission minter role to calculator
        bytes32 emissionMinterRole = token.EMISSION_MINTER_ROLE();
        vm.prank(admin);
        token.grantRole(emissionMinterRole, address(calculator));

        // Grant emissions role to calculator
        bytes32 emissionsRole = calculator.EMISSIONS_ROLE();
        vm.prank(admin);
        calculator.grantRole(emissionsRole, admin);
    }

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Constructor_InitialSetup() public view {
        assertEq(address(calculator.syndicateToken()), address(token));
        assertEq(calculator.TOTAL_EPOCHS(), 48);
        assertEq(calculator.EMISSIONS_CAP(), EMISSIONS_CAP);
        assertEq(calculator.SCALE(), SCALE);
        assertEq(calculator.currentEpoch(), 0);
        assertEq(calculator.totalEmitted(), 0);
        assertFalse(calculator.initialized());
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(calculator.hasRole(calculator.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(calculator.hasRole(calculator.CHANGE_FACTOR_MANAGER_ROLE(), changeFactorManager));
    }

    function test_RevertWhen_Constructor_ZeroAddresses() public {
        vm.expectRevert(EmissionsCalculator.ZeroAddress.selector);
        new EmissionsCalculator(address(0), admin, changeFactorManager);

        vm.expectRevert(EmissionsCalculator.ZeroAddress.selector);
        new EmissionsCalculator(address(token), address(0), changeFactorManager);

        vm.expectRevert(EmissionsCalculator.ZeroAddress.selector);
        new EmissionsCalculator(address(token), admin, address(0));
    }

    /*//////////////////////////////////////////////////////////////
                        INITIALIZATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_InitializeEmissions_Success() public {
        uint256 defaultChangeFactor = 0.95e18; // 95% change factor

        vm.expectEmit(false, false, false, true);
        emit EmissionsInitialized(defaultChangeFactor);

        vm.prank(admin);
        calculator.initializeEmissions(defaultChangeFactor);

        assertTrue(calculator.initialized());

        // Check that all epochs have the default change factor
        assertEq(calculator.changeFactor(), defaultChangeFactor);
    }

    function test_RevertWhen_InitializeEmissions_NotAdmin() public {
        vm.prank(user);
        vm.expectRevert();
        calculator.initializeEmissions(0.95e18);
    }

    function test_RevertWhen_InitializeEmissions_InvalidChangeFactor() public {
        vm.prank(admin);
        vm.expectRevert(EmissionsCalculator.InvalidChangeFactor.selector);
        calculator.initializeEmissions(0); // Zero change factor
    }

    function test_RevertWhen_InitializeEmissions_AlreadyInitialized() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        vm.prank(admin);
        vm.expectRevert(EmissionsCalculator.EmissionsCompleted.selector);
        calculator.initializeEmissions(0.9e18);
    }

    /*//////////////////////////////////////////////////////////////
                        CHANGE FACTOR TESTS
    //////////////////////////////////////////////////////////////*/

    function test_SetChangeFactor_Success() public {
        // Initialize first
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        uint256 newChangeFactor = 0.9e18;

        vm.expectEmit(true, false, true, true);
        emit ChangeFactorSet(0, newChangeFactor, changeFactorManager);

        vm.prank(changeFactorManager);
        calculator.setChangeFactor(newChangeFactor);

        assertEq(calculator.changeFactor(), newChangeFactor);
    }

    function test_SetChangeFactor_Zero() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        vm.prank(changeFactorManager);
        vm.expectRevert(EmissionsCalculator.InvalidChangeFactor.selector);
        calculator.setChangeFactor(0);
    }

    /*//////////////////////////////////////////////////////////////
                    EMISSION CALCULATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_CalculateAndMintEmission_FirstEpoch() public {
        uint256 changeFactor = 0.95e18; // 95% change factor
        vm.prank(admin);
        calculator.initializeEmissions(changeFactor);

        uint256 remainingSupply = calculator.getRemainingSupply();
        assertEq(remainingSupply, EMISSIONS_CAP);

        // Calculate expected emission for first epoch
        uint256 pt = calculator.calculateCumulativeProduct();
        uint256 expectedEmission = (remainingSupply * (SCALE - changeFactor)) / (SCALE - pt);

        uint256 initialBalance = token.balanceOf(treasury);

        vm.expectEmit(true, false, false, true);
        emit EmissionMinted(0, expectedEmission, remainingSupply - expectedEmission, treasury);

        vm.prank(admin);
        uint256 actualEmission = calculator.calculateAndMintEmission(treasury, 0);

        assertEq(actualEmission, expectedEmission);
        assertEq(token.balanceOf(treasury), initialBalance + expectedEmission);
        assertEq(calculator.currentEpoch(), 1);
        assertEq(calculator.totalEmitted(), expectedEmission);
    }

    function test_CalculateAndMintEmission_FinalEpoch() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        // Fast forward to final epoch (47)
        for (uint256 i = 0; i < TOTAL_EPOCHS - 1; i++) {
            vm.prank(admin);
            calculator.calculateAndMintEmission(treasury, i);
        }

        uint256 remainingSupply = calculator.getRemainingSupply();
        uint256 initialBalance = token.balanceOf(treasury);

        // Final epoch should sweep all remaining tokens
        vm.prank(admin);
        uint256 finalEmission = calculator.calculateAndMintEmission(treasury, TOTAL_EPOCHS - 1);

        assertEq(finalEmission, remainingSupply);
        assertEq(token.balanceOf(treasury), initialBalance + finalEmission);
        assertEq(calculator.currentEpoch(), TOTAL_EPOCHS);
        assertTrue(calculator.isCompleted());
    }

    function test_RevertWhen_CalculateAndMintEmission_NotInitialized() public {
        vm.prank(admin);
        vm.expectRevert(EmissionsCalculator.EmissionsCompleted.selector);
        calculator.calculateAndMintEmission(treasury, 0);
    }

    function test_RevertWhen_CalculateAndMintEmission_Completed() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        // Complete all epochs
        for (uint256 i = 0; i < TOTAL_EPOCHS; i++) {
            vm.prank(admin);
            calculator.calculateAndMintEmission(treasury, i);
        }

        // Try to mint another epoch
        vm.prank(admin);
        vm.expectRevert(EmissionsCalculator.EmissionsCompleted.selector);
        calculator.calculateAndMintEmission(treasury, TOTAL_EPOCHS);
    }

    function test_RevertWhen_CalculateAndMintEmission_ZeroAddress() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        vm.prank(admin);
        vm.expectRevert(EmissionsCalculator.ZeroAddress.selector);
        calculator.calculateAndMintEmission(address(0), 0);
    }

    /*//////////////////////////////////////////////////////////////
                        VIEW FUNCTION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_GetRemainingSupply() public view {
        uint256 remaining = calculator.getRemainingSupply();
        assertEq(remaining, EMISSIONS_CAP);
    }

    function test_CalculateCumulativeProduct() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        // Test cumulative product calculation
        uint256 product = calculator.calculateCumulativeProduct();

        // For all epochs with same change factor 0.95:
        // P_0 = 0.95^48
        uint256 expected = 0.95e18;
        for (uint256 i = 1; i < TOTAL_EPOCHS; i++) {
            expected = (expected * 0.95e18) / SCALE;
        }

        assertEq(product, expected);
    }

    function test_PreviewCurrentEmission() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        uint256 preview = calculator.getNextEmission();

        vm.prank(admin);
        uint256 actual = calculator.calculateAndMintEmission(treasury, 0);

        assertEq(preview, actual);
    }

    function test_GetEmissionsInfo() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        (uint256 current, uint256 total, uint256 emitted, uint256 remaining, bool completed) =
            calculator.getEmissionsInfo();

        assertEq(current, 0);
        assertEq(total, TOTAL_EPOCHS);
        assertEq(emitted, 0);
        assertEq(remaining, EMISSIONS_CAP);
        assertFalse(completed);

        // Mint one emission
        vm.prank(admin);
        uint256 firstEmission = calculator.calculateAndMintEmission(treasury, 0);

        (current, total, emitted, remaining, completed) = calculator.getEmissionsInfo();

        assertEq(current, 1);
        assertEq(total, TOTAL_EPOCHS);
        assertEq(emitted, firstEmission);
        assertEq(remaining, EMISSIONS_CAP - firstEmission);
        assertFalse(completed);
    }

    /*//////////////////////////////////////////////////////////////
                        INTEGRATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Integration_FullEmissionCycle() public {
        vm.prank(admin);
        calculator.initializeEmissions(0.98e18); // High change factor for testing

        uint256 totalMinted = 0;

        // Complete all 48 epochs
        for (uint256 i = 0; i < TOTAL_EPOCHS; i++) {
            vm.prank(admin);
            uint256 emission = calculator.calculateAndMintEmission(treasury, i);
            totalMinted += emission;
        }

        // Verify total emissions respect the cap
        assertLe(totalMinted, EMISSIONS_CAP);
        assertEq(calculator.totalEmitted(), totalMinted);
        assertTrue(calculator.isCompleted());
        assertEq(calculator.getRemainingSupply(), 0);
    }

    /*//////////////////////////////////////////////////////////////
                            FUZZ TESTS
    //////////////////////////////////////////////////////////////*/

    function testFuzz_ChangeFactor_ValidRange(uint256 changeFactor) public {
        changeFactor = bound(changeFactor, 1, SCALE - 1);

        vm.prank(admin);
        calculator.initializeEmissions(changeFactor);

        vm.prank(admin);
        uint256 emission = calculator.calculateAndMintEmission(treasury, 0);

        assertTrue(emission > 0);
        assertTrue(emission <= EMISSIONS_CAP);
    }

    function testFuzz_MultipleEpochs_EmissionSum(uint8 epochs) public {
        epochs = uint8(bound(epochs, 1, TOTAL_EPOCHS));

        vm.prank(admin);
        calculator.initializeEmissions(0.95e18);

        uint256 totalMinted = 0;
        for (uint256 i = 0; i < epochs; i++) {
            vm.prank(admin);
            uint256 emission = calculator.calculateAndMintEmission(treasury, i);
            totalMinted += emission;
        }

        assertEq(calculator.totalEmitted(), totalMinted);
        assertLe(totalMinted, EMISSIONS_CAP);
    }
}
