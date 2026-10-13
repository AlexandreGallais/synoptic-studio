import type { CornerSide } from "../fillet-side";

/**
 * A side with the reach ADR-0007 leaves a fillet on it: on a straight side shared with a reduced
 * factor, the setback asked scaled by that factor, as a parameter; the whole side otherwise.
 *
 * Formula: limit = f · s / L when f < 1, else 1.
 *
 * @kind geometry
 * @param side - the side, written from the corner
 * @param factor - its factor (`sideFactors`)
 * @param setback - the setback the corner asked on it
 * @returns the side with its limit
 * @see DERIV-fillet-beside-curve
 */
export function limitedSide(side: CornerSide, factor: number, setback: number): CornerSide {
  return side.isStraight && factor < 1 && side.length > 0
    ? { ...side, limit: (factor * setback) / side.length }
    : side;
}
