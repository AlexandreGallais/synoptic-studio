import type { Shape } from "./shape";
import type { RegularPolygon } from "../regular-polygon";

/**
 * Whether a shape is a regular polygon: the only shape with an anchor.
 *
 * @kind domain
 * @param shape - a shape of the model
 * @returns `true` for a regular polygon
 * @see RULE-shapes-regular-polygon
 */
export function isRegularPolygon(shape: Shape): shape is RegularPolygon {
  return "anchor" in shape;
}
