import {
	appchainCreateTeeModuleOptionsSchema,
	handleSchemaErrors,
} from "@/cli/schema";
import {
	getAppchainClient,
	getPublicClient,
	getWalletClient,
} from "@/utils/clients";
import {
	supportedEthereumChains,
	supportedSequencingChains,
	supportedSettlementChains,
} from "@/utils/constants";
import type { Command } from "@commander-js/extra-typings";
import { type Hex, zeroAddress } from "viem";
import { getNativeTokenFromBridge } from "../arbOwner/helpers";
import { deployWithdrawals } from "./features/deployWithdrawals";

export function createWithdrawalsContractsCommand(program: Command) {
	program
		.command("withdrawals")
		.description("Deploy withdrawals contracts. AssertionPoster & TeeModule")
		.requiredOption(
			"--settlement-rpc <url>",
			"RPC URL for the settlement chain",
		)
		.requiredOption(
			"--sequencing-rpc <url>",
			"RPC URL for the sequencing chain",
		)
		.requiredOption(
			"--synd-fork-sequencing-rpc <url>",
			"RPC URL for the synd fork sequencing chain",
		)
		.requiredOption("--ethereum-rpc <url>", "RPC URL for Ethereum")
		.requiredOption("--appchain-rpc <url>", "RPC URL for the appchain")
		.requiredOption(
			"--owner-private-key <key>",
			"Private key of the owner account",
		)
		.requiredOption(
			"--deployer-private-key <key>",
			"Private key of the deployer account",
		)
		.requiredOption(
			"--sequencing-contract <address>",
			"Address of the sequencing contract",
		)
		.requiredOption("--rollup <address>", "Address of the rollup contract")
		.requiredOption(
			"--upgrade-executor <address>",
			"Address of the upgrade executor contract",
		)
		.requiredOption("--bridge <address>", "Address of the bridge contract")
		.action(async (options: Record<string, unknown>) => {
			const {
				data: validatedOptions,
				success,
				error,
			} = appchainCreateTeeModuleOptionsSchema.safeParse(options);

			if (!success) {
				return handleSchemaErrors(error);
			}

			const {
				settlementRpc,
				sequencingRpc,
				ethereumRpc,
				deployerPrivateKey,
				ownerPrivateKey,
				rollup,
				upgradeExecutor,
				bridge,
				appchainRpc,
				sequencingContract,
			} = validatedOptions;

			const [
				settlementPublicClient,
				sequencingPublicClient,
				ethereumPublicClient,
				ownerSettlementWalletClient,
				deployerSettlementWalletClient,
			] = await Promise.all([
				getPublicClient(settlementRpc, supportedSettlementChains),
				getPublicClient(sequencingRpc, supportedSequencingChains),
				getPublicClient(ethereumRpc, supportedEthereumChains),
				getWalletClient(
					settlementRpc,
					supportedSettlementChains,
					ownerPrivateKey as `0x${string}`,
				),
				getWalletClient(
					settlementRpc,
					supportedSettlementChains,
					deployerPrivateKey as `0x${string}`,
				),
			]);

			const customNativeToken = await getNativeTokenFromBridge(
				settlementPublicClient,
				bridge,
			);

			const appchainPublicClient = await getAppchainClient({
				nativeToken: customNativeToken?.address ?? zeroAddress,
				settlementPublicClient: settlementPublicClient,
				rpcUrl: appchainRpc,
			});

			await deployWithdrawals({
				...validatedOptions,
				coreContracts: {
					rollup,
					upgradeExecutor,
					bridge,
				},
				sequencingContractAddress: sequencingContract as Hex,
				settlementPublicClient,
				deployerSettlementWalletClient,
				ownerSettlementWalletClient,
				sequencingPublicClient,
				appchainPublicClient,
				ethereumPublicClient,
			});
		});
}
