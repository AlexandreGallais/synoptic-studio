import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleCorners } from "../../../tests";
import { NO_HANDLES, cubicPoint, curveContour } from "../../geometry";
import { curveCornerOf } from "../handles";

import { cornersWithMidpoint } from "./corners-with-midpoint";

import type { CurveCorner } from "../../geometry";

/**
 * The square 100 × 100, its top side an arch: points 1 and 2 with vertical handles of 40.
 *
 * @returns its corners
 */
function archedSquare(): CurveCorner[] {
  return rectangleCorners(100, 100, [0, 0, 0, 0]).map((corner, index) => ({
    ...corner,
    handles: {
      ...NO_HANDLES,
      after: { x: 0, y: index === 0 ? -40 : 0 },
      before: { x: 0, y: index === 1 ? -40 : 0 },
      isOn: index < 2,
      mode: "none",
    },
  }));
}

describe("cornersWithMidpoint (DERIV-cubic-sides step 4)", () => {
  it("[F08.AC3] adds a sharp point in the middle of a straight side", () => {
    const square = rectangleCorners(100, 100, [0, 0, 0, 0]).map((corner) => curveCornerOf(corner));

    expect(cornersWithMidpoint(square, 1)[2]).toEqual({
      handles: NO_HANDLES,
      point: { x: 100, y: 50 },
      radius: 0,
    });
  });

  it("[F12.AC3] splits a curved side at its middle, its neighbours' handles halved", () => {
    // The arch splits at (50, −30): the new point's handles are (−25, 0) and (25, 0); point 1's
    // handle after becomes (0, −20), point 2's handle before (0, −20).
    const corners = cornersWithMidpoint(archedSquare(), 0);

    expect(corners.map((corner) => corner.point)).toEqual([
      { x: 0, y: 0 },
      { x: 50, y: -30 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 },
    ]);

    expect(corners[1]?.handles).toEqual({
      after: { x: 25, y: 0 },
      before: { x: -25, y: 0 },
      isOn: true,
      mode: "angle-and-length",
    });

    expect([corners[0]?.handles.after, corners[2]?.handles.before]).toEqual([
      { x: 0, y: -20 },
      { x: 0, y: -20 },
    ]);
  });

  test.prop({ s: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "[F12.AC3] keeps the outline: the two new sides lie on the old curve",
    ({ s }) => {
      const before = archedSquare();
      const whole = {
        end: { x: 100, y: 0 },
        first: { x: 0, y: -40 },
        kind: "cubic",
        second: { x: 100, y: -40 },
        start: { x: 0, y: 0 },
      } as const;
      const halves = curveContour(cornersWithMidpoint(before, 0)).filter(
        (piece) => piece.kind === "cubic",
      );
      const [first = whole, second = whole] = halves;

      expect(halves).toHaveLength(2);
      expect(cubicPoint(first, s).y).toBeCloseTo(cubicPoint(whole, s / 2).y, 9);
      expect(cubicPoint(second, s).x).toBeCloseTo(cubicPoint(whole, (1 + s) / 2).x, 9);
    },
  );

  it("[F12.AC3] splits the closing side, the first point's handle before halved, mirrored points set to angle", () => {
    // Points 4 and 1 on, every handle (−40, 0): side 4 from (0, 100) to (0, 0) gives P₀₁ (−20, 100),
    // P₂₃ (−20, 0), P₀₁₂ (−30, 75), P₁₂₃ (−30, 25), M (−30, 50).
    const ends = new Set([0, 3]);
    const left = { x: -40, y: 0 };
    const square = rectangleCorners(100, 100, [0, 0, 0, 0]).map((corner, index) => ({
      ...corner,
      handles: { ...NO_HANDLES, after: left, before: left, isOn: ends.has(index) },
    }));
    const corners = cornersWithMidpoint(square, 3);
    const half = { x: -20, y: 0 };

    expect(corners.map((corner) => corner.point)).toEqual([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 },
      { x: -30, y: 50 },
    ]);

    expect(corners.map((corner) => corner.handles)).toEqual([
      { ...NO_HANDLES, after: left, before: half, isOn: true, mode: "angle" },
      { ...NO_HANDLES, after: left, before: left },
      { ...NO_HANDLES, after: left, before: left },
      { ...NO_HANDLES, after: half, before: left, isOn: true, mode: "angle" },
      { ...NO_HANDLES, after: { x: 0, y: -25 }, before: { x: 0, y: 25 }, isOn: true },
    ]);
  });

  it("leaves the stored handle of an off neighbour as it is", () => {
    // Point 2 is off: its handle before (0, −40) is not drawn, so not halved.
    const arched = archedSquare().map((corner, index) =>
      index === 1 ? { ...corner, handles: { ...corner.handles, isOn: false } } : corner,
    );

    expect(cornersWithMidpoint(arched, 0)[2]?.handles).toEqual(arched[1]?.handles);
  });
});
