import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bisectionBracket } from "./bisection-bracket";

describe("bisectionBracket (REF-MATHWORLD-BISECTION)", () => {
  it("closes on √2 from below: its lower end squared stays below 2", () => {
    const [low, high] = bisectionBracket((value) => value * value < 2, [0, 2], 60);

    expect(low * low).toBeLessThan(2);
    expect(high).toBeCloseTo(Math.SQRT2, 15);
  });

  test.prop({ boundary: fc.double({ max: 999, min: -999, noNaN: true }) })(
    "keeps its lower end where the predicate holds",
    (input) => {
      const [low] = bisectionBracket((value) => value < input.boundary, [-1000, 1000], 60);

      expect(low < input.boundary).toBe(true);
    },
  );

  it("halves its width at each step: 10 halvings of [0, 1024] leave a width of 1", () => {
    const [low, high] = bisectionBracket((value) => value < 300.5, [0, 1024], 10);

    expect([low, high]).toEqual([300, 301]);
  });
});
