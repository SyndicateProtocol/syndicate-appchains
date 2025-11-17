import type { Command } from "@/node_modules/@commander-js/extra-typings";
import { callArbOwnerCommand } from "./call";
import { listArbOwnerCommand } from "./list";

export function arbOwnerCommand(program: Command) {
	const arbOwner = program
		.command("arbOwner")
		.description("Call ArbOwner functions through the UpgradeExecutor");

	listArbOwnerCommand(arbOwner);
	callArbOwnerCommand(arbOwner);
}
