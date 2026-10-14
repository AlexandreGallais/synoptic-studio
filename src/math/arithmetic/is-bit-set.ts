/**
 * Tells whether a bit of a non-negative integer is 1, bits numbered from 0 at the least
 * significant one.
 *
 * Formula: an integer is Σ bᵢ · 2ⁱ, so bit i is ⌊n / 2ⁱ⌋ mod 2. Written with division, since the
 * bitwise operators of ECMAScript work on 32-bit signed integers.
 *
 * @kind math
 * @param value - an integer ≥ 0
 * @param bit - the bit's number, 0 for the least significant
 * @returns true when that bit is 1
 * @see REF-WIKI-BIT-NUMBERING
 */
export function isBitSet(value: number, bit: number): boolean {
  return Math.floor(value / 2 ** bit) % 2 === 1;
}
