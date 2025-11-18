import { getPublicClient, getWalletClient } from "@/src/cli/helpers";
import {
	appchainCreateFoundationOptionsSchema,
	handleSchemaErrors,
} from "@/src/cli/schema";
import { foundation } from "@/src/foundation/foundation";
import {
	supportedEthereumChains,
	supportedSequencingChains,
	supportedSettlementChains,
} from "@/src/utils/constants";
import type { Command } from "@commander-js/extra-typings";
import type { Hex } from "viem";

export function createFoundationCommand(program: Command) {
	program
		.command("foundation")
		.description(
			"Create foundation for a new appchain. Nitro core, Syndicate sequencing, Arb Chain config ",
		)
		.requiredOption("--settlement-rpc <url>", "Parent chain RPC URL")
		.requiredOption("--sequencing-rpc <url>", "Sequencing chain RPC URL")
		.requiredOption("--ethereum-rpc <url>", "Ethereum chain RPC URL")
		.requiredOption("--appchain-rpc <url>", "Appchain RPC URL")
		.requiredOption("--appchain-explorer <url>", "Appchain explorer URL")
		.requiredOption("--id <number>", "Chain ID")
		.requiredOption("--name <string>", "Chain name")
		.requiredOption("--deployer-private-key <key>", "Deployer private key")
		.requiredOption("--owner-private-key <key>", "Owner private key")
		.option(
			"--native-token-address <address>",
			"Native token address (optional): defaults to ETH",
		)
		.option(
			"--core-contracts-created-at-hash <hash>",
			"Core contracts created at hash (optional): if provided, will skip deploying the nitro core contracts",
		)
		.action(async (options: Record<string, unknown>) => {
			const {
				data: validatedOptions,
				success,
				error,
			} = appchainCreateFoundationOptionsSchema.safeParse(options);

			if (!success) {
				return handleSchemaErrors(error);
			}

			const {
				settlementRpc,
				sequencingRpc,
				ethereumRpc,
				deployerPrivateKey,
				ownerPrivateKey,
			} = validatedOptions;

			const [
				settlementPublicClient,
				sequencingPublicClient,
				ethereumPublicClient,
				ownerSettlementWalletClient,
				deployerSettlementWalletClient,
				ownerSequencingWalletClient,
				deployerSequencingWalletClient,
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
				getWalletClient(
					sequencingRpc,
					supportedSequencingChains,
					ownerPrivateKey as `0x${string}`,
				),
				getWalletClient(
					sequencingRpc,
					supportedSequencingChains,
					deployerPrivateKey as `0x${string}`,
				),
			]);

			await foundation({
				deployerSettlementWalletClient,
				deployerSequencingWalletClient,
				ownerSettlementWalletClient,
				ownerSequencingWalletClient,
				settlementPublicClient,
				sequencingPublicClient,

				chainId: validatedOptions.id,
				chainName: validatedOptions.name.toLowerCase().replace(/\s+/g, "-"),
				nativeTokenAddress: validatedOptions.nativeTokenAddress,
				ethereumChainRpcUrl: ethereumPublicClient.chain.rpcUrls.default.http[0],
				ownerPrivateKey: validatedOptions.ownerPrivateKey as Hex,
				coreContractsCreatedAtHash:
					validatedOptions.coreContractsCreatedAtHash as Hex,
				appChainRpc: validatedOptions.appchainRpc,
				appChainExplorer: validatedOptions.appchainExplorer,
			});

			console.log("validatedOptions", validatedOptions);
		});
}
