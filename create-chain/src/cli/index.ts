import { Command } from "@commander-js/extra-typings";
import { aliasCommand } from "./commands/alias";
import { configureAppchainCommand } from "./commands/configureAppchain/index";

const program = new Command();

program.name("synd-cli").description("Syndicate Appchain CLI").version("1.0.0");

// Register alias command
aliasCommand(program);

// Register configureL3 command
configureAppchainCommand(program);

program.parse();
