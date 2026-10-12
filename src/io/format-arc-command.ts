import { formatCoordinatePair } from "./format-coordinate-pair";
import { formatSvgNumber } from "./format-svg-number";

import type { Arc } from "../geometry";

/**
 * Elliptical arc command of a fillet: `A r r 0 0 sweep x y`.
 *
 * A circle has rx = ry = r and no x-axis rotation; a fillet spans less than 180°, so the large-arc
 * flag is 0; the sweep flag is 1 when the arc turns in the positive direction (`DERIV-fillet-arc`
 * steps 4–5). Flags are single digits (§9.3.9).
 *
 * @kind format
 * @param arc - fillet arc, drawn from the current point to its end
 * @returns the `A` command, numbers at fixed precision
 * @see REF-SVG2-PATHS
 */
export function formatArcCommand(arc: Arc): string {
  const radius = formatSvgNumber(arc.radius);

  return `A${radius} ${radius} 0 0 ${arc.positive ? "1" : "0"} ${formatCoordinatePair(arc.end)}`;
}
