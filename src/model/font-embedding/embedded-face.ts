import type { FontFile } from "../font";

/** A face written in a document as an `@font-face` rule: its family's name, its weight, its style and the file embedded (ADR-0053). */
export type EmbeddedFace = {
  /** The family's name, as the texts' `font-family` names it. */
  readonly family: string;
  /** Weight, from 100 (thin) to 900 (black). */
  readonly weight: number;
  /** Whether the face is italic. */
  readonly isItalic: boolean;
  /** The file embedded whole, its `fsType` permitting it. */
  readonly file: FontFile;
};
