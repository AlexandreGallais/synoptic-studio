import { describe, expect, it } from "vitest";

import { isValidRectangle } from "./is-valid-rectangle";

describe("isValidRectangle", () => {
  it("accepts positive integer sizes", () => {
    expect(isValidRectangle({ height: 50, radii: [10, 10, 10, 10], width: 100 })).toBe(true);
  });

  it("accepts a size of 0 (Q15)", () => {
    expect(isValidRectangle({ height: 0, radii: [0, 0, 0, 0], width: 0 })).toBe(true);
  });

  it("rejects a negative width or height (Q15)", () => {
    expect(isValidRectangle({ height: 50, radii: [0, 0, 0, 0], width: -1 })).toBe(false);
    expect(isValidRectangle({ height: -1, radii: [0, 0, 0, 0], width: 50 })).toBe(false);
  });

  it("rejects a non-integer size (ADR-0003)", () => {
    expect(isValidRectangle({ height: 50, radii: [0, 0, 0, 0], width: 10.5 })).toBe(false);
  });

  it("rejects a negative or non-integer radius (ADR-0003)", () => {
    expect(isValidRectangle({ height: 50, radii: [-1, -1, -1, -1], width: 100 })).toBe(false);
    expect(isValidRectangle({ height: 50, radii: [2.5, 2.5, 2.5, 2.5], width: 100 })).toBe(false);
  });

  it("accepts a radius larger than the rectangle: it is clamped, not refused (ADR-0007)", () => {
    expect(isValidRectangle({ height: 25, radii: [1000, 1000, 1000, 1000], width: 100 })).toBe(
      true,
    );
  });

  it("[F02.AC5] sets no upper limit: the cap of 100 000 belongs to the interface (Q20)", () => {
    expect(
      isValidRectangle({
        height: 5_000_000,
        radii: [250_000, 250_000, 250_000, 250_000],
        width: 1_000_000,
      }),
    ).toBe(true);
  });

  it("[F03.AC1] accepts different radii, and refuses other than four of them (ADR-0029)", () => {
    expect(isValidRectangle({ height: 50, radii: [50, 0, 10, 0], width: 100 })).toBe(true);
    expect(isValidRectangle({ height: 50, radii: [10, 10, 10], width: 100 })).toBe(false);
    expect(isValidRectangle({ height: 50, radii: [10, 10, 10, 10, 10], width: 100 })).toBe(false);
  });
});
