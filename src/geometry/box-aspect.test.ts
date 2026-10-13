import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../tests";

import { boxAspect } from "./box-aspect";

describe("boxAspect (DERIV-stretched-polygon step 4)", () => {
  it("divides the height of the bounding box by its width", () => {
    expect(
      boxAspect([
        { x: -1, y: 3 },
        { x: 3, y: 1 },
        { x: 1, y: 0 },
      ]),
    ).toBe(0.75);
  });
});

/** Arbitrary point with integer coordinates. */
const POINT = integerPoint(1000);

/** Points spanning the x axis: two of them differ in x. */
const SPANNING = fc
  .array(POINT, { minLength: 2 })
  .filter((points) => points.some((point) => point.x !== points[0]?.x));

describe("boxAspect (properties)", () => {
  test.prop({
    dx: fc.integer({ max: 1000, min: -1000 }),
    dy: fc.integer({ max: 1000, min: -1000 }),
    points: SPANNING,
  })("does not change when the points move together", ({ dx, dy, points }) => {
    const moved = points.map((point) => ({ x: point.x + dx, y: point.y + dy }));

    expect(boxAspect(moved)).toBe(boxAspect(points));
  });

  test.prop({ points: SPANNING, stretch: fc.integer({ max: 100, min: 1 }) })(
    "grows by the vertical stretch of the points",
    ({ points, stretch }) => {
      const stretched = points.map((point) => ({ x: point.x, y: point.y * stretch }));

      expect(boxAspect(stretched)).toBeCloseTo(boxAspect(points) * stretch, 9);
    },
  );
});
