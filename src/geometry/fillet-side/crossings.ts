import { average, bisectionBracket, regularPartition } from "../../math";
import { cyclicItem } from "../contour";

import { FILLET_STEPS } from "./fillet-steps";

/** Samples looked at for crossings: Pomax's coarse table before the binary search. */
const SAMPLES = 32;

/**
 * Every parameter, from 0, where a function stops being negative, in order: a coarse table of
 * samples, then a bisection inside each bracket where it turns (`REF-POMAX-BEZIER`, "Projecting a
 * point onto a Bézier curve"; `REF-WIKI-IVT`, `REF-MATHWORLD-BISECTION`).
 *
 * @kind geometry
 * @param isBelow - whether the function is negative at a parameter
 * @param end - the largest parameter looked at
 * @returns the crossings, closest to 0 first; none when it never turns
 * @see DERIV-fillet-beside-curve
 */
export function crossings(isBelow: (t: number) => boolean, end: number): readonly number[] {
  const steps = regularPartition(end, SAMPLES);

  return steps
    .slice(1)
    .filter((t, index) => isBelow(cyclicItem(steps, index, t)) && !isBelow(t))
    .map((t) => {
      const [low, high] = bisectionBracket(isBelow, [t - end / SAMPLES, t], FILLET_STEPS);

      return average(low, high);
    });
}
