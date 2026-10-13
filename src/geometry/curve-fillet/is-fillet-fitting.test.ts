import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS } from "../../../tests";

import { isFilletFitting } from "./is-fillet-fitting";
import { orientedSides } from "./oriented-sides";

describe("isFilletFitting (DERIV-fillet-beside-curve step 11)", () => {
  it("fits a radius the sides hold, not one larger than both", () => {
    const sides = orientedSides(...LINE_AND_THIRDS);

    expect([isFilletFitting(sides, 10), isFilletFitting(sides, 60)]).toEqual([true, false]);
  });

  it("refuses a fillet reaching past a straight side's limit", () => {
    // The setback of radius 10 is 10, a tenth of the side: a limit of 0.05 is too short.
    const [before, after] = LINE_AND_THIRDS;

    expect(isFilletFitting(orientedSides({ ...before, limit: 0.05 }, after), 10)).toBe(false);
  });

  test.prop({ radius: fc.integer({ max: 50, min: 1 }) })(
    "fits every radius up to the shorter side, the curve 50 long",
    ({ radius }) => {
      expect(isFilletFitting(orientedSides(...LINE_AND_THIRDS), radius)).toBe(true);
    },
  );
});
