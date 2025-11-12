import type { Address } from "viem";
import {
	ArgParseError,
	type ArgDefinition,
	type ArgType,
	type ArgTypeMap,
	type CommandSchema,
	type ParsedArgs,
	type PositionalArgDefinition,
} from "./types";

/**
 * Default transformers for each argument type
 */
const DEFAULT_TRANSFORMERS: Record<
	ArgType,
	(value: string) => ArgTypeMap[ArgType]
> = {
	string: (value: string) => value,
	number: (value: string) => {
		const num = Number(value);
		if (Number.isNaN(num)) {
			throw new ArgParseError(`Invalid number: ${value}`);
		}
		return num;
	},
	address: (value: string) => {
		if (!/^0x[a-fA-F0-9]{40}$/.test(value)) {
			throw new ArgParseError(`Invalid address: ${value}`);
		}
		return value as Address;
	},
	bigint: (value: string) => {
		try {
			return BigInt(value);
		} catch {
			throw new ArgParseError(`Invalid bigint: ${value}`);
		}
	},
};

/**
 * Parse a single argument value according to its definition
 */
function parseArgValue<T extends ArgType>(
	rawValue: string | undefined,
	definition: ArgDefinition<T> | PositionalArgDefinition<T>,
	argName: string,
): ArgTypeMap[T] | undefined {
	// Handle missing value
	if (rawValue === undefined) {
		if (definition.required && definition.default === undefined) {
			throw new ArgParseError(`Missing required argument: ${argName}`);
		}
		return definition.default;
	}

	// Transform the value
	const transformer =
		definition.transform || DEFAULT_TRANSFORMERS[definition.type];
	let value: ArgTypeMap[T];
	try {
		value = transformer(rawValue) as ArgTypeMap[T];
	} catch (error) {
		if (error instanceof ArgParseError) {
			throw error;
		}
		throw new ArgParseError(
			`Failed to parse ${argName}`,
			error instanceof Error ? error.message : String(error),
		);
	}

	// Validate the value
	if (definition.validate) {
		const validationResult = definition.validate(value);
		if (validationResult !== true) {
			const message =
				typeof validationResult === "string"
					? validationResult
					: `Invalid value for ${argName}`;
			throw new ArgParseError(message);
		}
	}

	return value;
}

/**
 * Parse command-line arguments according to a schema
 */
export function parseArgs<T extends CommandSchema>(
	args: string[],
	schema: T,
): ParsedArgs<T> {
	const result: Record<string, unknown> = {};
	const usedIndices = new Set<number>();

	// Parse positional arguments
	if (schema.positional) {
		for (const positionalDef of schema.positional) {
			const rawValue = args[positionalDef.position];

			// Don't consume values that look like flags
			if (rawValue?.startsWith("--")) {
				if (positionalDef.required) {
					throw new ArgParseError(
						`Missing required positional argument: ${positionalDef.name}`,
					);
				}
				continue;
			}

			const value = parseArgValue(
				rawValue,
				positionalDef,
				positionalDef.name,
			);
			result[positionalDef.name] = value;
			usedIndices.add(positionalDef.position);
		}
	}

	// Parse flag arguments
	if (schema.flags) {
		for (const [flagName, flagDef] of Object.entries(schema.flags)) {
			if (!flagDef.flag) {
				throw new Error(`Flag definition for ${flagName} is missing flag name`);
			}

			const flagIndex = args.indexOf(flagDef.flag);
			const rawValue =
				flagIndex !== -1 && args[flagIndex + 1] ? args[flagIndex + 1] : undefined;

			const value = parseArgValue(rawValue, flagDef, flagDef.flag);
			result[flagName] = value;

			if (flagIndex !== -1) {
				usedIndices.add(flagIndex);
				if (rawValue !== undefined) {
					usedIndices.add(flagIndex + 1);
				}
			}
		}
	}

	return result as ParsedArgs<T>;
}

/**
 * Extract the raw value for a flag from arguments
 */
export function getRawFlag(args: string[], flag: string): string | undefined {
	const index = args.indexOf(flag);
	return index !== -1 && args[index + 1] ? args[index + 1] : undefined;
}
