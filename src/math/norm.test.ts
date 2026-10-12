import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { EPSILON } from "./epsilon";
import { norm } from "./norm";
import { subtract } from "./subtract";

/** Vector with integer components, as produced by integer vertices. */
const VECTOR = fc.record({
  x: fc.integer({ max: 1e6, min: -1e6 }),
  y: fc.integer({ max: 1e6, min: -1e6 }),
});

describe("norm (REF-MATHWORLD-VECTOR-NORM)", () => {
  it("is 5 for (3, 4)", () => {
    // √(3² + 4²) = √25 = 5.
    expect(norm({ x: 3, y: 4 })).toBe(5);
  });

  it("is the absolute value along an axis", () => {
    // √(0² + (−50)²) = 50: a vertical edge of length 50.
    expect(norm({ x: 0, y: -50 })).toBe(50);
  });

  it("is 0 for the zero vector: an edge of length 0", () => {
    expect(norm({ x: 0, y: 0 })).toBe(0);
  });

  test.prop({ v: VECTOR })("does not depend on the direction", ({ v }) => {
    expect(norm({ x: -v.x, y: -v.y })).toBe(norm(v));
  });

  test.prop({ a: VECTOR, b: VECTOR })("satisfies the triangle inequality", ({ a, b }) => {
    // ‖a − b‖ ≤ ‖a‖ + ‖b‖, within the float tolerance of Q10 relative to the sizes.
    expect(norm(subtract(a, b))).toBeLessThanOrEqual((norm(a) + norm(b)) * (1 + EPSILON));
  });
});
