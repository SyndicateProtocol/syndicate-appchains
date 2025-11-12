import type {
	ArgDefinition,
	CommandDefinition,
	CommandSchema,
	PositionalArgDefinition,
	SubcommandDefinition,
} from "./types";

/**
 * Global registry of all commands
 */
const commands = new Map<string, CommandDefinition<any>>();

/**
 * Register a command
 */
export function registerCommand<T extends CommandSchema>(
	command: CommandDefinition<T>,
): void {
	commands.set(command.name, command as CommandDefinition<any>);
}

/**
 * Get a registered command by name
 */
export function getCommand(name: string): CommandDefinition<any> | undefined {
	return commands.get(name);
}

/**
 * Get all registered commands
 */
export function getAllCommands(): CommandDefinition<any>[] {
	return Array.from(commands.values());
}

/**
 * Generate help text for a positional argument
 */
function formatPositionalArg(arg: PositionalArgDefinition): string {
	const name = arg.name.toUpperCase();
	return arg.required ? `<${name}>` : `[${name}]`;
}

/**
 * Generate help text for a flag argument
 */
function formatFlagArg(flagName: string, arg: ArgDefinition): string {
	const flag = arg.flag || `--${flagName}`;
	const valueName = flagName.toUpperCase().replace(/-/g, "_");
	const _suffix = arg.required ? "" : " (optional)";
	const _defaultValue =
		arg.default !== undefined ? `, default: ${arg.default}` : "";
	return `  ${flag} <${valueName}>`;
}

/**
 * Generate help text for a command
 */
export function generateCommandHelp(command: CommandDefinition): string {
	// Use custom help if provided
	if (command.customHelp) {
		return command.customHelp();
	}

	const lines: string[] = [];

	// Command header
	lines.push(`${command.name.toUpperCase()}`);
	lines.push(command.description);
	lines.push("");

	// Usage
	if (command.subcommands && command.subcommands.length > 0) {
		lines.push("USAGE:");
		lines.push(`  bun cli ${command.name} <SUBCOMMAND> [OPTIONS]`);
		lines.push("");
		lines.push("SUBCOMMANDS:");
		for (const sub of command.subcommands) {
			const positionals = sub.schema.positional
				? sub.schema.positional.map(formatPositionalArg).join(" ")
				: "";
			lines.push(
				`  ${sub.name} ${positionals}`.trimEnd().padEnd(30) +
					` ${sub.description}`,
			);
		}
		lines.push("");

		// Collect all unique flags from all subcommands
		const allFlags = new Map<
			string,
			{ def: ArgDefinition; subcommandNames: string[] }
		>();
		for (const sub of command.subcommands) {
			if (sub.schema.flags) {
				for (const [flagName, flagDef] of Object.entries(sub.schema.flags)) {
					// Type guard to ensure flagDef is an ArgDefinition
					if (!flagDef || typeof flagDef !== "object") continue;

					const typedFlagDef = flagDef as ArgDefinition;
					const flagKey = typedFlagDef.flag || `--${flagName}`;
					if (allFlags.has(flagKey)) {
						allFlags.get(flagKey)?.subcommandNames.push(sub.name);
					} else {
						allFlags.set(flagKey, {
							def: typedFlagDef,
							subcommandNames: [sub.name],
						});
					}
				}
			}
		}

		// Show all flags if any exist
		if (allFlags.size > 0) {
			lines.push("OPTIONS:");
			for (const [flagKey, { def, subcommandNames }] of allFlags) {
				const required = def.required ? " (required)" : " (optional)";
				const defaultValue =
					def.default !== undefined ? `, default: ${def.default}` : "";
				const usedBy =
					subcommandNames.length === command.subcommands.length
						? ""
						: ` [used by: ${subcommandNames.join(", ")}]`;
				lines.push(`  ${flagKey} <VALUE>`);
				lines.push(`    ${def.description}${required}${defaultValue}${usedBy}`);
			}
			lines.push("");
		}
	} else if (command.schema) {
		const positionals = command.schema.positional
			? command.schema.positional.map(formatPositionalArg).join(" ")
			: "";
		lines.push("USAGE:");
		lines.push(`  bun cli ${command.name} ${positionals} [OPTIONS]`.trimEnd());
		lines.push("");

		// Show flags for non-subcommand commands
		if (command.schema.flags && Object.keys(command.schema.flags).length > 0) {
			lines.push("OPTIONS:");
			for (const [flagName, flagDef] of Object.entries(command.schema.flags)) {
				const flagText = formatFlagArg(flagName, flagDef);
				const required = flagDef.required ? " (required)" : " (optional)";
				const defaultValue =
					flagDef.default !== undefined ? `, default: ${flagDef.default}` : "";
				lines.push(`${flagText}`);
				lines.push(`    ${flagDef.description}${required}${defaultValue}`);
			}
			lines.push("");
		}
	}

	// Examples
	if (command.examples && command.examples.length > 0) {
		lines.push("EXAMPLES:");
		for (const example of command.examples) {
			lines.push(`  ${example}`);
		}
		lines.push("");
	}

	return lines.join("\n");
}

/**
 * Generate help text for a subcommand
 */
export function generateSubcommandHelp(
	commandName: string,
	subcommand: SubcommandDefinition,
): string {
	const lines: string[] = [];

	// Subcommand header
	lines.push(`${commandName.toUpperCase()} ${subcommand.name.toUpperCase()}`);
	lines.push(subcommand.description);
	lines.push("");

	// Usage
	const positionals = subcommand.schema.positional
		? subcommand.schema.positional.map(formatPositionalArg).join(" ")
		: "";
	lines.push("USAGE:");
	lines.push(
		`  bun cli ${commandName} ${subcommand.name} ${positionals} [OPTIONS]`.trimEnd(),
	);
	lines.push("");

	// Options
	if (
		subcommand.schema.flags &&
		Object.keys(subcommand.schema.flags).length > 0
	) {
		lines.push("OPTIONS:");
		for (const [flagName, flagDef] of Object.entries(subcommand.schema.flags)) {
			const flagText = formatFlagArg(flagName, flagDef);
			const required = flagDef.required ? " (required)" : " (optional)";
			const defaultValue =
				flagDef.default !== undefined ? `, default: ${flagDef.default}` : "";
			lines.push(`${flagText}`);
			lines.push(`    ${flagDef.description}${required}${defaultValue}`);
		}
		lines.push("");
	}

	// Examples
	if (subcommand.examples && subcommand.examples.length > 0) {
		lines.push("EXAMPLES:");
		for (const example of subcommand.examples) {
			lines.push(`  ${example}`);
		}
		lines.push("");
	}

	return lines.join("\n");
}

/**
 * Generate global help text
 */
export function generateGlobalHelp(): string {
	const lines: string[] = [];

	lines.push("synd-cli: manage syndicate appchains");
	lines.push("");
	lines.push("USAGE:");
	lines.push("  bun cli <COMMAND> [OPTIONS]");
	lines.push("");
	lines.push("COMMANDS:");

	for (const command of getAllCommands()) {
		lines.push(`  ${command.name.padEnd(20)} ${command.description}`);
	}

	lines.push("");
	lines.push("OPTIONS:");
	lines.push("  --help, -h          Show this help message");
	lines.push("");
	lines.push(
		"Run 'bun cli <COMMAND> --help' for more information on a command.",
	);

	return lines.join("\n");
}
