import type { Command } from "@/node_modules/@commander-js/extra-typings";
import { print } from "@/src/utils/print";
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
