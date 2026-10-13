import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bentRectangle } from "../../../tests";

import { cornerSides } from "./corner-sides";
import { sideFactors } from "./side-factors";

describe("sideFactors (ADR-0007, ADR-0047)", () => {
  it("shares a straight side asked too much, and leaves a curve whole", () => {
    // The top side, 120: setbacks 80 + 80 ask 160, factor 0.75; the curved right side keeps 1.
    const sides = cornerSides(bentRectangle(40, [0, 0, 0, 0]));
    const setbacks = [
      [0, 80],
      [80, 0],
      [0, 0],
      [0, 0],
    ] as const;

    expect(sideFactors(sides, setbacks).slice(0, 2)).toEqual([0.75, 1]);
  });

  test.prop({ setback: fc.integer({ max: 200, min: 0 }) })(
    "gives factors from 0 to 1",
    ({ setback }) => {
      const sides = cornerSides(bentRectangle(40, [0, 0, 0, 0]));
      const factors = sideFactors(
        sides,
        sides.map(() => [setback, setback] as const),
      );

      expect(factors.every((factor) => factor >= 0 && factor <= 1)).toBe(true);
    },
  );
});
