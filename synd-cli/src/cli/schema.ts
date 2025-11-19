import { getAddress, zeroAddress } from "viem";
import { z } from "zod";
import { exitWithError } from "../utils/print";

export const ethAddressSchema = z
	.string()
	.refine(
		(val) => {
			try {
				getAddress(val);
				return true;
			} catch {
				return false;
			}
		},
		{
			message: "Must be a valid Ethereum address",
		},
	)
	.transform((val) => getAddress(val));

const positiveBigIntSchema = z
	.string()
	.refine(
		(val) => {
			if (!val) return true;
			try {
				const bigIntVal = BigInt(val);
				return bigIntVal > BigInt(0);
			} catch {
				return false;
			}
		},
		{ message: "Must be a positive integer" },
	)
	.transform((val) => (val ? BigInt(val) : undefined));

const privateKeySchema = (privateKeyName: string) =>
	z
		.string()
		.regex(/^0x[a-fA-F0-9]{64}$/, `Invalid ${privateKeyName} private key`);

const chainIdSchema = z.coerce
	.number()
	.int()
	.positive("Chain ID must be a positive integer");

const chainNameSchema = z
	.string()
	.min(1, "Chain name is required")
	.transform((val) => val.toLowerCase().replace(/\s+/g, "-"));

export const callArbOwnerOptionsSchema = z
	.object({
		settlementRpc: z.url("Invalid settlement chain RPC URL"),
		appchainRpc: z.url("Invalid appchain RPC URL").optional(),
		settlementUpgradeExecutor: ethAddressSchema,
		settlementInbox: ethAddressSchema,
		appchainUpgradeExecutor: ethAddressSchema,
		refundAddress: ethAddressSchema,
		gasLimit: positiveBigIntSchema.optional(),
		maxFeePerGas: positiveBigIntSchema.optional(),
	})
	.strict();

const chainInfo = z.object({
	chainName: z.string(),
	chainId: z.number(),
	chainOwner: z.string().transform((val) => getAddress(val)),
	minL2BaseFee: z.number(),
	parentChainId: z.number(),
	nativeToken: z.string().transform((val) => getAddress(val)),
	staker: z.string().optional(),
	batchPoster: z.string().optional(),
	networkFeeReceiver: z.string().optional(),
	infrastructureFeeCollector: z.string().optional(),
	explorerUrl: z.string().url(),
	rpcUrl: z.string().url(),
});

const l2Contracts = z.object({
	router: z.string().transform((val) => getAddress(val)),
	standardGateway: z.string().transform((val) => getAddress(val)),
	customGateway: z.string().transform((val) => getAddress(val)),
	wethGateway: z.string().transform((val) => getAddress(val)),
	weth: z.string().transform((val) => getAddress(val)),
	multicall: z.string().transform((val) => getAddress(val)),
	proxyAdmin: z.string().transform((val) => getAddress(val)),
});

const l3Contracts = z.object({
	router: z.string().transform((val) => getAddress(val)),
	standardGateway: z.string().transform((val) => getAddress(val)),
	customGateway: z.string().transform((val) => getAddress(val)),
	wethGateway: z.string().transform((val) => getAddress(val)),
	weth: z.string().transform((val) => getAddress(val)),
	proxyAdmin: z.string().transform((val) => getAddress(val)),
	beaconProxyFactory: z.string().transform((val) => getAddress(val)),
	upgradeExecutor: z.string().transform((val) => getAddress(val)),
	multicall: z.string().transform((val) => getAddress(val)),
});

const coreContracts = z.object({
	rollup: z.string().transform((val) => getAddress(val)),
	inbox: z.string().transform((val) => getAddress(val)),
	nativeToken: z.string().transform((val) => getAddress(val)),
	outbox: z.string().transform((val) => getAddress(val)),
	rollupEventInbox: z.string().transform((val) => getAddress(val)),
	challengeManager: z.string().transform((val) => getAddress(val)),
	adminProxy: z.string().transform((val) => getAddress(val)),
	sequencerInbox: z.string().transform((val) => getAddress(val)),
	bridge: z.string().transform((val) => getAddress(val)),
	upgradeExecutor: z.string().transform((val) => getAddress(val)),
	validatorUtils: z
		.string()
		.optional()
		.transform((val) => (val ? getAddress(val) : undefined)),
	validatorWalletCreator: z.string().transform((val) => getAddress(val)),
	deployedAtBlockNumber: z.number(),
});

const sequencing = z.object({
	syndicateSequencingChain: z.string().transform((val) => getAddress(val)),
	allowlistSequencingModule: z.string().transform((val) => getAddress(val)),
	requireAndModule: z.string().transform((val) => getAddress(val)),
	settlementBlockBeforeDeployment: z.string(),
	deployedAtBlock: z.string(),
});

const withdrawals = z.object({
	teeKeyManager: z.string().transform((val) => getAddress(val)),
	assertionPoster: z.string().transform((val) => getAddress(val)),
	teeModule: z.string().transform((val) => getAddress(val)),
	attestationDocVerifier: z.string().transform((val) => getAddress(val)),
});

const bridge = z.object({
	chainInfo,
	coreContracts,
	tokenBridgeContracts: z.object({
		l2Contracts,
		l3Contracts,
	}),
});

export const synd = z.object({
	config: z.object({
		arbConfigManager: z.string().transform((val) => getAddress(val)),
		arbChainConfig: z.string().transform((val) => getAddress(val)),
	}),
	bridge,
	sequencing,
	withdrawals,
});

export const handoffConfig = z.object({
	settlementChainRpcUrl: z.string().url(),
	sequencingChainRpcUrl: z.string().url(),
	appChainRpcUrl: z.string().url(),
	appChainExplorerUrl: z.string().url(),
	ownerPrivateKey: z.string(),
	newOwnerAddress: z.string().transform((val) => getAddress(val)),
	synd,
});

export const appchainCreateFoundationOptionsSchema = z
	.object({
		settlementRpc: z.url("Invalid settlement chain RPC URL"),
		sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
		ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
		appchainRpc: z.url("Invalid appchain RPC URL"),
		appchainExplorer: z.url("Invalid appchain explorer URL"),
		id: chainIdSchema,
		name: chainNameSchema,
		deployerPrivateKey: privateKeySchema("deployer"),
		ownerPrivateKey: privateKeySchema("owner"),
		nativeTokenAddress: ethAddressSchema.default(zeroAddress),
		coreContractsCreatedAtHash: z
			.string()
			.regex(/^0x[a-fA-F0-9]{64}$/, "Invalid transaction hash")
			.optional(),
	})
	.strict();

export const appchainCreateFeaturesOptionsSchema = z
	.object({
		ownerPrivateKey: privateKeySchema("owner"),
		deployerPrivateKey: privateKeySchema("deployer"),
		chainId: chainIdSchema,
		chainName: chainNameSchema,
		settlementRpc: z.url("Invalid settlement chain RPC URL"),
		sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
		syndForkSequencingRpc: z.url("Invalid synd fork sequencing chain RPC URL"),
		ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
		appchainRpc: z.url("Invalid appchain RPC URL"),
		appchainExplorer: z.url("Invalid appchain explorer URL"),
		sequencingContractAddress: ethAddressSchema,
		coreContracts,
	})
	.strict();

export const appchainCreateTeeModuleOptionsSchema = z
	.object({
		ownerPrivateKey: privateKeySchema("owner"),
		deployerPrivateKey: privateKeySchema("deployer"),
		settlementRpc: z.url("Invalid settlement chain RPC URL"),
		sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
		syndForkSequencingRpc: z.url("Invalid synd fork sequencing chain RPC URL"),
		ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
		appchainRpc: z.url("Invalid appchain RPC URL"),
		sequencingContractAddress: ethAddressSchema,
		rollup: ethAddressSchema,
		upgradeExecutor: ethAddressSchema,
		bridge: ethAddressSchema,
	})
	.strict();

export const appchainCheckTokenBridgeOptionsSchema = z
	.object({
		settlementRpc: z.url("Invalid settlement chain RPC URL"),
		appchainRpc: z.url("Invalid appchain RPC URL"),
		rollup: ethAddressSchema,
		createdAtHash: z
			.string()
			.regex(/^0x[a-fA-F0-9]{64}$/, "Invalid transaction hash"),
	})
	.strict();

export function handleSchemaErrors(errors: z.ZodError) {
	const err = errors.issues
		.map((err) => `  - ${err.path.join(".")}: ${err.message}`)
		.join("\n");
	return exitWithError(`Invalid options:\n${err}`);
}
