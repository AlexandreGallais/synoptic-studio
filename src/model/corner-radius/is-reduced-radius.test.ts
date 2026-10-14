import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isReducedRadius } from "./is-reduced-radius";

describe("isReducedRadius (ADR-0007)", () => {
  it("is true for 60 drawn 47.7, false for 20 drawn 20", () => {
    expect(isReducedRadius(60, 47.7)).toBe(true);
    expect(isReducedRadius(20, 20)).toBe(false);
  });

  test.prop({ radius: fc.nat(1e5) })("forgives a rounding error", ({ radius }) => {
    expect(isReducedRadius(radius, radius - 1e-12)).toBe(false);
  });
});
