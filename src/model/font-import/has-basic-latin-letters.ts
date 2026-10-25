import { BASIC_LATIN_LETTERS } from "./basic-latin-letters";

import type { FontMetrics } from "../font";

/**
 * Whether a font maps every letter of the ISO basic Latin alphabet, A to Z and a to z
 * (`BASIC_LATIN_LETTERS`), to a glyph of its own: a font that lacks one draws it with the
 * browser's fallback while the editor measures it with `.notdef`, its frame then not matching its
 * letters (Apple Color Emoji, Al Nile, Apple Braille map none).
 *
 * Formula: ∀ c ∈ {0x41 … 0x5A, 0x61 … 0x7A}, c ∈ advances.
 *
 * @kind domain
 * @param metrics - the font's metrics, its advances listing the characters it maps
 * @returns true when all 52 letters are mapped
 * @see REF-WIKI-ISO-BASIC-LATIN
 */
export function hasBasicLatinLetters(metrics: FontMetrics): boolean {
  return BASIC_LATIN_LETTERS.every((codePoint) => Object.hasOwn(metrics.advances, codePoint));
}
