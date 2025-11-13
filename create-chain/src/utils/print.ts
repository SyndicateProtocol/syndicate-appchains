import { formatEther, formatGwei } from "viem";

const keyWidth = 30;

export function print(key: string, value?: string | number) {
	value ? console.log(`${key}:`.padEnd(keyWidth) + value) : console.log(key);
}

/**
 * Print an indented key-value pair (for nested/grouped information)
 */
export function printIndented(key: string, value: string | number, indent = 2) {
	const indentStr = " ".repeat(indent);
	console.log(`${indentStr}${key}:`.padEnd(keyWidth + indent) + value);
}

/**
 * Print a horizontal separator line
 */
export function printSeparator(char = "=", width = 80) {
	console.log(char.repeat(width));
}

/**
 * Print a section header with separator lines
 */
export function printSection(title: string, char = "=", width = 80) {
	printSeparator(char, width);
	console.log(`\n${title}\n`);
	printSeparator(char, width);
}

/**
 * Format a bigint value with its wei, and ETH representation
 */
export function formatWeiValue(
	label: string,
	weiValue: bigint,
	options?: { unit?: string },
) {
	const unit = options?.unit || "ETH";
	console.log(`${`${label}:`.padEnd(keyWidth)}${weiValue} wei`);
	console.log(`${"".padEnd(keyWidth)}${formatGwei(weiValue)} gwei`);
	console.log(`${"".padEnd(keyWidth)}${formatEther(weiValue)} ${unit}`);
}

/**
 * Print an error message and exit with code 1
 */
export function exitWithError(message: string, options?: { emoji?: boolean }) {
	const showEmoji = options?.emoji ?? true;
	const prefix = showEmoji ? "❌ " : "";
	console.error(`${prefix}${message}`);
	process.exit(1);
}
