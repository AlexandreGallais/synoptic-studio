import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isValidFreeShape } from "./is-valid-free-shape";
import { isValidRectangle } from "./is-valid-rectangle";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

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
      isValidFreeShape(rectangleAsFreeShape({ height: 50, radii: [10, 0, 0, 0], width: 100 })),
    ).toBe(true);

    expect(
      isValidFreeShape(rectangleAsFreeShape({ height: 0, radii: [0, 0, 0, 0], width: 0 })),
    ).toBe(true);

    expect(isValidFreeShape(TRIANGLE)).toBe(true);
  });

  it("refuses a negative or fractional size, and radii not one per point (Q15, ADR-0029)", () => {
    expect(isValidFreeShape({ ...TRIANGLE, width: -1 })).toBe(false);
    expect(isValidFreeShape({ ...TRIANGLE, height: 2.5 })).toBe(false);
    expect(isValidFreeShape({ ...TRIANGLE, radii: [0, 0] })).toBe(false);
  });

  it("refuses fewer than 3 points, fractional points, and points on one line (ADR-0032)", () => {
    expect(
      isValidFreeShape({ ...TRIANGLE, points: TRIANGLE.points.slice(0, 2), radii: [0, 0] }),
    ).toBe(false);

    expect(
      isValidFreeShape({ ...TRIANGLE, points: [{ x: 0.5, y: 0 }, ...TRIANGLE.points.slice(1)] }),
    ).toBe(false);

    // All on the vertical line x = 0: no width can be drawn.
    expect(
      isValidFreeShape({
        ...TRIANGLE,
        points: [
          { x: 0, y: 0 },
          { x: 0, y: 1 },
          { x: 0, y: 2 },
        ],
      }),
    ).toBe(false);

    // All on the horizontal line y = 0: no height can be drawn.
    expect(
      isValidFreeShape({
        ...TRIANGLE,
        points: [
          { x: 0, y: 0 },
          { x: 1, y: 0 },
          { x: 2, y: 0 },
        ],
      }),
    ).toBe(false);
  });

  test.prop({
    height: fc.integer({ max: 1000, min: -10 }),
    radius: fc.oneof(fc.integer({ max: 100, min: -5 }), fc.double({ max: 100, min: 0 })),
    width: fc.integer({ max: 1000, min: -10 }),
  })(
    "accepts the rectangle tool's free shape exactly when the rectangle is valid",
    ({ height, radius, width }) => {
      const rectangle = { height, radii: [radius, 0, radius, 0], width };

      expect(isValidFreeShape(rectangleAsFreeShape(rectangle))).toBe(isValidRectangle(rectangle));
    },
  );
});
