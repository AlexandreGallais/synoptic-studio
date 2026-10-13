import { filletSetback } from "../corner-radius";

import type { CornerAsk } from "./corner-ask";

/**
 * The setbacks a corner asks on its two sides, before ADR-0007's sharing: r tan(|τ|/2) between
 * two straight sides (`DERIV-fillet-setback`); beside a curve, how far along its straight side its
 * fillet touches (`DERIV-fillet-beside-curve` step 8), 0 on the curve.
 *
 * @kind geometry
 * @param ask - the corner's radius, angle, sides and fillet on whole sides
 * @returns the setback on the side before, then on the side after
 * @see DERIV-fillet-beside-curve
 */
export function requestedSetbacks(ask: CornerAsk): readonly [number, number] {
  const [before, after] = ask.sides;

  if (ask.fillet === undefined) {
    return [filletSetback(ask.radius, ask.angle), filletSetback(ask.radius, ask.angle)];
  }

  return [
    before.isStraight ? ask.fillet.before * before.length : 0,
    after.isStraight ? ask.fillet.after * after.length : 0,
  ];
}
