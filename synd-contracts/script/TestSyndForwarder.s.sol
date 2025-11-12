// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {SyndForwarder} from "src/deployment/SyndForwarder.sol";

interface IOptimismPortal {
    function depositTransaction(address _to, uint256 _value, uint64 _gasLimit, bool _isCreation, bytes memory _data)
        external;
}

contract TestSyndForwarder is Script {
    address public stub = address(0x920487DB398Be410Eda65D1A33e826A0453C0814);
    address public optimismPortal = address(0x49f53e41452C74589E85cA1677426Ba426459e85);

    uint256 public gasLimit = 500000;

    SyndForwarder public forwarder = SyndForwarder(0x2E38D75B496B201532580D96c848786aE9692368);

    function run() public {
        vm.startBroadcast();

        // forwarder.deploy(bytes32(uint256(1)), stub, "");
        forwarder.call(
            address(optimismPortal),
            abi.encodeWithSelector(
                IOptimismPortal.depositTransaction.selector,
                address(forwarder),
                0,
                gasLimit,
                false,
                abi.encodeWithSelector(SyndForwarder.deploy.selector, bytes32(uint256(1)), stub, "")
            )
        );

        vm.stopBroadcast();
    }
}
