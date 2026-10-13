import { fc, test } from "@fast-check/vitest";
import { describe, expect } from "vitest";

import { curveCornerOf } from "../handles";

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
});
