import type { FontFace } from "./font-face";

/** A font family and the metrics of its faces (ADR-0052). */
export type FontFamily = {
  /** The family's name, as the SVG's `font-family` names it. */
  readonly name: string;
  /** The version of its files the metrics were read from; empty for a family kept from the files the editor read (`fontsWithFace`). */
  readonly version: string;
  /** Its faces. */
  readonly faces: readonly FontFace[];
};
