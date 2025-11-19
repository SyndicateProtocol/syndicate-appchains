import type { Command } from "@commander-js/extra-typings";
import { createAssertionPosterCommand } from "./assertionPoster";
import { createFeaturesCommand } from "./features";
import { createFoundationCommand } from "./foundation";
import { createTeeModuleCommand } from "./teeModule";
import { createWithdrawalsContractsCommand } from "./withdrawals";

export function createAppchainCommand(program: Command) {
	const createAppchainProgram = program
		.command("create")
		.description("Create a new appchain")
		.action(async () => {
			console.log("Creating appchain...");
		});
	createFoundationCommand(createAppchainProgram);
	createFeaturesCommand(createAppchainProgram);
	createAssertionPosterCommand(createAppchainProgram);
	createTeeModuleCommand(createAppchainProgram);
	createWithdrawalsContractsCommand(createAppchainProgram);
}
