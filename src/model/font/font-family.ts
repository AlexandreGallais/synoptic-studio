import type { FontFace } from "./font-face";

/** A font family and the metrics of its faces (ADR-0052). */
export type FontFamily = {
  /** The family's name, as the SVG's `font-family` names it. */
  readonly name: string;
  /** The version of the shipped data's files, Inter's `4.1`; empty for a family kept from the files the editor read (`fontsWithFace`), each face's file giving its own (`FontFile.version`). */
  readonly version: string;
  /** Its faces. */
  readonly faces: readonly FontFace[];
};
