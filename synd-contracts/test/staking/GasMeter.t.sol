pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {GasMeter} from "../../src/staking/GasMeter.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract GasMeterTestHelper {
    GasMeter public gasMeter;

    event Sequence(address indexed sequencer, uint256 loops);

    event RawData(bytes data, uint256 length);

    constructor(GasMeter _gasMeter) {
        gasMeter = _gasMeter;
    }

    function callTrackCall(uint256 loops) external {
        bytes memory meteredCall = abi.encodeWithSelector(this.sequence.selector, msg.sender, loops);
        gasMeter.meterCall(meteredCall);
    }

    function sequence(address sequencer, uint256 loops) external {
        require(msg.sender == address(gasMeter), "Only callable by GasMeter");
        for (uint256 i = 0; i < loops; i++) {
            // Loop to use gas
        }
        emit Sequence(sequencer, loops);
    }

    function trackedSequence(uint256 loops) external returns (uint256) {
        uint256 startGas = gasleft();
        for (uint256 i = 0; i < loops; i++) {
            // Loop to use gas
        }
        emit Sequence(msg.sender, loops);
        return startGas - gasleft();
    }
}

contract GasMeterTest is Test {
    address public gasMeterImpl;
    GasMeter public gasMeter;
    GasMeterTestHelper public gasMeterTestHelper;

    address public admin;
    uint256 public epoch = 1;

    function setUp() public {
        admin = makeAddr("admin");

        // Deploy GasTracker implementation
        gasMeterImpl = address(new GasMeter());

        // Deploy GasTracker proxy
        vm.prank(admin);
        gasMeter = GasMeter(address(new ERC1967Proxy(gasMeterImpl, abi.encodeCall(GasMeter.initialize, ()))));

        gasMeterTestHelper = new GasMeterTestHelper(gasMeter);

        vm.warp(gasMeter.getEpochStart(epoch));
    }

    function testMeteredCall() public {
        gasMeterTestHelper.callTrackCall(1000);
        uint256 result = gasMeter.gasUsed(epoch, address(gasMeterTestHelper));

        assertApproxEqAbs(result, gasMeterTestHelper.trackedSequence(1000), 1000);
    }

    function testEpochTracking() public {
        gasMeterTestHelper.callTrackCall(1000);
        uint256 result = gasMeter.gasUsed(epoch, address(gasMeterTestHelper));
        assertApproxEqAbs(result, gasMeterTestHelper.trackedSequence(1000), 1000);

        epoch++;
        vm.warp(gasMeter.getEpochStart(epoch));
        gasMeterTestHelper.callTrackCall(10000);
        result = gasMeter.gasUsed(epoch, address(gasMeterTestHelper));
        assertApproxEqAbs(result, gasMeterTestHelper.trackedSequence(10000), 1000);

        result = gasMeter.gasUsed(epoch + 1, address(gasMeterTestHelper));
        assertEq(result, 0);
    }

    function testDifferentChains() public {
        GasMeterTestHelper gasMeterTestHelper2 = new GasMeterTestHelper(gasMeter);
        gasMeterTestHelper2.callTrackCall(1000);
        uint256 result = gasMeter.gasUsed(epoch, address(gasMeterTestHelper2));
        assertApproxEqAbs(result, gasMeterTestHelper2.trackedSequence(1000), 1000);

        gasMeterTestHelper.callTrackCall(1000);
        result = gasMeter.gasUsed(epoch, address(gasMeterTestHelper));
        assertApproxEqAbs(result, gasMeterTestHelper.trackedSequence(1000), 1000);
    }
}
