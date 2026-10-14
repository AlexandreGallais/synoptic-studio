import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_HANDLE_OFF, LINE_AND_THIRDS, TWO_CURVES, sideOf } from "../../../tests";
import { norm, subtract } from "../../math";
import { straightCubic } from "../curve";
import { orientedSides } from "../fillet-side";
import { cubicPoint } from "../placement";

import { filletTangency } from "./fillet-tangency";

describe("filletTangency (DERIV-fillet-beside-curve, checks 1a, 1b and 3)", () => {
  it("[F12.AC7] touches a straight cubic as the fillet of a segment: centre (90, 10)", () => {
    // Radius 10 at a right corner: the setback is 10, a tenth of the side, a fifth of the cubic.
    const tangency = filletTangency(orientedSides(...LINE_AND_THIRDS), 10);

    expect(tangency?.centre.x).toBeCloseTo(90, 9);
    expect(tangency?.centre.y).toBeCloseTo(10, 9);
    expect(tangency?.after).toBeCloseTo(0.2, 9);
    expect(tangency?.before).toBeCloseTo(0.1, 9);
  });

  it("[F12.AC7] finds the same circle when the corner's handle is off, at t² (2 − t) = 0.2", () => {
    const tangency = filletTangency(orientedSides(...LINE_AND_HANDLE_OFF), 10);

    expect(tangency?.centre.x).toBeCloseTo(90, 9);
    expect(tangency?.after).toBeCloseTo(0.347938, 6);
  });

  it("[F12.AC7] touches two curved sides", () => {
    const tangency = filletTangency(orientedSides(...TWO_CURVES), 10);

    expect([tangency?.centre.y, tangency?.before, tangency?.after]).toEqual([
      expect.closeTo(10, 9),
      expect.closeTo(0.1, 9),
      expect.closeTo(0.2, 9),
    ]);
  });

  it("finds none for a radius the sides cannot hold", () => {
    expect(filletTangency(orientedSides(...LINE_AND_THIRDS), 1000)).toBeUndefined();
  });

  test.prop({ radius: fc.integer({ max: 40, min: 1 }) })(
    "touches the curve: its centre the radius away from the point touched",
    ({ radius }) => {
      const sides = orientedSides(...LINE_AND_HANDLE_OFF);
      const tangency = filletTangency(sides, radius);
      const touched = cubicPoint(sides[1].cubic, tangency?.after ?? 0);

      expect(norm(subtract(tangency?.centre ?? touched, touched))).toBeCloseTo(radius, 9);
    },
  );

  it("[F12.AC7] touches an equilateral corner at the setback 10√3 = 17.32051", () => {
    // Corner (100, 0), a side to (0, 0), a straight cubic to (50, 50√3), radius 10: D (82.67949, 10),
    // the curve touched at (91.33975, 15), t = √3/10 on both.
    const corner = { x: 100, y: 0 };
    const apex = { x: 50, y: 50 * Math.sqrt(3) };
    const sides = orientedSides(
      sideOf(straightCubic(corner, { x: 0, y: 0 }), true),
      sideOf(straightCubic(corner, apex), false),
    );
    const tangency = filletTangency(sides, 10);

    expect([tangency?.centre.x, tangency?.centre.y, tangency?.after, tangency?.before]).toEqual([
      expect.closeTo(82.67949, 5),
      expect.closeTo(10, 9),
      expect.closeTo(Math.sqrt(3) / 10, 9),
      expect.closeTo(Math.sqrt(3) / 10, 9),
    ]);
  });
});
