import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { discriminant } from "./discriminant";

describe("discriminant (REF-WIKI-QUADRATIC)", () => {
  it("is 1 for t² − 3t + 2 and 0 for t² + 2t + 1", () => {
    // (−3)² − 4 · 1 · 2 = 1; 2² − 4 · 1 · 1 = 0.
    expect(discriminant({ a: 1, b: -3, c: 2 })).toBe(1);
    expect(discriminant({ a: 1, b: 2, c: 1 })).toBe(0);
  });

  test.prop({ r: fc.integer({ max: 1000, min: -1000 }), s: fc.integer({ max: 1000, min: -1000 }) })(
    "is (r − s)² for (t − r)(t − s)",
    ({ r, s }) => {
      // (t − r)(t − s) = t² − (r + s) t + rs.
      expect(discriminant({ a: 1, b: -(r + s), c: r * s })).toBe((r - s) * (r - s));
    },
  );
});
