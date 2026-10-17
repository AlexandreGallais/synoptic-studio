import { ruleKerning } from "./rule-kerning";

import type { KerningLookup } from "./kerning-lookup";

/**
 * The kerning of a pair of glyphs: in each lookup the first subtable matching the pair gives its
 * value, and the lookups add up.
 *
 * @kind domain
 * @param lookups - the font's kerning lookups, in lookup list order
 * @param first - glyph of the first character
 * @param second - glyph of the second character
 * @returns the kerning, font units; 0 when no subtable matches
 * @see REF-OT-LAYOUT-COMMON
 */
export function glyphPairKerning(
  lookups: readonly KerningLookup[],
  first: number,
  second: number,
): number {
  return lookups.reduce(
    (sum, lookup) =>
      sum +
      (lookup.reduce<number | undefined>(
        (found, rule) => found ?? ruleKerning(rule, first, second),
        undefined,
      ) ?? 0),
    0,
  );
}
