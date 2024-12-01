// SPDX-License-Identifier: UNLICENSED

import {Script} from "forge-std/Script.sol";

import {MetafillerStorage} from "../src/backfill/MetafillerStorage.sol";

// Example command below to run, --legacy & --skip-simulation were needed to avoid forge throwing errors reguarding gas
// forge script script/AllowlistMetafillerStorageAddresses.s.sol:AllowlistMetafillerStorageAddresses --rpc-url metabased_stratos --private-key $ETH_PRIVATE_KEY --legacy --skip-simulation --broadcast -vvv

contract AllowlistMetafillerStorageAddresses is Script {
    MetafillerStorage public metafillerStorage;

    function run() public {
        vm.startBroadcast();
        address metafillerStorageAddress = vm.envAddress("METAFILLER_STORAGE_ADDRESS");
        metafillerStorage = MetafillerStorage(metafillerStorageAddress);
        // TODO: Update the addresses to allowlist
        address[10] memory addressesToAllowlist = [
            0xDbDc73078b2475B512763adC970eab8E0F75f5CE,
            0x57c02EfBFFc478104780333FDDCfe35218C9D9a0,
            0x8F62F2531214153327Ec9f2Db62b074830e8616C,
            0x00360102dc301BDDe557BF21e64EfcB0110275b8,
            0x4Df6409E95E0e184f082F1e44D74FF959029aeA4,
            0xefFa0298cb5dC49998C88d69Df9766e476572156,
            0xad2F869C257a9ab092b410580b757ADc368fd08b,
            0x329bf499866259c85d180d9c0206dDF4f8EbFc5D,
            0x625C1c22597208C76aA039725850c456512E38a5,
            0x800fddcF36700F20c3c234d6aC51342Ed4bF3181
        ];

        for (uint256 i = 0; i < addressesToAllowlist.length; i++) {
            metafillerStorage.grantRole(metafillerStorage.MANAGER_ROLE(), addressesToAllowlist[i]);
        }

        vm.stopBroadcast();
    }
}
