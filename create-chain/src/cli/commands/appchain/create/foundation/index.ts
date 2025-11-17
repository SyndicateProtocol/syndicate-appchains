import type { Command } from "@commander-js/extra-typings";

export function createFoundationCommand(program: Command) {
	program
		.command("foundation")
		.description("Create foundation for a new appchain")
		.action(async () => {
			console.log("Creating foundation...");
		});
}
