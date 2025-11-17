import { print } from "@/src/utils/print";
import type { Command } from "@commander-js/extra-typings";
import {
	formatFunctionSignatureForDisplay,
	getWriteFunctions,
} from "../helpers";

export function listArbOwnerCommand(program: Command) {
	program
		.command("list")
		.description("List all available ArbOwner write functions")
		.action(() => {
			const functions = getWriteFunctions();
			print("\nAvailable ArbOwner write functions:\n");
			for (const fn of functions) {
				print(`  ${formatFunctionSignatureForDisplay(fn)}`);
			}
		});
}
