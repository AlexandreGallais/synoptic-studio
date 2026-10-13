import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS } from "../../../tests";
import { orientedSides } from "../fillet-side";

import { isValidTangency } from "./is-valid-tangency";

describe("isValidTangency (ADR-0047, DERIV-fillet-beside-curve step 11)", () => {
  it("takes the fillet of radius 10 at (90, 10), not a centre off the radius, past the limit, or crossing", () => {
    const sides = orientedSides(...LINE_AND_THIRDS);
    const [line, curve] = sides;

    expect(
      isValidTangency(sides, 10, { centre: { x: 90, y: 10 }, measured: 0.1, running: 0.2 }),
    ).toBe(true);

    expect(
      isValidTangency(sides, 10, { centre: { x: 90, y: 12 }, measured: 0.1, running: 0.2 }),
    ).toBe(false);

    expect(
      isValidTangency([{ ...line, limit: 0.05 }, curve], 10, {
        centre: { x: 90, y: 10 },
        measured: 0.1,
        running: 0.2,
      }),
    ).toBe(false);

    // A centre 10 below the straight side but 5 from the curve: the circle crosses it.
    expect(
      isValidTangency(sides, 10, { centre: { x: 95, y: 10 }, measured: 0.05, running: 0.2 }),
    ).toBe(false);
  });

  test.prop({ radius: fc.integer({ max: 49, min: 1 }) })(
    "takes every fillet of a right corner whose sides hold it",
    ({ radius }) => {
      const sides = orientedSides(...LINE_AND_THIRDS);
      const candidate = {
        centre: { x: 100 - radius, y: radius },
        measured: radius / 100,
        running: radius / 50,
      };

      expect(isValidTangency(sides, radius, candidate)).toBe(true);
    },
  );
});
