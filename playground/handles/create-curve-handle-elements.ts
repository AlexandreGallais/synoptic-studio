import {
  add,
  createPathElement,
  formatCoordinatePair,
  norm,
  quarterTurn,
  scale,
  screenToUser,
  subtract,
} from "../../src";
import { readView } from "../view";

import type { CurveCorner, Point } from "../../src";

/** Half the width of a handle's stem, the thin line from its point, in screen pixels. */
const HALF_BAR = 0.75;

/** Half the side of the square at a handle's end, in screen pixels. */
const HALF_SQUARE = 4;

/** Half the sizes of a handle's marks, in user units at the view's scale. */
type HalfSizes = {
  /** Half the width of its stem. */
  readonly stem: number;
  /** Half the side of its square. */
  readonly square: number;
};

/**
 * Path data of a filled quadrilateral (frames and bars are fills, never a native stroke).
 *
 * @kind procedure
 * @param corners - its four corners, in order
 * @returns the closed path data
 */
function createQuadrilateralPathData(corners: readonly Point[]): string {
  return `M${corners.map((corner) => formatCoordinatePair(corner)).join(" L")} Z`;
}

/**
 * Path data of one handle: a thin stem from its point to its end, and a small square at the end.
 *
 * @kind procedure
 * @param point - where the handle starts, on the curve
 * @param end - where it ends, the control point
 * @param half - half the width of its stem and half the side of its square, in user units
 * @returns the path data of the bar and the square
 */
function createHandlePathData(point: Point, end: Point, half: HalfSizes): string {
  const along = subtract(end, point);
  const across = scale(quarterTurn(along), half.stem / norm(along));
  const stem = [
    add(point, across),
    add(end, across),
    subtract(end, across),
    subtract(point, across),
  ];
  const square = [
    { x: end.x - half.square, y: end.y - half.square },
    { x: end.x + half.square, y: end.y - half.square },
    { x: end.x + half.square, y: end.y + half.square },
    { x: end.x - half.square, y: end.y + half.square },
  ];

  return `${createQuadrilateralPathData(stem)} ${createQuadrilateralPathData(square)}`;
}

/**
 * The handles drawn in Points mode: for each point in handles mode, each handle of some length
 * as a stem from the point and a square at its end, 1.5 and 8 screen pixels divided by the
 * view's scale, so that they keep their size on screen at any zoom (`screenToUser`, ADR-0098,
 * decision 5).
 *
 * @kind procedure
 * @param document - page document
 * @param corners - the corners as drawn
 * @returns one path per handle drawn
 */
export function createCurveHandleElements(
  document: Document,
  corners: readonly CurveCorner[],
): readonly SVGPathElement[] {
  const { scale: viewScale } = readView(document);
  const half = {
    stem: screenToUser(HALF_BAR, viewScale),
    square: screenToUser(HALF_SQUARE, viewScale),
  };

  return corners
    .filter((corner) => corner.handles.isOn)
    .flatMap((corner) =>
      [corner.handles.before, corner.handles.after].map((handle) => ({ corner, handle })),
    )
    .filter(({ handle }) => norm(handle) > 0)
    .map(({ corner, handle }) => {
      const element = createPathElement(
        document,
        createHandlePathData(corner.point, add(corner.point, handle), half),
      );

      element.classList.add("curve-handle");

      return element;
    });
}
