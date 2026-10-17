/** The `sfntVersion` of a font file, its first four bytes, by the outlines it holds (`REF-OT-OTFF`). */
export const SFNT_VERSION: {
  /** A TrueType font as Apple's systems tag it: the tag `true` (`REF-APPLE-TT-SCALER`). */
  readonly appleTrueType: number;
  /** A font with CFF outlines: the tag `OTTO`. */
  readonly cff: number;
  /** A font with TrueType outlines. */
  readonly trueType: number;
} = { appleTrueType: 0x74_72_75_65, cff: 0x4f_54_54_4f, trueType: 0x00_01_00_00 };
