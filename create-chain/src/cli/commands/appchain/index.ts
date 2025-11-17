import type { Command } from "@commander-js/extra-typings";
import { arbOwnerCommand } from "./arbOwner";
import { createAppchainCommand } from "./create";

export function appchainCommand(program: Command) {
	const appchainProgram = program
		.command("appchain")
		.description("Manage appchains")
		.action(async () => {
			console.log("Managing appchains...");
		});
	arbOwnerCommand(appchainProgram);
	createAppchainCommand(appchainProgram);
}
