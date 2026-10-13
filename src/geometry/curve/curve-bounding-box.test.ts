import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint, rectangleCorners } from "../../../tests";

import { curveBoundingBox } from "./curve-bounding-box";

import type { CurveCorner } from "../handles";

/**
 * The square 100 × 100, its top side an arch: points 1 and 2 with vertical handles of 40.
 *
 * @param isOn - whether the handles are on
 * @returns its corners
 */
function archedSquare(isOn: boolean): CurveCorner[] {
  return rectangleCorners(100, 100, [0, 0, 0, 0]).map((corner, index) => ({
    ...corner,
    handles: {
      after: { x: 0, y: index === 0 ? -40 : 0 },
      before: { x: 0, y: index === 1 ? -40 : 0 },
      isOn,
      mode: "none",
      tension: 0,
    },
  }));
}

describe("curveBoundingBox (ADR-0046)", () => {
  it("[F12.AC6] is the box of the curve: the arch's top 30 above the points", () => {
    // The arch's middle is (50, −30) (DERIV-cubic-sides): the box is 100 × 130.
    expect(curveBoundingBox(archedSquare(true))).toEqual({
      maxX: 100,
      maxY: 100,
      minX: 0,
      minY: -30,
    });
  });

  it("is the box of the points when the handles are off", () => {
    expect(curveBoundingBox(archedSquare(false))).toEqual({
      maxX: 100,
      maxY: 100,
      minX: 0,
      minY: 0,
    });
  });

  test.prop({ points: fc.array(integerPoint(100), { maxLength: 6, minLength: 3 }) })(
    "holds every point of the shape",
    ({ points }) => {
      const box = curveBoundingBox(
        points.map((point) => ({
          handles: {
            after: { x: 7, y: -9 },
            before: { x: -3, y: 4 },
            isOn: true,
            mode: "none",
            tension: 0,
          },
          point,
          radius: 0,
        })),
      );

      for (const point of points) {
        expect(
          point.x >= box.minX && point.x <= box.maxX && point.y >= box.minY && point.y <= box.maxY,
        ).toBe(true);
      }
    },
  );
});
