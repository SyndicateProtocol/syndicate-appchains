import {
	type Chain,
	createPublicClient,
	createWalletClient,
	type Hex,
	hexToNumber,
	http,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";

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
	});
	if (!res.ok) {
		throw new Error(`Failed to get chainId for ${rpcUrl}`);
	}
	const chainId = await res.json();
	return hexToNumber(chainId.result);
}
