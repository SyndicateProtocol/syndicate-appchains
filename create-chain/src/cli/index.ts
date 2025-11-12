import { aliasCommand } from "./commands/alias";
import { configureL3Command } from "./commands/configureL3/index";
import { parseArgs } from "./parser";
import {
	generateCommandHelp,
	generateGlobalHelp,
	generateSubcommandHelp,
	getCommand,
	registerCommand,
} from "./registry";
import { ArgParseError } from "./types";

// Register all commands
registerCommand(aliasCommand);
registerCommand(configureL3Command);

async function main() {
	const args = process.argv.slice(2);

	// Show global help only if no command provided or help is first argument
	if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
		console.log(generateGlobalHelp());
		process.exit(0);
	}

	const commandName = args[0];
	const commandArgs = args.slice(1);

	const command = getCommand(commandName);

	if (!command) {
		console.error(`❌ Unknown command: ${commandName}`);
		console.error(`\nRun 'bun cli --help' to see available commands`);
		process.exit(1);
	}

	// Handle command-level help
	if (commandArgs.includes("--help") || commandArgs.includes("-h")) {
		console.log(generateCommandHelp(command));
		process.exit(0);
	}

	try {
		// Handle commands with subcommands
		if (command.subcommands) {
			const subcommandName = commandArgs[0];

			if (!subcommandName) {
				console.error(`❌ Missing subcommand for ${commandName}`);
				console.log(`\n${generateCommandHelp(command)}`);
				process.exit(1);
			}

			const subcommand = command.subcommands.find(
				(sub) => sub.name === subcommandName,
			);

			if (!subcommand) {
				console.error(
					`❌ Unknown subcommand: ${commandName} ${subcommandName}`,
				);
				console.log(`\n${generateCommandHelp(command)}`);
				process.exit(1);
			}

			// Handle subcommand-level help
			const subcommandArgs = commandArgs.slice(1);
			if (subcommandArgs.includes("--help") || subcommandArgs.includes("-h")) {
				console.log(generateSubcommandHelp(commandName, subcommand));
				process.exit(0);
			}

			// Parse and execute subcommand
			const parsedArgs = parseArgs(subcommandArgs, subcommand.schema);
			await subcommand.handler(parsedArgs);
		}
		// Handle commands without subcommands
		else if (command.schema && command.handler) {
			const parsedArgs = parseArgs(commandArgs, command.schema);
			await command.handler(parsedArgs);
		} else {
			console.error(`❌ Command ${commandName} is not properly configured`);
			process.exit(1);
		}
	} catch (error) {
		if (error instanceof ArgParseError) {
			console.error(`❌ ${error.message}`);
			if (error.details) {
				console.error(`   ${error.details}`);
			}
			process.exit(1);
		}
		throw error;
	}
}

main()
	.then(() => process.exit(0))
	.catch((error) => {
		console.error("🚫", error);
		process.exit(1);
	});
