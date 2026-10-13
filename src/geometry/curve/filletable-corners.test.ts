import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleCorners } from "../../../tests";

import { filletableCorners } from "./filletable-corners";

import type { CurveCorner, Handles } from "../handles";

/** Handles off: the drawing of E01. */
const OFF: Handles = {
  after: { x: 0, y: 0 },
  before: { x: 0, y: 0 },
  isOn: false,
  mode: "angle-and-length",
};

/**
 * The rectangle 120 × 80, radius 50 at every corner, point 1 with the given handles.
 *
 * @param first - handles of point 1
 * @returns its corners
 */
function rectangleWithFirst(first: Handles): CurveCorner[] {
  return rectangleCorners(120, 80, [50, 50, 50, 50]).map((corner, index) => ({
    ...corner,
    handles: index === 0 ? first : OFF,
  }));
}

describe("filletableCorners (ADR-0046)", () => {
  it("[F12.AC1] keeps no fillet at a point in handles mode, its zero handles leaving its sides straight", () => {
    const radii = filletableCorners(rectangleWithFirst({ ...OFF, isOn: true })).map(
      (corner) => corner.radius,
    );

    expect(radii).toEqual([0, 50, 50, 50]);
  });

  it("[F12.AC3] keeps no fillet next to a curved side, at both of its ends", () => {
    // Point 1's handle after (20, −20) bends the side to point 2; its handle before bends the side from point 4.
    const corners = rectangleWithFirst({
      ...OFF,
      after: { x: 20, y: -20 },
      before: { x: -20, y: 20 },
      isOn: true,
    });

    expect(filletableCorners(corners).map((corner) => corner.radius)).toEqual([0, 0, 50, 0]);
  });

  test.prop({ radius: fc.nat({ max: 1000 }) })(
    "[F12.AC5] changes nothing when every handle is off",
    ({ radius }) => {
      const corners = rectangleCorners(120, 80, [radius, radius, radius, radius]).map((corner) => ({
        ...corner,
        handles: OFF,
      }));

      expect(filletableCorners(corners)).toEqual(corners);
    },
  );
});
