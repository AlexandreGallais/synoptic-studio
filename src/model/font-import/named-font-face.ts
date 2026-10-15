import type { FontFace } from "../font";

/** A face read from a font file, and the family its file names (`REF-OT-NAME`). */
export type NamedFontFace = {
  /** The family's name: name ID 16 of the file, else name ID 1. */
  readonly family: string;
  /** The face: its weight, its style, its metrics and its file. */
  readonly face: FontFace;
};
