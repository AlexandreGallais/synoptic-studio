import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { quadraticRoots } from "./quadratic-roots";

describe("quadraticRoots (REF-WIKI-QUADRATIC)", () => {
  it("finds the small root of x² − 1634x + 2 without cancellation", () => {
    // Wikipedia's example: x₂ = 2 / x₁ ≈ 1.223991125 · 10⁻³.
    const [large, small] = quadraticRoots({ a: 1, b: -1634, c: 2 });

    expect(large).toBeCloseTo(1633.998776, 5);
    expect(small).toBeCloseTo(1.223991125e-3, 12);
  });

  it("leaves the linear root when a is 0, the other infinite", () => {
    // 240 t − 120 = 0: t = ½ (the arch's y).
    expect(quadraticRoots({ a: 0, b: 240, c: -120 })).toEqual([-Infinity, 0.5]);
  });

  it("gives NaN when there is no real root", () => {
    expect(quadraticRoots({ a: 1, b: 0, c: 1 }).every((t) => Number.isNaN(t))).toBe(true);
  });

  test.prop({
    a: fc.integer({ max: 100, min: 1 }),
    r: fc.integer({ max: 100, min: -100 }),
    s: fc.integer({ max: 100, min: -100 }),
  })("finds both roots of a(t − r)(t − s), but a double root at 0", ({ a, r, s }) => {
    // At r = s = 0, q = 0 and c / q is NaN: in a cubic, t = 0 is an end, counted anyway.
    fc.pre(r !== 0 || s !== 0);

    const roots = quadraticRoots({ a, b: -a * (r + s), c: a * r * s }).toSorted((x, y) => x - y);

    expect(roots[0]).toBeCloseTo(Math.min(r, s), 6);
    expect(roots[1]).toBeCloseTo(Math.max(r, s), 6);
  });
});
