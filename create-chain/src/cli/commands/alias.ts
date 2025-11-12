import { applyL1ToL2Alias } from "../../utils/alias";
import type { CommandDefinition, CommandSchema } from "../types";

const aliasSchema = {
	positional: [
		{
			position: 0,
			name: "address",
			description: "The address to alias",
			type: "address",
			required: true,
		},
	],
} as const satisfies CommandSchema;

export const aliasCommand: CommandDefinition<typeof aliasSchema> = {
	name: "alias",
	description: "Calculate the aliased address for L1->L2 messages",
	schema: aliasSchema,
	handler: async (args) => {
		const aliasedAddress = applyL1ToL2Alias(args.address);
		console.log(`Original:  ${args.address}`);
		console.log(`Aliased:   ${aliasedAddress}`);
	},
	examples: ["bun cli alias <ADDRESS>"],
};
