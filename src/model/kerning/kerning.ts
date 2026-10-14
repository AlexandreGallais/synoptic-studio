import type { KerningRow } from "./kerning-row";

/**
 * The kerning of a font between characters, by classes (ADR-0052): characters that kern alike
 * share a class, so that a pair's kerning is read from a small table. Class 0 kerns with nothing.
 */
export type Kerning = {
  /** Class of each character as the first of a pair, by code point; a character left out is in class 0. */
  readonly leftClasses: Readonly<Record<number, number>>;
  /** Class of each character as the second of a pair, by code point; a character left out is in class 0. */
  readonly rightClasses: Readonly<Record<number, number>>;
  /** Kerning of each pair of classes, font units, by left class then right class; negative tightens. */
  readonly values: readonly KerningRow[];
};
