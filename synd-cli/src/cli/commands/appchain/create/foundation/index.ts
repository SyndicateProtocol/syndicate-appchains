import type { Command } from "@/node_modules/@commander-js/extra-typings";
import {
	appchainCreateFoundationOptionsSchema,
	handleSchemaErrors,
} from "@/src/cli/schema";

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
		.requiredOption("--appchain-explorer-url <url>", "Appchain explorer URL")
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

			console.log("validatedOptions", validatedOptions);
		});
}
