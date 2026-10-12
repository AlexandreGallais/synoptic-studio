import type { FreeShape } from "./free-shape";
import type { RegularPolygon } from "./regular-polygon";
import type { RoundShape } from "./round-shape";

/** A shape of the model: free, regular polygon or round (ADR-0032, ADR-0036). */
export type Shape = FreeShape | RegularPolygon | RoundShape;

/**
 * Kind of a shape of the model (ADR-0036): a round shape by its `kind`, a regular polygon by its
 * `anchor`, otherwise a free shape.
 *
 * @kind domain
 * @param shape - a shape of the model
 * @returns `round`, `polygon` or `free`
 * @see docs/adr/0036-round-shape.md
 */
export function shapeKind(shape: Shape): "free" | "polygon" | "round" {
  if ("kind" in shape) {
    return "round";
  }

  return "anchor" in shape ? "polygon" : "free";
}
