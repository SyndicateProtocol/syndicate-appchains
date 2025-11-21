import { existsSync, mkdirSync, writeFileSync } from "node:fs"
import { dirname, resolve } from "node:path"
import type { ZodObject, ZodRawShape, ZodTypeAny } from "zod"
import { exitWithError } from "./print"

const CORE_CONTRACTS = {
  rollup: "0x0000000000000000000000000000000000000000",
  inbox: "0x0000000000000000000000000000000000000000",
  nativeToken: "0x0000000000000000000000000000000000000000",
  outbox: "0x0000000000000000000000000000000000000000",
  rollupEventInbox: "0x0000000000000000000000000000000000000000",
  challengeManager: "0x0000000000000000000000000000000000000000",
  adminProxy: "0x0000000000000000000000000000000000000000",
  sequencerInbox: "0x0000000000000000000000000000000000000000",
  bridge: "0x0000000000000000000000000000000000000000",
  upgradeExecutor: "0x0000000000000000000000000000000000000000",
  validatorUtils: "0x0000000000000000000000000000000000000000",
  validatorWalletCreator: "0x0000000000000000000000000000000000000000",
  deployedAtBlockNumber: 0
}

const CONFIG_TEMPLATES: Record<string, Record<string, unknown>> = {
  synd: {
    config: {
      arbConfigManager: "0x0000000000000000000000000000000000000000",
      arbChainConfig: "0x0000000000000000000000000000000000000000"
    },
    bridge: {
      chainInfo: {
        chainName: "",
        chainId: 0,
        chainOwner: "0x0000000000000000000000000000000000000000",
        minL2BaseFee: 0,
        parentChainId: 0,
        nativeToken: "0x0000000000000000000000000000000000000000",
        staker: "0x0000000000000000000000000000000000000000",
        batchPoster: "0x0000000000000000000000000000000000000000",
        networkFeeReceiver: "0x0000000000000000000000000000000000000000",
        infrastructureFeeCollector:
          "0x0000000000000000000000000000000000000000",
        explorerUrl: "",
        rpcUrl: ""
      },
      coreContracts: CORE_CONTRACTS,
      tokenBridgeContracts: {
        l2Contracts: {
          router: "0x0000000000000000000000000000000000000000",
          standardGateway: "0x0000000000000000000000000000000000000000",
          customGateway: "0x0000000000000000000000000000000000000000",
          wethGateway: "0x0000000000000000000000000000000000000000",
          weth: "0x0000000000000000000000000000000000000000",
          multicall: "0x0000000000000000000000000000000000000000",
          proxyAdmin: "0x0000000000000000000000000000000000000000"
        },
        l3Contracts: {
          router: "0x0000000000000000000000000000000000000000",
          standardGateway: "0x0000000000000000000000000000000000000000",
          customGateway: "0x0000000000000000000000000000000000000000",
          wethGateway: "0x0000000000000000000000000000000000000000",
          weth: "0x0000000000000000000000000000000000000000",
          proxyAdmin: "0x0000000000000000000000000000000000000000",
          beaconProxyFactory: "0x0000000000000000000000000000000000000000",
          upgradeExecutor: "0x0000000000000000000000000000000000000000",
          multicall: "0x0000000000000000000000000000000000000000"
        }
      }
    },
    sequencing: {
      syndicateSequencingChain: "0x0000000000000000000000000000000000000000",
      allowlistSequencingModule: "0x0000000000000000000000000000000000000000",
      requireAndModule: "0x0000000000000000000000000000000000000000",
      settlementBlockBeforeDeployment: "",
      deployedAtBlock: ""
    },
    withdrawals: {
      teeKeyManager: "0x0000000000000000000000000000000000000000",
      assertionPoster: "0x0000000000000000000000000000000000000000",
      teeModule: "0x0000000000000000000000000000000000000000",
      attestationDocVerifier: "0x0000000000000000000000000000000000000000"
    }
  },
  coreContracts: CORE_CONTRACTS
}

function camelToKebab(str: string): string {
  return str.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase()
}

function isOptional(schema: ZodTypeAny): boolean {
  // Check if the schema is optional by checking for innerType in _def
  // Optional schemas in Zod have an innerType property
  const def = schema as { _def?: { innerType?: unknown } }
  return def._def?.innerType !== undefined
}

export function createInitCommand(
  commandName: string,
  schema: ZodObject<ZodRawShape>
) {
  return async (options: { output?: string; force?: boolean }) => {
    const defaultPath = `options/${commandName}.json`
    const outputPath = options.output || defaultPath
    const absolutePath = resolve(outputPath)

    // Check if file already exists
    if (existsSync(absolutePath) && !options.force) {
      exitWithError(
        `Config file already exists at: ${outputPath}\n\nUse --force to overwrite or specify a different path with --output`
      )
    }

    // Ensure directory exists
    const dir = dirname(absolutePath)
    if (!existsSync(dir)) {
      mkdirSync(dir, { recursive: true })
    }

    const shape = schema.shape
    const example: Record<string, unknown> = {}

    for (const [key, fieldSchema] of Object.entries(shape)) {
      // Skip optional fields
      if (isOptional(fieldSchema as ZodTypeAny)) {
        continue
      }

      if (Object.keys(CONFIG_TEMPLATES).includes(key)) {
        example[key] = CONFIG_TEMPLATES[key]
      } else {
        example[key] = "UPDATE"
      }
    }

    // Convert to kebab-case and create clean JSON
    const kebabConfig: Record<string, unknown> = {}
    for (const [key, value] of Object.entries(example)) {
      const kebabKey = camelToKebab(key)
      kebabConfig[kebabKey] = value
    }

    // Write as standard JSON (no comments)
    const output = `${JSON.stringify(kebabConfig, null, 2)}\n`

    // Write the file
    writeFileSync(absolutePath, output, "utf-8")

    console.log(`✅ Created config file: ${outputPath}`)
    console.log(`\n📝 Next steps:`)
    console.log(`   1. Edit the file with your values`)
    console.log(
      `   2. Run: bun run synd-cli ${commandName.replace(/-/g, " ")} --config ${outputPath}`
    )
  }
}
