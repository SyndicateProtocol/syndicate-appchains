// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {L1Relayer} from "../src/staking/L1Relayer.sol";
import {SyndicateToken} from "../src/token/SyndicateToken.sol";
import {EmissionsCalculator} from "../src/token/emissions/EmissionsCalculator.sol";
import {EmissionsScheduler} from "../src/token/emissions/EmissionsScheduler.sol";

import {L2Relayer} from "../src/staking/L2Relayer.sol";

import {Refunder} from "../src/staking/Refunder.sol";
import {SyndStaking} from "../src/staking/SyndStaking.sol";
import {BasePool} from "../src/staking/BasePool.sol";

contract DeployStakingContracts is Script {
    // Settings
    address public l1Admin = address(0x243c63d5DBcF619ee36Fde7fF63D1564d5665b41); // Syndicate Gnosis Safe
    address public l2Admin = address(0x243c63d5DBcF619ee36Fde7fF63D1564d5665b41); // Syndicate Gnosis Safe
    address public l3Admin = address(0x03F8b8f48a3F22109bf1F4b54b54d0fdc96E7A67); // Ledger
    uint256 public decayFactor = 0.98e18;
    uint256 public startingEpoch = 3;

    // DEPLOYED ON ETHEREUM
    address public L1_STANDARD_BRIDGE = address(0x3154Cf16ccdb4C6d922629664174b904d80F2C35); // https://docs.base.org/base-chain/network-information/base-contracts#ethereum-mainnet
    address public L1_CROSS_DOMAIN_MESSENGER = address(0x866E82a600A1414e583f7F13623F1aC5d58b0Afa); // https://docs.base.org/base-chain/network-information/base-contracts#ethereum-mainnet
    address public L1_TOKEN = address(0x1bAB804803159aD84b8854581AA53AC72455614E); // https://etherscan.io/token/0x1bab804803159ad84b8854581aa53ac72455614e

    // DEPLOYED ON BASE
    address public ARB_INBOX = address(0xAE824E2d20F21B222932aFC6079cDaA1EB5b2F00); // https://www.notion.so/syndicateprotocol/Commons-26494475eae680588c63cecbbb1e212a?pvs=25
    address public L2_TOKEN = address(0x11dC28D01984079b7efE7763b533e6ed9E3722B9); // https://basescan.org/address/0x11dc28d01984079b7efe7763b533e6ed9e3722b9

    // Populate after deployment
    address public l2Relayer = address(0x5c1aD8136FF7C1bEF7fAc1AD09ccCdc40488119E);
    address public basePool = address(0x71cF8bf70Bb4f5ba8e4B4588bacB5ee108f3Ed10);
    address public refunder = address(0x9BE716F21428a254a2e4825cfa1d8A0893B9827B);

    function run() public {
        vm.startBroadcast();

        // deployL3Contracts();
        // deployL2Contracts();
        // deployL1Contracts();

        vm.stopBroadcast();
    }

    function deployL1Contracts() public {
        assert(block.chainid == 1);
        assert(l2Relayer != address(0));
        assert(basePool != address(0));
        assert(l1Admin != address(0));

        L1Relayer _l1Relayer =
            new L1Relayer(L1_STANDARD_BRIDGE, L1_CROSS_DOMAIN_MESSENGER, L1_TOKEN, L2_TOKEN, l2Relayer, l1Admin);
        console2.log("L1Relayer deployed to:", address(_l1Relayer));

        EmissionsCalculator _emissionsCalculator = new EmissionsCalculator(address(L1_TOKEN), msg.sender, l1Admin);
        console2.log("EmissionsCalculator deployed to:", address(_emissionsCalculator));

        EmissionsScheduler _emissionsScheduler = new EmissionsScheduler(
            startingEpoch, address(_emissionsCalculator), address(_l1Relayer), address(basePool), l1Admin, l1Admin
        );
        console2.log("EmissionsScheduler deployed to:", address(_emissionsScheduler));

        // L1Relayer deployed to: 0x96f93df52B769AD3E7f633E4fc68cb2Cc1E33686
        // EmissionsCalculator deployed to: 0x83D41A09D8A01dDa13e380f74032D4f2afc3545D
        // EmissionsScheduler deployed to: 0xff8eb6318A3960863004f18eDd36aD8C8C711b29

        // Initialize emissions calculator
        _emissionsCalculator.initializeEmissions(decayFactor);
        _emissionsCalculator.grantRole(_emissionsCalculator.EMISSIONS_ROLE(), address(_emissionsScheduler));
        console2.log("Emissions setup completed successfully");

        // Renounce default admin role
        _emissionsCalculator.grantRole(_emissionsCalculator.DEFAULT_ADMIN_ROLE(), l1Admin);
        _emissionsCalculator.renounceRole(_emissionsCalculator.DEFAULT_ADMIN_ROLE(), msg.sender);
        console2.log("EmissionsCalculator default admin transferred to:", l1Admin);
    }

    function deployL2Contracts() public {
        assert(block.chainid == 8453);
        assert(refunder != address(0));
        assert(l2Admin != address(0));

        L2Relayer _l2Relayer = new L2Relayer(ARB_INBOX, L2_TOKEN, refunder, l2Admin);
        console2.log("L2Relayer deployed to:", address(_l2Relayer));

        // L2Relayer deployed to: 0x5c1aD8136FF7C1bEF7fAc1AD09ccCdc40488119E
    }

    function deployL3Contracts() public {
        assert(block.chainid == 510003);
        assert(l3Admin != address(0));

        SyndStaking _syndStaking = new SyndStaking(l3Admin);
        console2.log("SyndStaking deployed to:", address(_syndStaking));

        BasePool _basePool = new BasePool(address(_syndStaking));
        console2.log("BasePool deployed to:", address(_basePool));

        Refunder _refunder = new Refunder(address(_basePool), address(_syndStaking), l3Admin);
        console2.log("Refunder deployed to:", address(_refunder));

        // SyndStaking deployed to: 0xF9637B60f27AF139FC46EAa655cFBbe4E731BCdF
        // BasePool deployed to: 0x71cF8bf70Bb4f5ba8e4B4588bacB5ee108f3Ed10
        // Refunder deployed to: 0x9BE716F21428a254a2e4825cfa1d8A0893B9827B
    }
}
