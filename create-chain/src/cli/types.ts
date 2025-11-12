import type { Address } from "viem";

/**
 * Supported argument types with their TypeScript equivalents
 */
export type ArgType = "string" | "number" | "address" | "bigint";

/**
 * Maps ArgType to actual TypeScript types
 */
export interface ArgTypeMap {
	string: string;
	number: number;
	address: Address;
	bigint: bigint;
}

/**
 * Definition for a single argument
 */
export interface ArgDefinition<T extends ArgType = ArgType> {
	/** The flag name (e.g., "--parent-rpc") */
	flag?: string;
	/** Short description of the argument */
	description: string;
	/** Whether this argument is required */
	required?: boolean;
	/** Default value if not provided */
	default?: ArgTypeMap[T];
	/** The type of the argument */
	type: T;
	/** Custom validation function */
	validate?: (value: ArgTypeMap[T]) => boolean | string;
	/** Transform function to convert raw string to typed value */
	transform?: (value: string) => ArgTypeMap[T];
}

/**
 * Positional argument definition (no flag, just position)
 */
export interface PositionalArgDefinition<T extends ArgType = ArgType>
	extends Omit<ArgDefinition<T>, "flag"> {
	/** Position index (0-based) */
	position: number;
	/** Display name for help text */
	name: string;
}

/**
 * Schema defining all arguments for a command
 */
export interface CommandSchema {
	/** Positional arguments */
	positional?: PositionalArgDefinition[];
	/** Named flag arguments */
	flags?: Record<string, ArgDefinition>;
}

/**
 * Parsed argument values matching the schema
 */
export type ParsedArgs<T extends CommandSchema> = {
	[K in keyof T["flags"]]: T["flags"][K] extends ArgDefinition<infer U>
		? T["flags"][K]["required"] extends true
			? ArgTypeMap[U]
			: ArgTypeMap[U] | undefined
		: never;
} & {
	[K in keyof T["positional"]]: T["positional"][K] extends PositionalArgDefinition<
		infer U
	>
		? ArgTypeMap[U]
		: never;
};

/**
 * Command handler function type
 */
export type CommandHandler<T extends CommandSchema = CommandSchema> = (
	args: ParsedArgs<T>,
) => Promise<void>;

/**
 * Subcommand definition
 */
export interface SubcommandDefinition<T extends CommandSchema = CommandSchema> {
	/** Name of the subcommand */
	name: string;
	/** Description for help text */
	description: string;
	/** Argument schema for this subcommand */
	schema: T;
	/** Handler function */
	handler: CommandHandler<T>;
	/** Examples for help text */
	examples?: string[];
}

/**
 * Full command definition
 */
export interface CommandDefinition<T extends CommandSchema = CommandSchema> {
	/** Name of the command */
	name: string;
	/** Description for help text */
	description: string;
	/** Argument schema (for commands without subcommands) */
	schema?: T;
	/** Handler function (for commands without subcommands) */
	handler?: CommandHandler<T>;
	/** Subcommands (for commands with subcommands) */
	subcommands?: SubcommandDefinition[];
	/** Examples for help text */
	examples?: string[];
	/** Custom help message generator (overrides default help generation) */
	customHelp?: () => string;
}

/**
 * Error thrown during argument parsing
 */
export class ArgParseError extends Error {
	constructor(
		message: string,
		public readonly details?: string,
	) {
		super(message);
		this.name = "ArgParseError";
	}
}
