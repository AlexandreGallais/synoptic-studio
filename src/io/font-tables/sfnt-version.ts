/** The `sfntVersion` of a font file, its first four bytes, by the outlines it holds (`REF-OT-OTFF`). */
export const SFNT_VERSION: {
  /** A font with CFF outlines: the tag `OTTO`. */
  readonly cff: number;
  /** A font with TrueType outlines. */
  readonly trueType: number;
} = { cff: 0x4f_54_54_4f, trueType: 0x00_01_00_00 };
