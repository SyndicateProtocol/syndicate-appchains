import { toHex } from "viem"

const ALIAS_OFFSET = BigInt("0x1111000000000000000000000000000000001111")

/**
 * Apply aliasing: L1 -> L2 address transformation
 */
export function applyL1ToL2Alias(l1Address: string): string {
  return toHex(BigInt(l1Address) + ALIAS_OFFSET)
}

/**
 * Undo aliasing: L2 (aliased) -> L1 original
 */
export function undoL1ToL2Alias(l2Address: string): string {
  return toHex(BigInt(l2Address) - ALIAS_OFFSET)
}
