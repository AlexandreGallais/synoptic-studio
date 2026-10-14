import type { FontMetrics } from "./font-metrics";

/** One face of a font family: a weight, upright or italic, and its metrics (ADR-0052). */
export type FontFace = {
  /** Weight, from 100 (thin) to 900 (black); 400 regular, 700 bold. */
  readonly weight: number;
  /** Whether the face is italic. */
  readonly isItalic: boolean;
  /** Its metrics, read from its font file. */
  readonly metrics: FontMetrics;
};
