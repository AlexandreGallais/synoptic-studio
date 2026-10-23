import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { curveContour } from "./curve-contour";
import { shapeStraightBox } from "./shape-straight-box";

import type { Point } from "../../math";
import type { CurveCorner } from "../handles";
import type { Size } from "../layout-box";

/**
 * Corners of a sharp rectangle, handles off, top-left first, clockwise.
 *
 * @param at - its top-left
 * @param size - its width and height, 0 or more
 * @param radius - the radius of each corner
 * @returns its four corners
 */
function rectangleAt(at: Point, size: Size, radius: number): CurveCorner[] {
  const { height, width } = size;

  return [
    { x: at.x, y: at.y },
    { x: at.x + width, y: at.y },
    { x: at.x + width, y: at.y + height },
    { x: at.x, y: at.y + height },
  ].map((point) => ({
    handles: {
      after: { x: 0, y: 0 },
      before: { x: 0, y: 0 },
      isOn: false,
      mode: "angle-and-length",
    },
    point,
    radius,
  }));
}

describe("shapeStraightBox (DERIV-arc-bounding-box)", () => {
  it("is the point a shape 0 × 0 lies on: (10, 20), its contour having no piece", () => {
    // Four corners at (10, 20): every side has no length, so the contour has no piece and the box
    // of its pieces would be empty, [+∞, −∞]; the box of the corners is [10, 10] × [20, 20].
    const corners = rectangleAt({ x: 10, y: 20 }, { height: 0, width: 0 }, 0);

    expect(curveContour(corners)).toEqual([]);

    expect(shapeStraightBox(corners)).toEqual({
      maxX: 10,
      maxY: 20,
      minX: 10,
      minY: 20,
    });
  });

  it("is the rectangle itself, 100 × 50 at (10, 20), its corners rounded by 10 inside it", () => {
    // Each fillet arc stays inside the corner it rounds: the box is [10, 110] × [20, 70].
    const corners = rectangleAt({ x: 10, y: 20 }, { height: 50, width: 100 }, 10);

    expect(shapeStraightBox(corners)).toEqual({
      maxX: 110,
      maxY: 70,
      minX: 10,
      minY: 20,
    });
  });

  it("is the box of the contour as drawn for a shape 10 × 0, whose contour has pieces", () => {
    // 10 × 0 at (10, 20), radius 5: its contour runs from (10, 20) to (20, 20) and back, its
    // fillets without room: [10, 20] × [20, 20].
    const corners = rectangleAt({ x: 10, y: 20 }, { height: 0, width: 10 }, 5);

    expect(shapeStraightBox(corners)).toEqual({ maxX: 20, maxY: 20, minX: 10, minY: 20 });
  });
});

describe("shapeStraightBox (properties)", () => {
  test.prop({
    at: integerPoint(100),
    height: fc.integer({ max: 50, min: 0 }),
    width: fc.integer({ max: 50, min: 0 }),
  })(
    "holds a sharp rectangle's corners exactly, 0 wide or 0 tall included",
    ({ at, height, width }) => {
      expect(shapeStraightBox(rectangleAt(at, { height, width }, 0))).toEqual({
        maxX: at.x + width,
        maxY: at.y + height,
        minX: at.x,
        minY: at.y,
      });
    },
  );
});
