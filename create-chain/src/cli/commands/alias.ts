import type { CommandDefinition } from "../types";
import { applyL1ToL2Alias } from "../../utils/alias";

/**
 * Command definition for the alias command
 */
export const aliasCommand: CommandDefinition = {
	name: "alias",
	description: "Calculate the aliased address for L1->L2 messages",
	schema: {
		positional: [
			{
				position: 0,
				name: "address",
				description: "The address to alias",
				type: "address",
				required: true,
			},
		],
	},
	handler: async (args) => {
		const aliasedAddress = applyL1ToL2Alias(args.address);
		console.log(`Original:  ${args.address}`);
		console.log(`Aliased:   ${aliasedAddress}`);
	},
	examples: [
		"bun cli alias 0x1234567890123456789012345678901234567890",
	],
};
