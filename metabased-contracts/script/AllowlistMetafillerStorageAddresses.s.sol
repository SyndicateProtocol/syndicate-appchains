pragma solidity 0.8.25;

// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Script} from "forge-std/Script.sol";

import {MetafillerStorage} from "../src/backfill/MetafillerStorage.sol";

contract AllowlistMetafillerStorageAddresses is Script {
    MetafillerStorage public metafillerStorage;

    function run() public {
        vm.startBroadcast();
        address metafillerStorageAddress = vm.envAddress("METAFILLER_STORAGE_ADDRESS");
        metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        // TODO: Update the addresses to allowlist
        address[10] memory addressesToAllowlist = [
            0x5f008726C9584810fB4C6e7B794e7EA5aC79C1C6,
            0x6a5462609FEBcd374Cb72385c2543E9cb833DA1c,
            0xc68DFaA38E19f4fEc8356C9eA9A0362256fdC37c,
            0x86662598A5477CEc9c0BB110B75A1C0589f8cf7B,
            0xDc2305Cce62D17c777C4752134a68b65F2a41f1d,
            0x046217776Ce34aFbb59859EBa0f5957E39c3DDF0,
            0x2909C162338500EC4D6c02944A0Eda66569a42Bd,
            0x3cE840dEA371b4849E250F0637994baD3393979d,
            0xFA9DCeC1C6103DD9206eA251EBb8A9F0a7E698bF,
            0x560bDee5c0392e9061e48b014222a3a12e487142
        ];

        for (uint256 i = 0; i < addressesToAllowlist.length; i++) {
            metafillerStorage.grantRole(metafillerStorage.MANAGER_ROLE(), addressesToAllowlist[i]);
        }

        vm.stopBroadcast();
    }
}
