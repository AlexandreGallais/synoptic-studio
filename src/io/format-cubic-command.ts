import { formatCoordinatePair } from "./format-coordinate-pair";

import type { Cubic } from "../geometry";

/**
 * Path command of a cubic side: `C`, its two control points and its end, each pair at fixed
 * precision (`REF-SVG2-PATHS` §9.3.6, EN-001).
 *
 * @kind format
 * @param cubic - start, control points and end; the start is the current point of the path
 * @returns `C x1 y1 x2 y2 x y`, written `Cx1 y1 x2 y2 x y`
 * @see REF-SVG2-PATHS
 */
export function formatCubicCommand(cubic: Cubic): string {
  return `C${[cubic.first, cubic.second, cubic.end].map((point) => formatCoordinatePair(point)).join(" ")}`;
}
