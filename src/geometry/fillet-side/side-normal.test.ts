import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_QUARTER } from "../../../tests";
import { dot } from "../../math";

import { orientedSides } from "./oriented-sides";
import { sideNormal } from "./side-normal";
import { sideTangent } from "./side-tangent";

describe("sideNormal (DERIV-fillet-beside-curve steps 2 and 3)", () => {
  it("points from the straight side down towards the curve, from the curve left towards the line", () => {
    const [before, after] = orientedSides(...LINE_AND_QUARTER);

    expect(sideNormal(before, 0.5).x).toBeCloseTo(0, 12);
    expect(sideNormal(before, 0.5).y).toBeCloseTo(1, 12);
    expect(sideNormal(after, 0).x).toBeCloseTo(-1, 12);
  });

  test.prop({ t: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "is perpendicular to the side",
    ({ t }) => {
      const [, after] = orientedSides(...LINE_AND_QUARTER);

      expect(dot(sideNormal(after, t), sideTangent(after.cubic, t))).toBeCloseTo(0, 12);
    },
  );
});
