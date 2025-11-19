import type { PublicClientWithChain } from "@/types";
import {
	type Chain,
	createPublicClient,
	createWalletClient,
	defineChain,
	type Hex,
	hexToNumber,
	http,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { getNativeCurrency, isNativeTokenEth } from "./helpers";

export async function getPublicClient(
	rpcUrl: string,
	supportedChains: Record<string, { chain: Chain }>,
) {
	const chain = await getChainFromRpcUrl(rpcUrl, supportedChains);
	return createPublicClient({
		chain,
		transport: http(rpcUrl),
	});
}

export async function getWalletClient(
	rpcUrl: string,
	supportedChains: Record<string, { chain: Chain }>,
	privateKey: Hex,
) {
	const chain = await getChainFromRpcUrl(rpcUrl, supportedChains);
	return createWalletClient({
		chain,
		account: privateKeyToAccount(privateKey),
		transport: http(rpcUrl),
	});
}

export async function getAppchainClient({
	chainName,
	nativeToken,
	rpcUrl,
	explorerUrl,
	settlementPublicClient,
}: {
	chainName?: string;
	nativeToken?: Hex;
	rpcUrl: string;
	explorerUrl?: string;
	settlementPublicClient: PublicClientWithChain;
}) {
	const chainId = await getRpcUrlChainId(rpcUrl);
	const nativeCurrency =
		nativeToken && !isNativeTokenEth(nativeToken)
			? await getNativeCurrency(settlementPublicClient, nativeToken)
			: {
					decimals: 18,
					name: "Ether",
					symbol: "ETH",
				};
	const name = chainName || `appchain: ${chainId}`;
	return createPublicClient({
		chain: defineChain({
			id: chainId,
			name: name,
			network: name,
			nativeCurrency,
			rpcUrls: {
				default: { http: [rpcUrl] },
				public: { http: [rpcUrl] },
			},
			blockExplorers: explorerUrl
				? {
						default: {
							name: `${name} Explorer`,
							url: explorerUrl,
						},
					}
				: undefined,
		}),
		transport: http(rpcUrl),
	});
}

async function getChainFromRpcUrl<T extends Record<string, { chain: Chain }>>(
	rpcUrl: string,
	supportedChains: T,
): Promise<Chain> {
	const chainId = await getRpcUrlChainId(rpcUrl);

	const chainIdStr = chainId.toString();
	if (chainIdStr in supportedChains) {
		return supportedChains[chainIdStr].chain;
	}

	throw new Error(
		`Could not resolve chain for RPC URL: ${rpcUrl} (chainId: ${chainId})`,
	);
}

async function getRpcUrlChainId(rpcUrl: string) {
	const res = await fetch(rpcUrl, {
		method: "POST",
		body: JSON.stringify({
			jsonrpc: "2.0",
			method: "eth_chainId",
			params: [],
			id: 1,
		}),
		headers: {
			"Content-Type": "application/json",
		},
	});
	if (!res.ok) {
		throw new Error(`Failed to get chainId for ${rpcUrl}`);
	}
	const chainId = await res.json();
	return hexToNumber(chainId.result);
}
