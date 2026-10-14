/** The kerning of one class with every class, font units. */
type Row = readonly number[];

/** Kerning by classes of glyphs: `GPOS` pair adjustment format 2. */
export type ClassKerningRule = {
  /** Tells it from the kerning listed pair by pair. */
  readonly kind: "classes";
  /** Class of each first glyph the rule covers, by glyph; a glyph left out is not covered. */
  readonly firstClasses: Readonly<Record<number, number>>;
  /** Class of each second glyph, by glyph; a glyph left out is in class 0. */
  readonly secondClasses: Readonly<Record<number, number>>;
  /** Kerning of each pair of classes, font units, by first class then second class. */
  readonly values: readonly Row[];
};
