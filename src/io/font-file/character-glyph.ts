/** A character and the glyph that draws it: its Unicode code point, then its glyph index (`REF-OT-CMAP`). */
export type CharacterGlyph = readonly [codePoint: number, glyph: number];
