import { turnAbout } from "../../geometry";
import { turnDirection } from "../../math";
import { boxCenter, turnedCorners } from "../placement";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeKeptInPlace } from "./free-shape-kept-in-place";

import type { FreeShape } from "./free-shape";
import type { Point } from "../../math";

/**
 * Free shape after one of its points is typed in the coordinates of the SVG (ADR-0033): its
 * corners are kept as drawn, that point replaced, and its width and height become the box of its
 * curve (`curveBoundingBox`), so that stretching it later stays without loss. On a turned shape the point typed
 * is turned back into the straight shape, and the new box placed so that the other points stay
 * where they are drawn (ADR-0035, `DERIV-turned-shape` step 5); the integer typed then holds in
 * the drawing, within a rounding error.
 *
 * @kind domain
 * @param shape - valid free shape (`isValidFreeShape`)
 * @param index - number of the point, from 0
 * @param point - its new place as drawn: each coordinate typed is an integer, one not typed keeps
 * its drawn value
 * @returns the free shape with the point moved, to check with `isValidFreeShape` (points with no
 * width or no height between them are refused); an index out of range moves nothing but rebases the shape on its
 * drawn corners
 * @see docs/domain/interaction.md#_3-node-editing
 */
export function freeShapeWithPoint(shape: FreeShape, index: number, point: Point): FreeShape {
  const center = boxCenter(shape);
  const straight =
    shape.angle === 0 ? point : turnAbout(point, center, turnDirection(-shape.angle));
  const edited = freeShapeAroundCorners(
    freeShapeCorners(shape).map((corner, other) =>
      other === index ? { ...corner, point: straight } : corner,
    ),
    shape.angle,
  );
  const kept = index === 0 ? 1 : 0;

  return freeShapeKeptInPlace(edited, turnedCorners(freeShapeCorners(shape), shape), {
    after: kept,
    before: kept,
  });
}
