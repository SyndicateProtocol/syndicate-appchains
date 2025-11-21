import type { synd } from "@/cli/schema"
import fs from "node:fs/promises"
import path from "node:path"
import { stringify } from "viem"
import type z from "zod"

const outputDirName = "appchains"

export async function writeToFile(
  chainName: string,
  fileName: string,
  data: string
) {
  const basePath = path.join(__dirname, "../../", outputDirName)

  // If chainName is provided, create a subdirectory
  const targetPath = chainName
    ? path.join(basePath, chainName, fileName)
    : path.join(basePath, fileName)

  if (chainName) {
    // Ensure the directory exists
    const chainDir = path.join(basePath, chainName)
    await fs.mkdir(chainDir, { recursive: true })
  }

  return Bun.write(targetPath, data)
}
export async function upsertToSyndObject(
  chainName: string,
  environment: string,
  objectPath: SyndObjectPaths,
  data: unknown
) {
  const basePath = path.join(__dirname, "../../", outputDirName)
  const chainDir = path.join(basePath, chainName)
  const syndFilePath = path.join(
    chainDir,
    `${environment}.synd.${chainName}.json`
  )

  // Ensure the directory exists
  await fs.mkdir(chainDir, { recursive: true })

  let syndObject: Record<string, unknown> = {}

  // Try to read existing synd file
  try {
    const existingData = await fs.readFile(syndFilePath, "utf-8")
    syndObject = JSON.parse(existingData)
  } catch (_error) {
    // File doesn't exist or is invalid, start with empty object
  }

  // Set the value at the specified path
  const pathParts = objectPath.split(".")
  let current: Record<string, unknown> = syndObject

  for (let i = 0; i < pathParts.length - 1; i++) {
    const part = pathParts[i]
    if (
      part === "__proto__" ||
      part === "constructor" ||
      part === "prototype"
    ) {
      throw new Error(
        `Prototype pollution property blocked in objectPath: '${part}'`
      )
    }
    if (!current[part]) {
      current[part] = {}
    }
    current = current[part] as Record<string, unknown>
  }

  const lastPart = pathParts[pathParts.length - 1]
  if (
    lastPart === "__proto__" ||
    lastPart === "constructor" ||
    lastPart === "prototype"
  ) {
    throw new Error(
      `Prototype pollution property blocked in objectPath: '${lastPart}'`
    )
  }
  current[lastPart] = data

  // Write the updated object back to file
  return Bun.write(syndFilePath, stringify(syndObject, null, 2))
}
export async function readEoaSecrets(
  chainName: string,
  environment: string
): Promise<Record<string, { address: string; privateKey: string }>> {
  const basePath = path.join(__dirname, "../../", outputDirName)
  const chainDir = path.join(basePath, chainName)
  const eoaSecretsPath = path.join(
    chainDir,
    `${environment}-eoaSecrets.${chainName}.json`
  )

  try {
    const existingData = await fs.readFile(eoaSecretsPath, "utf-8")
    return JSON.parse(existingData)
  } catch (_error) {
    // File doesn't exist or is invalid, return empty object
    return {}
  }
}
export async function upsertToEoaSecrets(
  chainName: string,
  environment: string,
  role: string,
  eoaData: { address: string; privateKey: string }
) {
  const eoaSecrets = await readEoaSecrets(chainName, environment)
  eoaSecrets[role] = eoaData

  return writeToFile(
    chainName,
    `${environment}-eoaSecrets.${chainName}.json`,
    JSON.stringify(eoaSecrets, null, 2)
  )
}

// Type utilities for generating valid synd object paths
type PathImpl<T, Key extends keyof T> = Key extends string
  ? T[Key] extends Record<string, unknown>
    ?
        | `${Key}.${PathImpl<T[Key], Exclude<keyof T[Key], keyof unknown[]>> & string}`
        | `${Key}.${Exclude<keyof T[Key], keyof unknown[]> & string}`
    : never
  : never

type PathImpl2<T> = PathImpl<T, keyof T> | keyof T

export type SyndPath<T> = PathImpl2<T> extends string | keyof T
  ? PathImpl2<T>
  : keyof T

export type SyndObjectPaths = SyndPath<z.infer<typeof synd>>
