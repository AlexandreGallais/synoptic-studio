import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { stretchedSideDirections } from "./stretched-side-directions";

/** Decimals compared: atan2, acos and the sums of cosines are correct to the last bits. */
const DECIMALS = 12;

describe("stretchedSideDirections (DERIV-stretched-polygon steps 1–2)", () => {
  it("gives the regular directions 2πj/n at λ = 1, the odd closure included", () => {
    // Hexagon: π/3, 2π/3; pentagon: 2π/5, then the closure acos(−1/2 − cos 2π/5) = 4π/5.
    const hexagon = stretchedSideDirections(6, 1);
    const pentagon = stretchedSideDirections(5, 1);

    expect(hexagon).toHaveLength(2);
    expect(hexagon[0]).toBeCloseTo(Math.PI / 3, DECIMALS);
    expect(hexagon[1]).toBeCloseTo((2 * Math.PI) / 3, DECIMALS);
    expect(pentagon).toHaveLength(2);
    expect(pentagon[0]).toBeCloseTo((2 * Math.PI) / 5, DECIMALS);
    expect(pentagon[1]).toBeCloseTo((4 * Math.PI) / 5, DECIMALS);
  });

  it("scales the directions vertically: the hexagon's first side has tan φ₁ = λ √3", () => {
    // θ₁ = π/3: (cos, λ sin) = (1/2, λ √3/2), so tan φ₁ = λ √3; φ₂ = π − φ₁ (step 2).
    const [first = 0, second] = stretchedSideDirections(6, 2);

    expect(Math.tan(first)).toBeCloseTo(2 * Math.sqrt(3), DECIMALS);
    expect(second).toBeCloseTo(Math.PI - first, DECIMALS);
  });

  it("gives the triangle and the square one shape whatever λ: 2π/3 and π/2", () => {
    expect(stretchedSideDirections(3, 5)[0]).toBeCloseTo((2 * Math.PI) / 3, DECIMALS);
    expect(stretchedSideDirections(4, 0.2)[0]).toBeCloseTo(Math.PI / 2, DECIMALS);
  });

  it("gives NaN when no side closes an odd polygon: a pentagon flattened by λ = e⁻³", () => {
    // φ₁ = atan2(e⁻³ sin 72°, cos 72°) ≈ 8.7°: −1/2 − cos φ₁ ≈ −1.49, outside [−1, 1].
    expect(stretchedSideDirections(5, Math.exp(-3))[1]).toBeNaN();
  });
});

/** Vertical scale `λ = e^t`, `t` from −3 to 3 in steps of 1/100. */
const SCALE = fc.integer({ max: 300, min: -300 }).map((step) => Math.exp(step / 100));

describe("stretchedSideDirections (properties)", () => {
  test.prop({ half: fc.integer({ max: 6, min: 2 }), scale: SCALE })(
    "gives an even polygon rising directions in (0, π), symmetric about the vertical",
    ({ half, scale }) => {
      // n = 2m: θⱼ and θₘ₋ⱼ are symmetric about π/2, and so are their scaled directions: φⱼ + φₘ₋ⱼ = π.
      const directions = stretchedSideDirections(2 * half, scale);

      expect(directions).toHaveLength(half - 1);

      for (const [index, direction] of directions.entries()) {
        expect(direction).toBeGreaterThan(0);
        expect(direction).toBeLessThan(Math.PI);
        expect(direction).toBeGreaterThan(directions[index - 1] ?? 0);
        expect(direction + (directions[half - 2 - index] ?? 0)).toBeCloseTo(Math.PI, DECIMALS);
      }
    },
  );
});
