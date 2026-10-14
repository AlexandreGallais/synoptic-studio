import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { hasArea } from "./has-area";

describe("hasArea (REF-WIKI-RECTANGLE)", () => {
  it("is true for 120 × 80, false for 0 × 80", () => {
    expect(hasArea({ height: 80, width: 120 })).toBe(true);
    expect(hasArea({ height: 80, width: 0 })).toBe(false);
  });

  test.prop({ height: fc.nat(1e5), width: fc.nat(1e5) })(
    "is true exactly when both sides are > 0",
    ({ height, width }) => {
      expect(hasArea({ height, width })).toBe(width > 0 && height > 0);
    },
  );
});
