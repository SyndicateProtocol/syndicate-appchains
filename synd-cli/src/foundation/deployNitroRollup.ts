import { sleep } from "bun";
import { stringify } from "viem";
import type { DeployNitroRollupParams } from "../types";
import { generateBridgeConfig } from "../utils/generateBridgeConfig";
import { print } from "../utils/print";
import { createRollup } from "./createRollup";
import { getConfigAndCoreContracts } from "./getConfigAndCoreContracts";

export async function deployNitroRollup({
	chainId,
	chainName,
	ownerSettlementWalletClient,
	settlementPublicClient,
	appChainRpc,
	appChainExplorer,
	nativeTokenAddress,
	deployerSettlementWalletClient,
}: DeployNitroRollupParams) {
	const hash = await createRollup({
		chainId,
		nativeTokenAddress,
		deployerSettlementWalletClient,
		ownerSettlementWalletClient,
		settlementPublicClient,
	});
	print("🫷  Waiting for 10 seconds before fetching chain config...");
	await sleep(10000);

	const { chainConfig, coreContracts } = await getConfigAndCoreContracts({
		hash,
		settlementPublicClient,
	});

	const bridgeConfig = generateBridgeConfig({
		coreContracts,
		chainName,
		chainId,
		parentChainId: settlementPublicClient.chain.id,
		rollupOwnerAddress: ownerSettlementWalletClient.account.address,
		rpcUrl: appChainRpc,
		explorerUrl: appChainExplorer,
	});
	print("🔍  Bridge Config");
	print(stringify(bridgeConfig, null, 2));

	return {
		chainConfig,
		bridgeConfig,
		coreContracts,
	};
}
