/** The base of the binary representation. */
const BINARY = 2;

/**
 * The bit length of a non-negative integer: how many binary digits write it, 0 for 0 (the empty
 * string), 1 for 1, 3 for 4 to 7.
 *
 * Formula: ℓ(0) = 0, ℓ(n) = ⌊log₂ n⌋ + 1, the length of n in base 2.
 *
 * @kind math
 * @param value - an integer ≥ 0
 * @returns the number of its binary digits
 * @see REF-WIKI-BIT-LENGTH
 */
export function bitLength(value: number): number {
  return value === 0 ? 0 : value.toString(BINARY).length;
}
