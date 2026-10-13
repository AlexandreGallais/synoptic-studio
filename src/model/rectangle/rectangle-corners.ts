import { freeShapeCorners } from "../free-shape";

import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

import type { Rectangle } from "./rectangle";
import type { Corner } from "../../geometry";

/**
 * Straight corners of a rectangle: those of the free shape the rectangle tool makes (ADR-0032),
 * each vertex with its requested radius (ADR-0029); its angle is not applied (`turnedCorners`).
 *
 * @kind domain
 * @param rectangle - width, height and the radius of each corner
 * @returns four corners, clockwise from the top-left vertex (Q11)
 * @see docs/domain/shapes.md#_2-corner-radius
 */
export function rectangleCorners(rectangle: Rectangle): readonly Corner[] {
  // A rectangle has no handles (EN-024): its corners are plain.
  return freeShapeCorners(rectangleAsFreeShape(rectangle)).map(({ point, radius }) => ({
    point,
    radius,
  }));
}
