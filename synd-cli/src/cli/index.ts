import { Command } from "@commander-js/extra-typings";
import { aliasCommand } from "./commands/alias";
import { appchainCommand } from "./commands/appchain";

const program = new Command();

program.name("synd-cli").description("Syndicate Appchain CLI").version("1.0.0");

aliasCommand(program);
appchainCommand(program);

program.parse();
