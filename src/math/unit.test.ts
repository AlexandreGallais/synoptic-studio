import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { norm } from "./norm";
import { unit } from "./unit";

/** Nonzero vector with integer components. */
const NONZERO = fc
  .record({ x: fc.integer({ max: 1e6, min: -1e6 }), y: fc.integer({ max: 1e6, min: -1e6 }) })
  .filter((v) => v.x !== 0 || v.y !== 0);

describe("unit (REF-MATHWORLD-UNIT-VECTOR)", () => {
  it("divides by the length", () => {
    // (3, 4) / 5 = (0.6, 0.8).
    expect(unit({ x: 3, y: 4 })).toEqual({ x: 0.6, y: 0.8 });
  });

  it("gives the direction of an axis-aligned edge", () => {
    // (−100, 0) / 100 = (−1, 0): the bottom edge of a clockwise rectangle.
    expect(unit({ x: -100, y: 50 - 50 }).x).toBe(-1);
  });

  it("gives (0, 0) for the zero vector: an edge of length 0 has no direction", () => {
    expect(unit({ x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
  });

  test.prop({ v: NONZERO })("has length 1 for any nonzero vector", ({ v }) => {
    expect(norm(unit(v))).toBeCloseTo(1, 12);
  });
});
