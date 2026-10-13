import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint, rectangleCorners } from "../../../tests";
import { roundedContour } from "../corner-radius";

import { curveContour } from "./curve-contour";

import type { CurveCorner } from "./curve-corner";
import type { Handles } from "./handles";
import type { Corner } from "../contour";

/** Handles off: the drawing of E01. */
const OFF: Handles = {
  after: { x: 0, y: 0 },
  before: { x: 0, y: 0 },
  isOn: false,
  mode: "angle-and-length",
};

/**
 * Corners with their handles off, then some replaced.
 *
 * @param corners - points and radii
 * @param changes - handles of some corners, by number
 * @returns the corners with handles
 */
function curved(
  corners: readonly Corner[],
  changes: Readonly<Record<number, Partial<Handles>>>,
): CurveCorner[] {
  return corners.map((corner, index) => {
    const change = changes[index];

    return { ...corner, handles: change === undefined ? OFF : { ...OFF, ...change, isOn: true } };
  });
}

describe("curveContour (ADR-0046)", () => {
  it("[F12.AC5] is the rounded contour, piece for piece, when every handle is off", () => {
    const corners = rectangleCorners(100, 50, [10, 0, 25, 5]);

    expect(curveContour(curved(corners, {}))).toEqual(roundedContour(corners));
  });

  it("[F12.AC3] draws the top side of a square as a cubic, its corners sharp, the others rounded", () => {
    // Point 1 (0, 0) after (30, −15): the side to (100, 0) is a cubic; point 2 keeps radius 10 but
    // is next to a curved side, so it is sharp; points 3 and 4 keep their fillets.
    const pieces = curveContour(
      curved(rectangleCorners(100, 100, [0, 10, 10, 10]), { 0: { after: { x: 30, y: -15 } } }),
    );

    expect(pieces.filter((piece) => piece.kind === "cubic")).toEqual([
      {
        end: { x: 100, y: 0 },
        first: { x: 30, y: -15 },
        kind: "cubic",
        second: { x: 100, y: 0 },
        start: { x: 0, y: 0 },
      },
    ]);

    expect(pieces.filter((piece) => piece.kind === "arc").map((piece) => piece.radius)).toEqual([
      10, 10,
    ]);
  });

  it("[F12.AC3] keeps a curved closing side, from the last point to the first", () => {
    // Point 4 (0, 100) after (−20, −30): the side back to (0, 0) is a cubic.
    const pieces = curveContour(
      curved(rectangleCorners(100, 100, [0, 0, 0, 0]), { 3: { after: { x: -20, y: -30 } } }),
    );

    expect(pieces.at(-1)).toEqual({
      end: { x: 0, y: 0 },
      first: { x: -20, y: 70 },
      kind: "cubic",
      second: { x: 0, y: 0 },
      start: { x: 0, y: 100 },
    });
  });

  test.prop({ points: fc.array(integerPoint(100), { maxLength: 6, minLength: 3 }) })(
    "draws each curved side once, from its point to the next",
    ({ points }) => {
      const [first, second] = points;

      fc.pre(
        first !== undefined &&
          second !== undefined &&
          (first.x !== second.x || first.y !== second.y),
      );

      const corners = points.map((point) => ({ point, radius: 5 }));
      const cubics = curveContour(curved(corners, { 0: { after: { x: 5, y: 5 } } })).filter(
        (piece) => piece.kind === "cubic",
      );

      expect(cubics.map((cubic) => [cubic.start, cubic.end])).toEqual([[first, second]]);
    },
  );
});
