import { getAppchainClient, getPublicClient } from "@/utils/clients";
import { supportedSettlementChains } from "@/utils/constants";
import type { Command } from "@commander-js/extra-typings";
import {
	appchainCheckTokenBridgeOptionsSchema,
	handleSchemaErrors,
} from "../../../schema";
import { checkTokenBridge } from "./checkTokenBridge";

export function checkTokenBridgeCommand(program: Command) {
	program
		.command("checkTokenBridge")
		.description("Check token bridge deployment and retryable ticket execution")
		.requiredOption(
			"--settlement-rpc <url>",
			"RPC URL for the settlement chain",
		)
		.requiredOption("--appchain-rpc <url>", "RPC URL for the appchain")
		.requiredOption("--rollup <address>", "Address of the rollup contract")
		.requiredOption(
			"--created-at-hash <hash>",
			"Transaction hash where token bridge was created",
		)
		.action(async (options: Record<string, unknown>) => {
			const {
				data: validatedOptions,
				success,
				error,
			} = appchainCheckTokenBridgeOptionsSchema.safeParse(options);

			if (!success) {
				return handleSchemaErrors(error);
			}

			const { settlementRpc, appchainRpc, rollup, createdAtHash } =
				validatedOptions;

			const settlementPublicClient = await getPublicClient(
				settlementRpc,
				supportedSettlementChains,
			);

			const appchainPublicClient = await getAppchainClient({
				settlementPublicClient: settlementPublicClient,
				rpcUrl: appchainRpc,
			});

			await checkTokenBridge({
				rollup,
				appchainPublicClient,
				settlementPublicClient,
				tokenBridgeCreatedAtHash: createdAtHash as `0x${string}`,
			});
		});
}
