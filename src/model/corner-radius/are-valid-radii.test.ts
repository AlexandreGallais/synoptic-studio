import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { areValidRadii } from "./are-valid-radii";

describe("areValidRadii", () => {
  it("[F03.AC1] accepts one integer radius ≥ 0 per vertex, different ones included", () => {
    expect(areValidRadii([50, 0, 0, 0], 4)).toBe(true);
    expect(areValidRadii([0, 0, 0], 3)).toBe(true);
  });

  it("[F03.AC1] refuses a number of radii other than the number of vertices", () => {
    expect(areValidRadii([10, 10, 10], 4)).toBe(false);
    expect(areValidRadii([10, 10, 10, 10, 10], 4)).toBe(false);
    expect(areValidRadii([], 3)).toBe(false);
  });

  it("[F03.AC1] refuses a negative or fractional radius (ADR-0003)", () => {
    expect(areValidRadii([10, -1, 10, 10], 4)).toBe(false);
    expect(areValidRadii([10, 10, 2.5, 10], 4)).toBe(false);
  });

  it("accepts a radius larger than the shape: it is clamped, not refused (ADR-0007)", () => {
    expect(areValidRadii([1000, 0, 0, 0], 4)).toBe(true);
  });

  test.prop([fc.array(fc.nat({ max: 100_000 }), { maxLength: 12, minLength: 3 })])(
    "accepts any list of integers ≥ 0 whose length is the count",
    (radii) => {
      expect(areValidRadii(radii, radii.length)).toBe(true);
      expect(areValidRadii(radii, radii.length + 1)).toBe(false);
    },
  );
});
