import { describe, expect, it } from "vitest";

import { turningAngles } from "./turning-angles";

/** Decimals compared: atan2 of exact integers is correct to the last bits of a double. */
const DECIMALS = 12;

describe("turningAngles", () => {
  it("gives a clockwise quarter turn at every corner of the rectangle of US-001", () => {
    // The first vertex sees the last edge (0, 50) → (0, 0): cyclic contour (Q11).
    const angles = turningAngles([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ]);

    expect(angles).toHaveLength(4);

    for (const angle of angles) {
      expect(angle).toBeCloseTo(Math.PI / 2, DECIMALS);
    }
  });

  it("gives no angle for an empty contour", () => {
    expect(turningAngles([])).toEqual([]);
  });
});
