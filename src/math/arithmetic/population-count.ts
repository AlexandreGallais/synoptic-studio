import { isBitSet } from "./is-bit-set";

/** The base of the binary representation. */
const BINARY = 2;

/**
 * The population count, or Hamming weight, of a non-negative integer: how many of its bits are 1.
 *
 * Formula: the number of i from 0 to the integer's bit length − 1 whose bit is 1.
 *
 * @kind math
 * @param value - an integer ≥ 0
 * @returns the number of bits set to 1
 * @see REF-WIKI-HAMMING-WEIGHT
 */
export function populationCount(value: number): number {
  return Array.from({ length: value.toString(BINARY).length }, (unused, bit) => bit).filter((bit) =>
    isBitSet(value, bit),
  ).length;
}
