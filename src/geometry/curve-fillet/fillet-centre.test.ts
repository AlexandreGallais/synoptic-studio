import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS } from "../../../tests";
import { norm, subtract } from "../../math";
import { cubicPoint } from "../placement";

import { filletCentre } from "./fillet-centre";
import { orientedSides } from "./oriented-sides";

describe("filletCentre (DERIV-fillet-beside-curve step 3)", () => {
  it("stands the radius away from the curve, towards the corner's inside", () => {
    // The cubic down x = 100 at t 0.2 is (100, 10); 10 to its left, (90, 10).
    const [, after] = orientedSides(...LINE_AND_THIRDS);

    expect(filletCentre(after, 10, 0.2).x).toBeCloseTo(90, 12);
    expect(filletCentre(after, 10, 0.2).y).toBeCloseTo(10, 12);
  });

  test.prop({
    radius: fc.integer({ max: 100, min: 0 }),
    t: fc.double({ max: 1, min: 0, noNaN: true }),
  })("is the radius away from the side's point", ({ radius, t }) => {
    const [, after] = orientedSides(...LINE_AND_THIRDS);

    const centre = filletCentre(after, radius, t);
    const touched = cubicPoint(after.cubic, t);

    expect(norm(subtract(centre, touched))).toBeCloseTo(radius, 9);
  });
});
