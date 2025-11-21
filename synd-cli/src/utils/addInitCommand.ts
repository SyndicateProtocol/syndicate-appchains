import type { Command } from "@commander-js/extra-typings";
import type { ZodObject, ZodRawShape } from "zod";
import { createInitCommand } from "./initConfig";

// Helper to add an "init" subcommand to any command that has a schema
export function addInitSubcommand(
	command: Command,
	commandName: string,
	schema: ZodObject<ZodRawShape>,
) {
	command
		.command("init")
		.description("Generate an example config file")
		.option(
			"--output <path>",
			"Output path for config file",
			`options/${commandName}.json`,
		)
		.option("--force", "Overwrite existing file")
		.action(createInitCommand(commandName, schema));
}
