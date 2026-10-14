import { glyphPairKey } from "./glyph-pair-key";

import type { ClassKerningRule } from "./class-kerning-rule";
import type { KerningRule } from "./kerning-rule";

/**
 * The kerning a rule by classes gives a pair of glyphs: the value of their classes when it covers
 * the first glyph, the second glyph in class 0 when its class definition leaves it out.
 *
 * @kind domain
 * @param rule - a rule by classes
 * @param first - glyph of the first character
 * @param second - glyph of the second character
 * @returns the kerning, font units; undefined when the rule does not cover the first glyph
 * @see REF-OT-GPOS
 */
function groupRuleKerning(
  rule: ClassKerningRule,
  first: number,
  second: number,
): number | undefined {
  const firstClass = rule.firstClasses[first];

  return firstClass === undefined
    ? undefined
    : (rule.values[firstClass]?.[rule.secondClasses[second] ?? 0] ?? 0);
}

/**
 * The kerning a subtable gives a pair of glyphs, if it matches the pair: a rule by pairs matches
 * a listed pair; a rule by classes matches every pair whose first glyph it covers.
 *
 * @kind domain
 * @param rule - the subtable, by pairs or by classes
 * @param first - glyph of the first character
 * @param second - glyph of the second character
 * @returns the kerning, font units; undefined when the rule does not match the pair
 * @see REF-OT-GPOS
 */
export function ruleKerning(rule: KerningRule, first: number, second: number): number | undefined {
  return rule.kind === "pairs"
    ? rule.pairs[glyphPairKey(first, second)]
    : groupRuleKerning(rule, first, second);
}
