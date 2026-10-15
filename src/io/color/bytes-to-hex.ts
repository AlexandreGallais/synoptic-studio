/** The base of hexadecimal digits. */
const HEXADECIMAL = 16;

/** The digits of one byte. */
const DIGITS_PER_BYTE = 2;

/**
 * A color's stored form from its red, green and blue bytes: `#` and each byte in two lower-case
 * hexadecimal digits (REF-CSS-COLOR-4, §16.2.1; ADR-0056). 255, 128, 0 is `#ff8000`.
 *
 * @kind format
 * @param bytes - the red, green and blue bytes, integers from 0 to 255
 * @returns the color, `#` and six lower-case digits
 * @see REF-CSS-COLOR-4
 */
export function bytesToHex(bytes: readonly number[]): string {
  const pairs = bytes.map((byte) => byte.toString(HEXADECIMAL).padStart(DIGITS_PER_BYTE, "0"));

  return `#${pairs.join("")}`;
}
