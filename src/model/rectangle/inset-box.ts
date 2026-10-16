import type { TurnedBox } from "../placement";

/**
 * A box inset by the same distance on each side, each side's parallel at that distance inward:
 * its corner moved in by it on both axes, its width and its height shorter by twice it, never
 * below 0. A box 100 × 40 at (10, 20) inset by 3 is 94 × 34 at (13, 23).
 *
 * Formula: (x + d, y + d), max(w − 2d, 0) × max(h − 2d, 0).
 *
 * @kind domain
 * @param box - the box's position and size
 * @param inset - the distance from each side, in user units
 * @returns the inner box
 * @see REF-WIKI-PARALLEL-CURVE
 */
export function insetBox(box: Omit<TurnedBox, "angle">, inset: number): Omit<TurnedBox, "angle"> {
  return {
    height: Math.max(box.height - 2 * inset, 0),
    width: Math.max(box.width - 2 * inset, 0),
    x: box.x + inset,
    y: box.y + inset,
  };
}
