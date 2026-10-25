import { integerQuotient, nonNegativeRemainder } from "../../math";

/** Values of one byte. */
const BYTE_VALUES = 256;

/**
 * Writes an unsigned integer in big-endian byte order, the order of every OpenType field ("All
 * OpenType fonts use big-endian (network) byte order"): its most significant byte first.
 *
 * Formula: byte i = ⌊value / 256^(size − 1 − i)⌋ mod 256, for i = 0 … size − 1.
 *
 * @kind format
 * @param value - from 0 to 256^size − 1
 * @param size - its size in bytes: 2 for a `uint16`, 4 for a `uint32` or an `Offset32`
 * @returns its bytes, most significant first
 */
export function unsignedToBigEndian(value: number, size: number): readonly number[] {
  return Array.from({ length: size }, (unused, index) =>
    nonNegativeRemainder(integerQuotient(value, BYTE_VALUES ** (size - 1 - index)), BYTE_VALUES),
  );
}
