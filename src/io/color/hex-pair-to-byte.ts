/** The base of hexadecimal digits. */
const HEXADECIMAL = 16;

/**
 * The byte two hexadecimal digits stand for, from `00` (0) to `ff` (255), as each pair of a hex
 * color gives its red, green, blue or alpha (REF-CSS-COLOR-4, §5.2).
 *
 * Formula: 16 · first digit + second digit.
 *
 * @kind format
 * @param pair - two hexadecimal digits, already checked
 * @returns the byte, an integer from 0 to 255
 * @see REF-CSS-COLOR-4
 */
export function hexPairToByte(pair: string): number {
  return Number.parseInt(pair, HEXADECIMAL);
}
