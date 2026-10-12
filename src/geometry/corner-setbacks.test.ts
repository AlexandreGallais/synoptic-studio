import { describe, expect, it } from "vitest";

import { cornerSetbacks } from "./corner-setbacks";

import type { Corner } from "./corner";

/** Decimals compared: tangents of doubles, correct to their last bits. */
const DECIMALS = 9;

/**
 * Corners of a square of side 100, clockwise from the top-left vertex.
 *
 * @param radii - radius requested at each corner, in drawing order
 * @returns the four corners
 */
function square(radii: readonly [number, number, number, number]): Corner[] {
  const points = [
    { x: 0, y: 0 },
    { x: 100, y: 0 },
    { x: 100, y: 100 },
    { x: 0, y: 100 },
  ];

  return points.map((point, index) => ({ point, radius: radii[index] ?? 0 }));
}

describe("cornerSetbacks (DERIV-local-radius-clamp step 1)", () => {
  it("equals the radius at the right corners of a square", () => {
    // s = r · tan(π/4) = r.
    const setbacks = cornerSetbacks(square([1000, 0, 50, 10]));

    expect(setbacks[0]).toBeCloseTo(1000, DECIMALS);
    expect(setbacks[1]).toBe(0);
    expect(setbacks[2]).toBeCloseTo(50, DECIMALS);
    expect(setbacks[3]).toBeCloseTo(10, DECIMALS);
  });

  it("is 0 at an aligned vertex, whatever its radius", () => {
    // (50, 0) lies on the top edge: τ = 0, s = 30 · tan(0) = 0.
    const corners = [
      { point: { x: 0, y: 0 }, radius: 0 },
      { point: { x: 50, y: 0 }, radius: 30 },
      { point: { x: 100, y: 0 }, radius: 0 },
      { point: { x: 100, y: 50 }, radius: 0 },
      { point: { x: 0, y: 50 }, radius: 0 },
    ];

    expect(cornerSetbacks(corners)[1]).toBe(0);
  });

  it("is 0 everywhere on a rectangle of size 0, without NaN (Q15)", () => {
    // Zero-length edges: τ = atan2(0, 0) = 0 at every vertex.
    const corners = [
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 0, y: 50 }, radius: 10 },
      { point: { x: 0, y: 50 }, radius: 10 },
    ];

    expect(cornerSetbacks(corners)).toEqual([0, 0, 0, 0]);
  });
});
