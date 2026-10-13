import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_QUARTER, TWO_CURVES } from "../../../tests";

import { footOnSide } from "./foot-on-side";
import { orientedSides } from "./oriented-sides";

describe("footOnSide (REF-EUCLID-III18, REF-POMAX-BEZIER)", () => {
  it("projects a point on a straight cubic at the share of its length", () => {
    // (90, 10) onto the cubic from (100, 0) to (0, 0): x 90 is a tenth of the way.
    const [before] = orientedSides(...TWO_CURVES);

    expect(footOnSide(before, { x: 90, y: 10 }, 1)).toBeCloseTo(0.1, 12);
  });

  it("gives the end when the point lies beyond it", () => {
    const [before] = orientedSides(...TWO_CURVES);

    expect(footOnSide(before, { x: 0, y: 10 }, 0.5)).toBeCloseTo(0.5, 12);
  });

  test.prop({ angle: fc.double({ max: 1.5, min: 0.05, noNaN: true }) })(
    "finds where a point on the quarter circle's normal stands",
    ({ angle }) => {
      // The quarter circle of centre (−100, 0): the point at radius 50 along the angle, its foot
      // at the same angle on the cubic, within the cubic's error to a circle.
      const [, after] = orientedSides(...LINE_AND_QUARTER);
      const point = { x: -100 + 50 * Math.cos(angle), y: 50 * Math.sin(angle) };
      const t = footOnSide(after, point, 1);

      expect(t).toBeGreaterThan(0);
      expect(t).toBeLessThan(1);
    },
  );
});
