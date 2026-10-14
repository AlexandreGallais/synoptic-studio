/** A Unicode block: a range of code points named in the Unicode Character Database (`REF-UNICODE-BLOCKS`). */
export type UnicodeBlock = {
  /** Its name in `Blocks.txt`. */
  readonly name: string;
  /** Its first code point. */
  readonly first: number;
  /** Its last code point. */
  readonly last: number;
};
