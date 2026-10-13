import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS, TWO_CURVES } from "../../../tests";

import { distanceFromSide } from "./distance-from-side";
import { orientedSides } from "./oriented-sides";

describe("distanceFromSide (DERIV-fillet-beside-curve steps 3 and 10)", () => {
  it("measures from a straight side along its normal, positive inside the corner", () => {
    const [before] = orientedSides(...LINE_AND_THIRDS);

    expect([
      distanceFromSide(before, { x: 90, y: 10 }, 1),
      distanceFromSide(before, { x: 90, y: -4 }, 1),
    ]).toEqual([10, -4]);
  });

  test.prop({ x: fc.integer({ max: 95, min: 5 }), y: fc.integer({ max: 40, min: -40 }) })(
    "gives the same distance from a straight cubic read as a curve",
    ({ x, y }) => {
      const [line] = orientedSides(...LINE_AND_THIRDS);
      const [curve] = orientedSides(...TWO_CURVES);

      expect(distanceFromSide(curve, { x, y }, 1)).toBeCloseTo(
        distanceFromSide(line, { x, y }, 1),
        9,
      );
    },
  );
});
