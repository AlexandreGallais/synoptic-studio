/** The opacity a percentage of 100 stands for: fully opaque. */
const WHOLE = 100;

/**
 * The `fill-opacity` number of an opacity typed in percent: the percentage of the range 0 (fully
 * transparent) to 1 (fully opaque), written as a number, which SVG 1.1 readers read too (SVG 2,
 * §13.4.3).
 *
 * Formula: percent / 100.
 *
 * @kind format
 * @param percent - the opacity, an integer from 0 to 100
 * @returns the opacity from 0 to 1
 * @see REF-SVG2-FILL-OPACITY
 */
export function percentToOpacity(percent: number): number {
  return percent / WHOLE;
}
