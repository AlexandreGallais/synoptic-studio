import type { SecondGlyphKerning } from "./second-glyph-kerning";

/** Kerning listed glyph pair by glyph pair: `GPOS` pair adjustment format 1, or a `kern` format 0 subtable. */
export type PairKerningRule = {
  /** Tells it from the kerning by classes. */
  readonly kind: "pairs";
  /** Kerning of each listed pair, font units, by first glyph then second glyph. */
  readonly pairs: Readonly<Record<number, SecondGlyphKerning>>;
};
