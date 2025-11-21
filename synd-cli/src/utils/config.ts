import { readFileSync } from "node:fs"
import { resolve } from "node:path"
import { exitWithError } from "./print"

/**
 * Load and parse a JSON config file
 */
export function loadConfigFile<T>(configPath: string): T {
  try {
    const absolutePath = resolve(configPath)
    const fileContent = readFileSync(absolutePath, "utf-8")
    return JSON.parse(fileContent) as T
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") {
      exitWithError(`Config file not found: ${configPath}`)
    }
    if (error instanceof SyntaxError) {
      exitWithError(`Invalid JSON in config file: ${configPath}\n${error.message}`)
    }
    exitWithError(`Failed to load config file: ${configPath}\n${error}`)
  }
}

/**
 * Merge config file values with CLI options
 * CLI options take precedence over config file values
 */
export function mergeConfigWithOptions<T extends Record<string, unknown>>(
  configFile: Partial<T> | undefined,
  cliOptions: Partial<T>
): Partial<T> {
  if (!configFile) {
    return cliOptions
  }

  // CLI options override config file values
  // Only use config file values if CLI option is undefined
  const merged: Partial<T> = { ...configFile }

  for (const key in cliOptions) {
    if (cliOptions[key] !== undefined) {
      merged[key] = cliOptions[key]
    }
  }

  return merged
}

/**
 * Convert kebab-case keys to camelCase for config files
 * This allows config files to use more readable kebab-case
 */
export function kebabToCamelCase(obj: Record<string, unknown>): Record<string, unknown> {
  const result: Record<string, unknown> = {}

  for (const key in obj) {
    const camelKey = key.replace(/-([a-z])/g, (_, letter) => letter.toUpperCase())
    result[camelKey] = obj[key]
  }

  return result
}
