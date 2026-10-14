import {
  add,
  createPathElement,
  formatCoordinatePair,
  norm,
  quarterTurn,
  scale,
  subtract,
} from "../../src";

import type { CurveCorner, Point } from "../../src";

/** Half the width of a handle's stem, the thin line from its point. */
const HALF_BAR = 0.75;

/** Half the side of the square at a handle's end. */
const HALF_SQUARE = 4;

/**
 * Path data of a filled quadrilateral (frames and bars are fills, never a native stroke, US-018).
 *
 * @kind procedure
 * @param corners - its four corners, in order
 * @returns the closed path data
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
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
 * @returns the path data of the bar and the square
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function createHandlePathData(point: Point, end: Point): string {
  const along = subtract(end, point);
  const across = scale(quarterTurn(along), HALF_BAR / norm(along));
  const stem = [
    add(point, across),
    add(end, across),
    subtract(end, across),
    subtract(point, across),
  ];
  const square = [
    { x: end.x - HALF_SQUARE, y: end.y - HALF_SQUARE },
    { x: end.x + HALF_SQUARE, y: end.y - HALF_SQUARE },
    { x: end.x + HALF_SQUARE, y: end.y + HALF_SQUARE },
    { x: end.x - HALF_SQUARE, y: end.y + HALF_SQUARE },
  ];

  return `${createQuadrilateralPathData(stem)} ${createQuadrilateralPathData(square)}`;
}

/**
 * The handles drawn in Points mode: for each point in handles mode, each handle of some length
 * as a stem from the point and a square at its end (US-022).
 *
 * @kind procedure
 * @param document - page document
 * @param corners - the corners as drawn
 * @returns one path per handle drawn
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
export function createCurveHandleElements(
  document: Document,
  corners: readonly CurveCorner[],
): readonly SVGPathElement[] {
  return corners
    .filter((corner) => corner.handles.isOn)
    .flatMap((corner) =>
      [corner.handles.before, corner.handles.after].map((handle) => ({ corner, handle })),
    )
    .filter(({ handle }) => norm(handle) > 0)
    .map(({ corner, handle }) => {
      const element = createPathElement(
        document,
        createHandlePathData(corner.point, add(corner.point, handle)),
      );

      element.classList.add("curve-handle");

      return element;
    });
}
