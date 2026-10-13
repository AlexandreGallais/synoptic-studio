import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleCorners } from "../../../tests";
import { NO_HANDLES, curveCornerOf } from "../handles";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";

describe("freeShapeAroundCorners", () => {
  const coordinate = fc.double({ max: 1e5, min: -1e5, noNaN: true });
  const corner = fc.record({
    point: fc.record({ x: coordinate, y: coordinate }),
    radius: fc.nat({ max: 100 }),
  });

  test.prop({ corners: fc.array(corner, { maxLength: 8, minLength: 3 }) })(
    "[F08.AC1] draws the corners where they are, bit for bit, at the size of their box",
    ({ corners }) => {
      const curved = corners.map((stored) => curveCornerOf(stored));

      expect(freeShapeCorners(freeShapeAroundCorners(curved, 0))).toEqual(curved);
    },
  );

  it("[F12.AC6] takes the box of the curve: a square whose top arches 30 up is 100 × 130", () => {
    // The arch tops at (50, −30) (DERIV-cubic-sides); at height 100 the whole shape shrinks so that
    // the arch fits: the bottom at y = −30 + 100 · 130 / 130 = 70.
    const shape = freeShapeAroundCorners(
      rectangleCorners(100, 100, [0, 0, 0, 0]).map((stored, index) => ({
        ...stored,
        handles: {
          ...NO_HANDLES,
          after: { x: 0, y: index === 0 ? -40 : 0 },
          before: { x: 0, y: index === 1 ? -40 : 0 },
          isOn: index < 2,
          mode: "none" as const,
        },
      })),
      0,
    );

    expect([shape.x, shape.y, shape.width, shape.height]).toEqual([0, -30, 100, 130]);
    expect(freeShapeCorners({ ...shape, height: 100 })[3]?.point).toEqual({ x: 0, y: 70 });
  });
});
