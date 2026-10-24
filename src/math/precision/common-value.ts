import { isWithinTolerance } from "./is-within-tolerance";

/**
 * The value common to several: the first one, when every one lies within a tolerance of it
 * (`isWithinTolerance`); none when one lies farther, or when there is none. A tolerance of 0
 * compares exactly, `-0` equal to `0`; `NaN` and an infinite value are within none of another.
 *
 * Formula: v₁ when |vᵢ − v₁| ≤ t for every i.
 *
 * @kind math
 * @param values - the values, in any order, the first one the value kept
 * @param tolerance - the largest variation accepted, t ≥ 0
 * @returns the first value, `undefined` when one differs or there is none (an absent value,
 * ADR-0025)
 * @see REF-WIKI-ENGINEERING-TOLERANCE
 */
export function commonValue(values: readonly number[], tolerance: number): number | undefined {
  const [first] = values;

  if (first === undefined) {
    return undefined;
  }

  return values.every((value) => isWithinTolerance(value, first, tolerance)) ? first : undefined;
}
