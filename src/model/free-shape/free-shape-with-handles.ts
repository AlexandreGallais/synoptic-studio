import { turnedHandles } from "../../geometry";
import { turnDirection } from "../../math";
import { turnedCorners } from "../placement";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeKeptInPlace } from "./free-shape-kept-in-place";

import type { FreeShape } from "./free-shape";
import type { Handles } from "../../geometry";

/**
 * Free shape with the handles of one point replaced, given as drawn (turned with the shape): they
 * are turned back, stored with the shape at scale 1, and the box — the box of the curve, which
 * the handles may change — is placed so that the points stay where they are drawn (ADR-0035,
 * ADR-0046).
 *
 * @kind domain
 * @param shape - valid free shape (`isValidFreeShape`)
 * @param index - number of the point, from 0
 * @param handles - its new handles, switch and mode included, vectors as drawn on the page
 * @returns the free shape with those handles, its points in place
 * @see docs/domain/shapes.md
 */
export function freeShapeWithHandles(shape: FreeShape, index: number, handles: Handles): FreeShape {
  const straight =
    shape.angle === 0 ? handles : turnedHandles(handles, turnDirection(-shape.angle));
  const edited = freeShapeAroundCorners(
    freeShapeCorners(shape).map((corner, other) =>
      other === index ? { ...corner, handles: straight } : corner,
    ),
    shape.angle,
  );

  return freeShapeKeptInPlace(edited, turnedCorners(freeShapeCorners(shape), shape), {
    after: 0,
    before: 0,
  });
}
