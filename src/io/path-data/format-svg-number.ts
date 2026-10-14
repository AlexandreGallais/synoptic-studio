import { SVG_DECIMALS } from "./svg-decimals";

/**
 * Number written in SVG path data: rounded to `SVG_DECIMALS` decimals, shortest form.
 *
 * Trailing zeros are dropped and a negative zero is written `0`. Rounding is that of
 * `Number.prototype.toFixed`: to the nearest, an exact binary tie away from zero (`0.015625` →
 * `0.01563`); a transcription must reproduce it rather than round half to even (ADR-0025).
 *
 * @kind format
 * @param value - finite derived coordinate or length, in user units
 * @returns its decimal text, e.g. `86.60254`, `2.5`, `0`
 * @see REF-SVG2-PATHS
 */
export function formatSvgNumber(value: number): string {
  const rounded = Number(value.toFixed(SVG_DECIMALS));

  return String(rounded);
}
