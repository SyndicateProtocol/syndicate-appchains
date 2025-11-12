import type { CommandDefinition } from "../../types";
import { setWasmMaxStackDepthSubcommand } from "./setWasmMaxStackDepth";

/**
 * Command definition for configureL3
 */
export const configureL3Command: CommandDefinition = {
	name: "configureL3",
	description:
		"Generate targets & calldata needed to configure syndicate appchains via its UpgradeExecutor on the parent chain",
	subcommands: [setWasmMaxStackDepthSubcommand],
};
