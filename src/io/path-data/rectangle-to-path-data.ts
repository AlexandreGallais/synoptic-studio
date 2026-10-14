import { formatCoordinatePair } from "./format-coordinate-pair";

import type { LineDecoration } from "../../model";

/**
 * Path data of an upright rectangle, clockwise on screen from its top-left corner (Q11): the
 * top-left, top-right, bottom-right and bottom-left corners joined by lines, then closed
 * (SVG 2, §9.3.3 to §9.3.5).
 *
 * @kind format
 * @param rectangle - its top-left corner, width and height, in user units
 * @returns `M x y L x+w y L x+w y+h L x y+h Z`, numbers at fixed precision
 * @see REF-SVG2-PATHS
 */
export function rectangleToPathData(rectangle: LineDecoration): string {
  const { height, width, x, y } = rectangle;
  const corners = [
    { x, y },
    { x: x + width, y },
    { x: x + width, y: y + height },
    { x, y: y + height },
  ].map((corner) => formatCoordinatePair(corner));

  return `M${corners.join(" L")} Z`;
}
