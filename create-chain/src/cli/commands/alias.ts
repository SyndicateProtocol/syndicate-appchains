import type { Command } from "@commander-js/extra-typings";
import { getAddress } from "viem";
import { applyL1ToL2Alias } from "../../utils/alias";
import { exitWithError, print } from "../../utils/print";

/**
 * Register the alias command
 */
export function aliasCommand(program: Command) {
	program
		.command("alias")
		.description("Calculate the aliased address for L1->L2 messages")
		.argument("<address>", "The address to alias")
		.action((address: string) => {
			try {
				const parsedAddress = getAddress(address);
				const aliasedAddress = applyL1ToL2Alias(parsedAddress);
				print("Original", address);
				print("Aliased", aliasedAddress);
			} catch (error) {
				exitWithError(String(error));
			}
		});
}
