import fs from "node:fs/promises";
import path from "node:path";
import {
	type Address,
	type Chain,
	erc20Abi,
	getAddress,
	type Hex,
	type PublicClient,
	stringify,
	zeroAddress,
} from "viem";

import type { ChainNativeCurrency, GetChainsResponse } from "@/src/types";
import type { SyndObjectPaths } from "./schema";

export async function getDoesChainExist(chainId: number) {
	const res = await fetch("https://chainid.network/chains.json");
	const chains = (await res.json()) as GetChainsResponse;
	const chain = chains.find((chain) => chain.chainId === chainId);
	if (chain) {
		console.debug(`Chain ${chainId} already exists`, chain);
		return true;
	}
	return false;
}

export function isNativeTokenEth(nativeTokenAddress?: string) {
	return nativeTokenAddress === undefined || nativeTokenAddress === zeroAddress;
}

export async function getNativeCurrency(
	parentChainClient: PublicClient,
	tokenAddress: Hex,
) {
	let nativeCurrency: ChainNativeCurrency;
	if (isNativeTokenEth(tokenAddress)) {
		nativeCurrency = {
			decimals: 18,
			name: "Ether",
			symbol: "ETH",
		};
	} else {
		const [decimals, name, symbol] = await Promise.all([
			parentChainClient.readContract({
				address: tokenAddress,
				abi: erc20Abi,
				functionName: "decimals",
			}),
			parentChainClient.readContract({
				address: tokenAddress,
				abi: erc20Abi,
				functionName: "name",
			}),
			parentChainClient.readContract({
				address: tokenAddress,
				abi: erc20Abi,
				functionName: "symbol",
			}),
		]);
		nativeCurrency = {
			decimals,
			name,
			symbol,
		};
	}
	return nativeCurrency;
}

export function getChainExplorerUrl(chain: Chain) {
	return chain.blockExplorers?.default.url;
}

export function getChainRpcUrl(chain: Chain) {
	return chain.rpcUrls.default.http[0];
}

export async function writeToFile(
	chainName: string,
	fileName: string,
	data: string,
) {
	const basePath = path.join(__dirname, "../../", "out");

	// If chainName is provided, create a subdirectory
	const targetPath = chainName
		? path.join(basePath, chainName, fileName)
		: path.join(basePath, fileName);

	if (chainName) {
		// Ensure the directory exists
		const chainDir = path.join(basePath, chainName);
		await fs.mkdir(chainDir, { recursive: true });
	}

	return Bun.write(targetPath, data);
}

export async function upsertToSyndObject(
	chainName: string,
	environment: string,
	objectPath: SyndObjectPaths,
	data: unknown,
) {
	const basePath = path.join(__dirname, "../../", "out");
	const chainDir = path.join(basePath, chainName);
	const syndFilePath = path.join(
		chainDir,
		`${environment}.synd.${chainName}.json`,
	);

	// Ensure the directory exists
	await fs.mkdir(chainDir, { recursive: true });

	let syndObject: Record<string, unknown> = {};

	// Try to read existing synd file
	try {
		const existingData = await fs.readFile(syndFilePath, "utf-8");
		syndObject = JSON.parse(existingData);
	} catch (_error) {
		// File doesn't exist or is invalid, start with empty object
	}

	// Set the value at the specified path
	const pathParts = objectPath.split(".");
	let current: Record<string, unknown> = syndObject;

	for (let i = 0; i < pathParts.length - 1; i++) {
		const part = pathParts[i];
		if (part === "__proto__" || part === "constructor" || part === "prototype") {
			throw new Error(`Prototype pollution property blocked in objectPath: '${part}'`);
		}
		if (!current[part]) {
			current[part] = {};
		}
		current = current[part] as Record<string, unknown>;
	}

	const lastPart = pathParts[pathParts.length - 1];
	if (lastPart === "__proto__" || lastPart === "constructor" || lastPart === "prototype") {
		throw new Error(`Prototype pollution property blocked in objectPath: '${lastPart}'`);
	}
	current[lastPart] = data;

	// Write the updated object back to file
	return Bun.write(syndFilePath, stringify(syndObject, null, 2));
}

export function isValidAddress(address: string) {
	try {
		getAddress(address);
		return true;
	} catch (_error) {
		return false;
	}
}

export function isAddressEq(
	address1: Address | Hex | string,
	address2: Address | Hex | string,
) {
	return getAddress(address1) === getAddress(address2);
}

export async function readEoaSecrets(
	chainName: string,
	environment: string,
): Promise<Record<string, { address: string; privateKey: string }>> {
	const basePath = path.join(__dirname, "../../", "out");
	const chainDir = path.join(basePath, chainName);
	const eoaSecretsPath = path.join(
		chainDir,
		`${environment}-eoaSecrets.${chainName}.json`,
	);

	try {
		const existingData = await fs.readFile(eoaSecretsPath, "utf-8");
		return JSON.parse(existingData);
	} catch (_error) {
		// File doesn't exist or is invalid, return empty object
		return {};
	}
}

export async function upsertToEoaSecrets(
	chainName: string,
	environment: string,
	role: string,
	eoaData: { address: string; privateKey: string },
) {
	const eoaSecrets = await readEoaSecrets(chainName, environment);
	eoaSecrets[role] = eoaData;

	return writeToFile(
		chainName,
		`${environment}-eoaSecrets.${chainName}.json`,
		JSON.stringify(eoaSecrets, null, 2),
	);
}

export function scaleByPercentage(
	value: bigint,
	percentageScaleFactor: number,
): bigint {
	return (value * BigInt(percentageScaleFactor)) / BigInt(100);
}
