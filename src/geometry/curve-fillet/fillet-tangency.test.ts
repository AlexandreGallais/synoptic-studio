import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_HANDLE_OFF, LINE_AND_THIRDS, TWO_CURVES } from "../../../tests";
import { norm, subtract } from "../../math";
import { cubicPoint } from "../placement";

import { filletTangency } from "./fillet-tangency";
import { orientedSides } from "./oriented-sides";

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
    expect(filletTangency(orientedSides(...TWO_CURVES), 10)?.centre.y).toBeCloseTo(10, 9);
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
});
