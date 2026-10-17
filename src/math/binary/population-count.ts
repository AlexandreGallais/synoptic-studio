import { integerRange } from "../arithmetic";

import { bitLength } from "./bit-length";
import { isBitSet } from "./is-bit-set";

/**
 * The population count, or Hamming weight, of a non-negative integer: how many of its bits are 1.
 *
 * Formula: the number of i from 0 to the integer's bit length − 1 (`bitLength`) whose bit is 1.
 *
 * @kind math
 * @param value - an integer ≥ 0
 * @returns the number of bits set to 1
 * @see REF-WIKI-HAMMING-WEIGHT
 */
export function populationCount(value: number): number {
  return integerRange(0, bitLength(value) - 1).filter((bit) => isBitSet(value, bit)).length;
}
