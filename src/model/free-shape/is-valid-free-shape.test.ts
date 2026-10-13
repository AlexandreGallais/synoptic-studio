import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { freeShapeOf } from "../../../tests";
import { isValidRectangle, rectangleAsFreeShape } from "../rectangle";

import { isValidFreeShape } from "./is-valid-free-shape";

/** A triangle stored in a 2 × 2 box, drawn 10 × 4. */
const TRIANGLE = {
  height: 4,
  points: [
    { x: 1, y: 0 },
    { x: 2, y: 2 },
    { x: 0, y: 2 },
  ],
  radii: [0, 0, 0],
  width: 10,
};

describe("isValidFreeShape", () => {
  it("[F09.AC1] accepts the free shape of the rectangle tool, size 0 included (Q15)", () => {
    expect(
      isValidFreeShape(
        rectangleAsFreeShape({
          angle: 0,
          x: 0,
          y: 0,
          height: 50,
          radii: [10, 0, 0, 0],
          width: 100,
        }),
      ),
    ).toBe(true);

    expect(
      isValidFreeShape(
        rectangleAsFreeShape({ angle: 0, x: 0, y: 0, height: 0, radii: [0, 0, 0, 0], width: 0 }),
      ),
    ).toBe(true);

    expect(isValidFreeShape(freeShapeOf(TRIANGLE))).toBe(true);
  });

  it("[F08.AC1] accepts fractional sizes and points, as a converted polygon leaves them (Q27)", () => {
    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, height: 86.60254 }))).toBe(true);

    const halfPoint = [{ x: 0.5, y: 0 }, ...TRIANGLE.points.slice(1)];

    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, points: halfPoint }))).toBe(true);
  });

  it("refuses a negative or infinite size, a radius not an integer ≥ 0 (Q15, ADR-0029)", () => {
    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, width: -1 }))).toBe(false);
    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, height: Infinity }))).toBe(false);
    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, radii: [0, -1, 0] }))).toBe(false);
    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, radii: [0, 2.5, 0] }))).toBe(false);
  });

  it("refuses fewer than 3 points, a point not finite, and points on one line (ADR-0032)", () => {
    const twoPoints = TRIANGLE.points.slice(0, 2);
    const notFinite = [{ x: NaN, y: 0 }, ...TRIANGLE.points.slice(1)];

    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, points: twoPoints, radii: [0, 0] }))).toBe(
      false,
    );

    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, points: notFinite }))).toBe(false);

    // An infinite point still leaves a box of some width and height: only finiteness refuses it.
    const infinite = [{ x: Infinity, y: 0 }, ...TRIANGLE.points.slice(1)];

    expect(isValidFreeShape(freeShapeOf({ ...TRIANGLE, points: infinite }))).toBe(false);

    // All on the vertical line x = 0: no width can be drawn.
    expect(
      isValidFreeShape(
        freeShapeOf({
          ...TRIANGLE,
          points: [
            { x: 0, y: 0 },
            { x: 0, y: 1 },
            { x: 0, y: 2 },
          ],
        }),
      ),
    ).toBe(false);

    // All on the horizontal line y = 0: no height can be drawn.
    expect(
      isValidFreeShape(
        freeShapeOf({
          ...TRIANGLE,
          points: [
            { x: 0, y: 0 },
            { x: 1, y: 0 },
            { x: 2, y: 0 },
          ],
        }),
      ),
    ).toBe(false);
  });

  test.prop({
    height: fc.integer({ max: 1000, min: -10 }),
    radius: fc.oneof(fc.integer({ max: 100, min: -5 }), fc.double({ max: 100, min: 0 })),
    width: fc.integer({ max: 1000, min: -10 }),
  })(
    "accepts the rectangle tool's free shape exactly when the rectangle is valid",
    ({ height, radius, width }) => {
      const rectangle = { angle: 0, height, radii: [radius, 0, radius, 0], width, x: 0, y: 0 };

      expect(isValidFreeShape(rectangleAsFreeShape(rectangle))).toBe(isValidRectangle(rectangle));
    },
  );

  it.each([
    { angle: 360, isValid: false, x: 0, y: 0 },
    { angle: -1, isValid: false, x: 0, y: 0 },
    { angle: 0, isValid: false, x: NaN, y: 0 },
    // A fractional position is allowed (ADR-0035).
    { angle: 45, isValid: true, x: 0.5, y: -3.25 },
  ])("[F04.AC3] checks the angle $angle and the position ($x, $y)", ({ isValid, ...placement }) => {
    expect(isValidFreeShape({ ...freeShapeOf(TRIANGLE), ...placement })).toBe(isValid);
  });
});
