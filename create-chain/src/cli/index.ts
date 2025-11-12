import { Command } from "@commander-js/extra-typings";
import { aliasCommand } from "./commands/alias";
import { configureL3Command } from "./commands/configureL3/index";

const program = new Command();

program
	.name("cli")
	.description("L3 Configuration CLI")
	.version("1.0.0");

// Register alias command
aliasCommand(program);

// Register configureL3 command
configureL3Command(program);

program.parse();
