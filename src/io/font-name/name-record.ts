/** One name record of a font's `name` table, its string placed from the table's start (`REF-OT-NAME`). */
export type NameRecord = {
  /** Platform: 0 Unicode, 1 Macintosh, 3 Windows. */
  readonly platform: number;
  /** Encoding of the platform: for Windows, 1 Unicode BMP and 10 Unicode full repertoire. */
  readonly encoding: number;
  /** Language of the platform: 0x0409 English (United States) for Windows, 0 English for Macintosh. */
  readonly language: number;
  /** Name ID: 1 family, 5 version, 16 typographic family. */
  readonly nameId: number;
  /** Where the string starts, from the table's start, in bytes. */
  readonly offset: number;
  /** Length of the string, in bytes. */
  readonly length: number;
};
