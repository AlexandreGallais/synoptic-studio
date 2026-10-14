import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { boundingBox } from "../placement";

import { boxSize } from "./box-size";

describe("boxSize (REF-WIKI-BOUNDING-BOX)", () => {
  it("is 120 × 100 from (0, −20) to (120, 80)", () => {
    expect(boxSize({ maxX: 120, maxY: 80, minX: 0, minY: -20 })).toEqual({
      height: 100,
      width: 120,
    });
  });

  test.prop({ points: fc.array(integerPoint(1000), { minLength: 1 }) })(
    "is the spread of the points on each axis",
    ({ points }) => {
      const xs = points.map((point) => point.x);
      const ys = points.map((point) => point.y);

      expect(boxSize(boundingBox(points))).toEqual({
        height: Math.max(...ys) - Math.min(...ys),
        width: Math.max(...xs) - Math.min(...xs),
      });
    },
  );
});
