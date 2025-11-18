import {
	type Chain,
	createPublicClient,
	createWalletClient,
	defineChain,
	type Hex,
	hexToNumber,
	http,
	type PrivateKeyAccount,
	type PublicClient,
	type Transport,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import {
	supportedEthereumChains,
	supportedSequencingChains,
	supportedSettlementChains,
} from "./constants";
import { getChainRpcUrl, getNativeCurrency } from "./helpers";
import { print } from "./print";
import { featuresConfig, foundationConfig, handoffConfig } from "./schema";

export async function getFoundationConfig() {
	const configFile = Bun.file("./foundation.config.json");
	if (!(await configFile.exists())) {
		print("🚫 foundation.config.json not found");
		process.exit(1);
	}

	const contents = await configFile.json();
	const { success, data: config, error } = foundationConfig.safeParse(contents);
	if (!success) {
		print("🚫 Invalid foundation.config.json");
		console.error(error);
		throw new Error("Invalid foundation.config.json");
	}

	const { chainName, ...rest } = config;
	const [
		settlementPublicClient,
		sequencingPublicClient,
		ethereumPublicClient,
		ownerSettlementWalletClient,
		deployerSettlementWalletClient,
		ownerSequencingWalletClient,
		deployerSequencingWalletClient,
	] = await Promise.all([
		getSettlementClient(config.settlementChainRpcUrl),
		getSequencingClient(config.sequencingChainRpcUrl),
		getEthereumClient(config.ethereumChainRpcUrl),
		getSettlementWalletClient(
			config.settlementChainRpcUrl,
			config.ownerPrivateKey,
		),
		getSettlementWalletClient(
			config.settlementChainRpcUrl,
			config.deployerPrivateKey,
		),
		getSequencingWalletClient(
			config.sequencingChainRpcUrl,
			config.ownerPrivateKey,
		),
		getSequencingWalletClient(
			config.sequencingChainRpcUrl,
			config.deployerPrivateKey,
		),
	]);

	return {
		...rest,
		settlementPublicClient,
		sequencingPublicClient,
		ethereumPublicClient,
		ownerSettlementWalletClient,
		deployerSettlementWalletClient,
		ownerSequencingWalletClient,
		deployerSequencingWalletClient,
		chainName: chainName.toLowerCase().replace(/\s+/g, "-"), // Normalize chain name as it will appear in the nitro and maestro configs
	};
}

export async function getFeaturesConfig() {
	const featuresConfigFile = Bun.file("./features.config.json");

	if (!(await featuresConfigFile.exists())) {
		print("🚫 features.config.json not found");
		process.exit(1);
	}

	const contents = await featuresConfigFile.json();
	// console.log("contents", contents);
	const { success, data: config, error } = featuresConfig.safeParse(contents);
	if (!success) {
		print("🚫 Invalid features.config.json");
		console.error(error);
		throw new Error("Invalid features.config.json");
	}

	// console.log("getting clients");

	const [
		settlementPublicClient,
		sequencingPublicClient,
		ethereumPublicClient,
		ownerSettlementWalletClient,
		deployerSettlementWalletClient,
		ownerSequencingWalletClient,
		deployerSequencingWalletClient,
	] = await Promise.all([
		getSettlementClient(config.settlementChainRpcUrl),
		getSequencingClient(config.sequencingChainRpcUrl),
		getEthereumClient(config.ethereumChainRpcUrl),
		getSettlementWalletClient(
			config.settlementChainRpcUrl,
			config.ownerPrivateKey,
		),
		getSettlementWalletClient(
			config.settlementChainRpcUrl,
			config.deployerPrivateKey,
		),
		getSequencingWalletClient(
			config.sequencingChainRpcUrl,
			config.ownerPrivateKey,
		),
		getSequencingWalletClient(
			config.sequencingChainRpcUrl,
			config.deployerPrivateKey,
		),
	]);

	// console.log("got clients");

	const appchainPublicClient = await getAppchainClient({
		chainId: config.chainId,
		chainName: config.chainName,
		nativeToken: config.coreContracts.nativeToken,
		settlementPublicClient: settlementPublicClient,
		rpcUrl: config.appChainRpcUrl,
		explorerUrl: config.appChainExplorerUrl,
	});

	const ownerAppchainWalletClient = createWalletClient({
		chain: appchainPublicClient.chain,
		account: privateKeyToAccount(config.ownerPrivateKey as Hex),
		transport: http(getChainRpcUrl(appchainPublicClient.chain)),
	});

	const deployerAppchainWalletClient = createWalletClient({
		chain: appchainPublicClient.chain,
		account: privateKeyToAccount(config.deployerPrivateKey as Hex),
		transport: http(getChainRpcUrl(appchainPublicClient.chain)),
	});

	// console.log("got config");

	return {
		...config,
		settlementPublicClient,
		sequencingPublicClient,
		ethereumPublicClient,
		ownerSettlementWalletClient,
		deployerSettlementWalletClient,
		ownerSequencingWalletClient,
		deployerSequencingWalletClient,
		appchainPublicClient,
		ownerAppchainWalletClient,
		deployerAppchainWalletClient,
	};
}

export async function getHandoffConfig() {
	const handoffConfigFile = Bun.file("./handoff.config.json");
	if (!(await handoffConfigFile.exists())) {
		print("🚫 handoff.config.json not found");
		process.exit(1);
	}

	const contents = await handoffConfigFile.json();
	const { success, data: config, error } = handoffConfig.safeParse(contents);
	if (!success) {
		print("🚫 Invalid handoff.config.json");
		console.error(error);
		throw new Error("Invalid handoff.config.json");
	}
	// console.log("config", config);

	const [settlementPublicClient, sequencingPublicClient] = await Promise.all([
		getSettlementClient(config.settlementChainRpcUrl),
		getSequencingClient(config.sequencingChainRpcUrl),
	]);

	const [ownerSettlementWalletClient, ownerSequencingWalletClient] =
		await Promise.all([
			getSettlementWalletClient(
				config.settlementChainRpcUrl,
				config.ownerPrivateKey,
			),
			getSequencingWalletClient(
				config.sequencingChainRpcUrl,
				config.ownerPrivateKey,
			),
		]);

	const appchainPublicClient = await getAppchainClient({
		chainId: config.synd.bridge.chainInfo.chainId,
		chainName: config.synd.bridge.chainInfo.chainName,
		nativeToken: config.synd.bridge.coreContracts.nativeToken,
		rpcUrl: config.appChainRpcUrl,
		explorerUrl: config.appChainExplorerUrl,
		settlementPublicClient: settlementPublicClient,
	});
	const ownerAppchainWalletClient = createWalletClient({
		chain: appchainPublicClient.chain,
		account: privateKeyToAccount(config.ownerPrivateKey as Hex),
		transport: http(getChainRpcUrl(appchainPublicClient.chain)),
	});

	return {
		newOwnerAddress: config.newOwnerAddress,
		settlementPublicClient,
		sequencingPublicClient,
		ownerSettlementWalletClient,
		ownerSequencingWalletClient,
		appchainPublicClient,
		ownerAppchainWalletClient,
		synd: config.synd,
	};
}

async function getSettlementClient(rpcUrl: string) {
	const chain = await getChainFromRpcUrl(rpcUrl, supportedSettlementChains);
	// console.log("chain", chain);
	return createPublicClient<Transport, Chain>({
		chain,
		transport: http(rpcUrl),
	});
}

async function getSettlementWalletClient(rpcUrl: string, privateKey: string) {
	const chain = await getChainFromRpcUrl(rpcUrl, supportedSettlementChains);
	return createWalletClient<Transport, Chain, PrivateKeyAccount>({
		chain,
		account: privateKeyToAccount(privateKey as Hex),
		transport: http(rpcUrl),
	});
}

async function getSequencingClient(rpcUrl: string) {
	const chain = await getChainFromRpcUrl(rpcUrl, supportedSequencingChains);
	return createPublicClient<Transport, Chain>({
		chain,
		transport: http(rpcUrl),
	});
}

async function getSequencingWalletClient(rpcUrl: string, privateKey: string) {
	const chain = await getChainFromRpcUrl(rpcUrl, supportedSequencingChains);
	return createWalletClient<Transport, Chain, PrivateKeyAccount>({
		chain,
		account: privateKeyToAccount(privateKey as Hex),
		transport: http(rpcUrl),
	});
}

async function getEthereumClient(rpcUrl: string) {
	const chain = await getChainFromRpcUrl(rpcUrl, supportedEthereumChains);
	return createPublicClient<Transport, Chain>({
		chain,
		transport: http(rpcUrl),
	});
}

async function getAppchainClient({
	chainId,
	chainName,
	nativeToken,
	rpcUrl,
	explorerUrl,
	settlementPublicClient,
}: {
	chainId: number;
	chainName: string;
	nativeToken: Hex;
	rpcUrl: string;
	explorerUrl: string;
	settlementPublicClient: PublicClient<Transport, Chain>;
}) {
	const nativeCurrency = await getNativeCurrency(
		settlementPublicClient,
		nativeToken,
	);
	return createPublicClient({
		chain: defineChain({
			id: chainId,
			name: chainName,
			network: chainName,
			nativeCurrency: nativeCurrency ?? {
				decimals: 18,
				name: "Ether",
				symbol: "ETH",
			},
			rpcUrls: {
				default: { http: [rpcUrl] },
				public: { http: [rpcUrl] },
			},
			blockExplorers: {
				default: {
					name: `${chainName} Explorer`,
					url: explorerUrl,
				},
			},
		}),
		transport: http(rpcUrl),
	});
}

async function getChainFromRpcUrl<T extends Record<string, { chain: Chain }>>(
	rpcUrl: string,
	supportedChains: T,
): Promise<Chain> {
	const chainId = await getRpcUrlChainId(rpcUrl);
	// console.log("chainId", chainId);

	const chainIdStr = chainId.toString();
	if (chainIdStr in supportedChains) {
		return supportedChains[chainIdStr].chain;
	}

	throw new Error(
		`Could not resolve chain for RPC URL: ${rpcUrl} (chainId: ${chainId})`,
	);
}

async function getRpcUrlChainId(rpcUrl: string) {
	if (rpcUrl === "http://localhost:8545") {
		return 84532;
	}
	const res = await fetch(rpcUrl, {
		method: "POST",
		body: JSON.stringify({
			jsonrpc: "2.0",
			method: "eth_chainId",
			params: [],
			id: 1,
		}),
	});
	if (!res.ok) {
		throw new Error(`Failed to get chainId for ${rpcUrl}`);
	}
	const chainId = await res.json();
	// console.log("chainId", chainId, rpcUrl);
	return hexToNumber(chainId.result);
}
