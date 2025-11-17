import type { Command } from "@/node_modules/@commander-js/extra-typings";
import {
	appchainCreateFoundationOptionsSchema,
	handleSchemaErrors,
} from "@/src/cli/schema";

export function createFeaturesCommand(program: Command) {
	program
		.command("features")
		.description("Create features for a new appchain")
		.action(async () => {
			console.log("Creating features...");
		});
}
