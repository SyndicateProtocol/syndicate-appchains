import type { Command } from "@/node_modules/@commander-js/extra-typings";
import { createFeaturesCommand } from "./features";
import { createFoundationCommand } from "./foundation";

export function createAppchainCommand(program: Command) {
	const createAppchainProgram = program
		.command("create")
		.description("Create a new appchain")
		.action(async () => {
			console.log("Creating appchain...");
		});
	createFoundationCommand(createAppchainProgram);
	createFeaturesCommand(createAppchainProgram);
}
