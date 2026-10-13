import { bisection } from "../../math";

/** Samples looked at for the first crossing: Pomax's coarse table before the binary search. */
const SAMPLES = 64;

/**
 * The first parameter, from 0, where a function stops being negative: a coarse table of samples,
 * then a bisection inside the first bracket — the solution closest to the corner
 * (`REF-POMAX-BEZIER`, "Projecting a point onto a Bézier curve"; `REF-INKSCAPE-FILLET-SPEC`;
 * `REF-WIKI-IVT`, `REF-MATHWORLD-BISECTION`).
 *
 * @kind geometry
 * @param isBelow - whether the function is still negative at a parameter
 * @param end - the largest parameter looked at
 * @returns the crossing, or `undefined` when the function stays negative up to `end`
 * @see DERIV-fillet-beside-curve
 */
export function firstCrossing(isBelow: (t: number) => boolean, end: number): number | undefined {
  const steps = Array.from({ length: SAMPLES }, (unused, index) => ((index + 1) * end) / SAMPLES);
  const first = steps.findIndex((t) => !isBelow(t));

  return first === -1
    ? undefined
    : bisection(isBelow, (first * end) / SAMPLES, ((first + 1) * end) / SAMPLES);
}
