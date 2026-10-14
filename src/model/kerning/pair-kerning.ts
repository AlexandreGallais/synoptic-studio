import type { Kerning } from "./kerning";

/**
 * The kerning between two adjacent characters: the value of the pair of their classes, 0 when
 * either kerns with nothing.
 *
 * @kind domain
 * @param kerning - the font's kerning
 * @param left - code point of the first character
 * @param right - code point of the second character
 * @returns the adjustment of the first character's advance, font units; negative tightens
 * @see REF-OT-GPOS
 */
export function pairKerning(kerning: Kerning, left: number, right: number): number {
  return kerning.values[kerning.leftClasses[left] ?? 0]?.[kerning.rightClasses[right] ?? 0] ?? 0;
}
