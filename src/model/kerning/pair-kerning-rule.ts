/** Kerning listed glyph pair by glyph pair: `GPOS` pair adjustment format 1, or a `kern` format 0 subtable. */
export type PairKerningRule = {
  /** Tells it from the kerning by classes. */
  readonly kind: "pairs";
  /** Kerning of each listed pair, font units, by the pair's key (`glyphPairKey`, `REF-OT-KERN`). */
  readonly pairs: Readonly<Record<number, number>>;
};
