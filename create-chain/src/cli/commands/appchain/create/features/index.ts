import type { Command } from "@commander-js/extra-typings";

export function createFeaturesCommand(program: Command) {
	program
		.command("features")
		.description("Create features for a new appchain")
		.action(async () => {
			console.log("Creating features...");
		});
}
