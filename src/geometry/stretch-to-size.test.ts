import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boundingBox } from "./bounding-box";
import { stretchToSize } from "./stretch-to-size";

/** Integer coordinates around the origin. */
const COORDINATE = fc.integer({ max: 1000, min: -1000 });

/** A point with integer coordinates. */
const ANY_POINT = fc.record({ x: COORDINATE, y: COORDINATE });

describe("stretchToSize", () => {
  it("[F09.AC2] stretches each axis by its own factor, the top-left of the box fixed", () => {
    // DERIV-free-shape-stretch check rows: o = (0, 0), n = 2 × 2, sx = 5, sy = 2.
    expect(
      stretchToSize(
        [
          { x: 1, y: 0 },
          { x: 2, y: 2 },
          { x: 0, y: 2 },
        ],
        { height: 4, width: 10 },
      ),
    ).toEqual([
      { x: 5, y: 0 },
      { x: 10, y: 4 },
      { x: 0, y: 4 },
    ]);

    // o = (3, 3), n = 2 × 4, sx = sy = 2: the top-left stays at (3, 3).
    expect(
      stretchToSize(
        [
          { x: 3, y: 3 },
          { x: 5, y: 3 },
          { x: 5, y: 7 },
        ],
        { height: 8, width: 4 },
      ),
    ).toEqual([
      { x: 3, y: 3 },
      { x: 7, y: 3 },
      { x: 7, y: 11 },
    ]);
  });

  it("[F09.AC2] puts the far edge exactly on the width, multiplying first", () => {
    // (29 / 7) · 7 would give 29.000000000000004; 29 · 7 / 7 gives 29.
    expect(
      stretchToSize(
        [
          { x: 0, y: 0 },
          { x: 7, y: 0 },
          { x: 7, y: 1 },
        ],
        { height: 1, width: 29 },
      ),
    ).toEqual([
      { x: 0, y: 0 },
      { x: 29, y: 0 },
      { x: 29, y: 1 },
    ]);
  });

  it("[F09.AC3] draws every point on one line at size 0, and gives them back exactly", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 1, y: 0 },
      { x: 1, y: 1 },
      { x: 0, y: 1 },
    ];

    expect(stretchToSize(points, { height: 50, width: 0 }).map((point) => point.x)).toEqual([
      0, 0, 0, 0,
    ]);

    expect(stretchToSize(points, { height: 50, width: 100 })).toEqual([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ]);
  });

  test.prop({
    height: fc.nat({ max: 10_000 }),
    points: fc.array(ANY_POINT, { minLength: 2 }),
    width: fc.nat({ max: 10_000 }),
  })(
    "[F09.AC2] gives a box of exactly the size asked, its top-left where it was",
    ({ height, points, width }) => {
      const before = boundingBox(points);

      fc.pre(before.maxX > before.minX && before.maxY > before.minY);

      const after = boundingBox(stretchToSize(points, { height, width }));

      expect(after).toEqual({
        maxX: before.minX + width,
        maxY: before.minY + height,
        minX: before.minX,
        minY: before.minY,
      });
    },
  );
});
