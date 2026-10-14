import { describe, expect, it } from "vitest";

import { formatCoordinatePair } from "./format-coordinate-pair";

describe("formatCoordinatePair", () => {
  it("writes x then y, separated by one space, at fixed precision (EN-001)", () => {
    // 50 · √3 = 86.6025403784… → 5 decimals.
    expect(formatCoordinatePair({ x: 100, y: 50 * Math.sqrt(3) })).toBe("100 86.60254");
  });

  it("writes a negative zero as 0", () => {
    expect(formatCoordinatePair({ x: -0, y: 0 })).toBe("0 0");
  });
});
