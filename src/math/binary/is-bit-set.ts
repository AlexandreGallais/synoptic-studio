import { integerQuotient, modulo } from "../arithmetic";

/**
 * Tells whether a bit of a non-negative integer is 1, bits numbered from 0 at the least
 * significant.
 *
 * Formula: bit i of n is ⌊n / 2ⁱ⌋ mod 2 (`integerQuotient`). Written with division, since the
 * bitwise operators of ECMAScript work on 32-bit signed integers.
 *
 * @kind math
 * @param value - an integer ≥ 0
 * @param bit - the bit's number, 0 for the least significant
 * @returns true when that bit is 1
 * @see DERIV-bit-of-integer
 */
export function isBitSet(value: number, bit: number): boolean {
  return modulo(integerQuotient(value, 2 ** bit), 2) === 1;
}
