import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS, integerPoint, sideOf } from "../../../tests";
import { straightCubic } from "../curve";

import { orientedSides } from "./oriented-sides";

describe("orientedSides (DERIV-fillet-beside-curve step 2)", () => {
  it("turns the two sides of a right corner towards its inside", () => {
    // a = (−1, 0), b = (0, 1): a⊥ · b = −1, so σ = −1; n_A = −a⊥ = (0, 1), down towards the curve.
    const [before, after] = orientedSides(...LINE_AND_THIRDS);

    expect([before.sign, after.sign]).toEqual([-1, 1]);
  });

  it("has no inside for sides leaving along one line", () => {
    const corner = { x: 0, y: 0 };
    const [before] = orientedSides(
      sideOf(straightCubic(corner, { x: -10, y: 0 }), true),
      sideOf(straightCubic(corner, { x: 10, y: 0 }), false),
    );

    expect(before.sign).toBe(0);
  });

  test.prop({ a: integerPoint(50), b: integerPoint(50) })(
    "gives the two sides opposite signs",
    ({ a, b }) => {
      const corner = { x: 0, y: 0 };
      const [before, after] = orientedSides(
        sideOf(straightCubic(corner, a), true),
        sideOf(straightCubic(corner, b), false),
      );

      expect(before.sign + after.sign).toBe(0);
    },
  );
});
