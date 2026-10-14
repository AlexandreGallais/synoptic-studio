import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cyclicItem } from "../contour";
import { cubicSide, curveBoundingBox } from "../curve";
import { cubicPoint } from "../placement";

import { ellipseCorners } from "./ellipse-corners";

import type { Point, Vector } from "../../math";
import type { CurveCorner } from "../handles";
import type { Size } from "../layout-box";

/** A corner read where none is expected: its numbers are NaN, so that any test on it fails. */
const NO_CORNER: CurveCorner = {
  handles: {
    after: { x: NaN, y: NaN },
    before: { x: NaN, y: NaN },
    isOn: false,
    mode: "none",
  },
  point: { x: NaN, y: NaN },
  radius: NaN,
};

/**
 * The point at t of each of the 4 sides of the shape.
 *
 * @param corners - the shape's corners
 * @param t - parameter from 0 to 1
 * @returns one point per side
 */
function sidePoints(corners: readonly CurveCorner[], t: number): Point[] {
  return corners.map((corner, index) =>
    cubicPoint(cubicSide(corner, cyclicItem(corners, index + 1, NO_CORNER)), t),
  );
}

/**
 * The handle after of a corner of an ellipse, rounded to 5 decimals.
 *
 * @param size - the ellipse's box
 * @param index - number of the corner
 * @returns its handle after
 */
function roundedHandle(size: Size, index: number): Vector {
  const { after } = cyclicItem(ellipseCorners(size), index, NO_CORNER).handles;

  return { x: Math.round(after.x * 1e5) / 1e5, y: Math.round(after.y * 1e5) / 1e5 };
}

describe("ellipseCorners (DERIV-round-shape-curve)", () => {
  it("[F10.AC3] puts a circle's 4 points at the top, right, bottom and left, clockwise", () => {
    expect(ellipseCorners({ height: 100, width: 100 }).map((corner) => corner.point)).toEqual([
      { x: 50, y: 0 },
      { x: 100, y: 50 },
      { x: 50, y: 100 },
      { x: 0, y: 50 },
    ]);
  });

  it("[F10.AC3] gives each point handles of 50κ ≈ 27.61424 along the curve, mirrored", () => {
    const top = cyclicItem(ellipseCorners({ height: 100, width: 100 }), 0, NO_CORNER);

    expect(top.handles).toMatchObject({ isOn: true, mode: "angle-and-length" });
    expect(top.handles.after.x).toBeCloseTo(27.61424, 5);
    expect(top.handles.before.x).toBeCloseTo(-27.61424, 5);
    expect(top.radius).toBe(0);
  });

  it("[F10.AC3] gives an ellipse 120 × 80 handles of 60κ across and 40κ down", () => {
    // Step 3: top and bottom κa = 33.13708; right and left κb = 22.09139.
    const size = { height: 80, width: 120 };

    expect([0, 1, 2, 3].map((index) => roundedHandle(size, index))).toEqual([
      { x: 33.13708, y: 0 },
      { x: 0, y: 22.09139 },
      { x: -33.13708, y: 0 },
      { x: 0, y: -22.09139 },
    ]);
  });

  it("passes the middle of its first side on the ellipse: (102.42641, 11.71573)", () => {
    // (60, 40) + (60 · √2/2, −40 · √2/2) (checks).
    const middle = cyclicItem(sidePoints(ellipseCorners({ height: 80, width: 120 }), 1 / 2), 0, {
      x: NaN,
      y: NaN,
    });

    expect(middle.x).toBeCloseTo(102.42641, 5);
    expect(middle.y).toBeCloseTo(11.71573, 5);
  });

  it("draws the segment of a box of width 0, without −0", () => {
    // Step 5: every side runs along x = 0.
    const corners = ellipseCorners({ height: 50, width: 0 });
    const handleXs = corners.flatMap(({ handles }) => [handles.after.x, handles.before.x]);

    expect(corners.map((corner) => corner.point)).toEqual([
      { x: 0, y: 0 },
      { x: 0, y: 25 },
      { x: 0, y: 50 },
      { x: 0, y: 25 },
    ]);

    expect(handleXs.every((x) => Object.is(x, 0))).toBe(true);
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    t: fc.double({ max: 1, min: 0, noNaN: true }),
    width: fc.integer({ max: 1000, min: 1 }),
  })("stays within 0.00028 of the ellipse, outside it", ({ height, t, width }) => {
    // Steps 1–2: in the frame of the unit circle, the distance from the centre is in [1, 1.00028].
    const [a, b] = [width / 2, height / 2];
    const points = sidePoints(ellipseCorners({ height, width }), t);

    for (const point of points) {
      const distance = Math.hypot((point.x - a) / a, (point.y - b) / b);

      expect(distance).toBeGreaterThan(1 - 1e-12);
      expect(distance).toBeLessThan(1.00028);
    }
  });

  test.prop({ height: fc.nat(1000), width: fc.nat(1000) })(
    "[F10.AC1] touches the four sides of its box, never past them",
    ({ height, width }) => {
      // Step 4: each side is monotone on both axes, its extremes at its ends.
      const box = curveBoundingBox(ellipseCorners({ height, width }));

      expect(box.minX).toBeCloseTo(0, 9);
      expect(box.minY).toBeCloseTo(0, 9);
      expect(box.maxX).toBeCloseTo(width, 9);
      expect(box.maxY).toBeCloseTo(height, 9);
    },
  );
});
