import { describe, expect, it } from "vitest";

import { isModelSize } from "./is-model-size";

describe("isModelSize", () => {
  it("accepts 0 and positive integers (Q15)", () => {
    expect(isModelSize(0)).toBe(true);
    expect(isModelSize(100)).toBe(true);
  });

  it("rejects negative, fractional and unsafe values (ADR-0003)", () => {
    expect(isModelSize(-1)).toBe(false);
    expect(isModelSize(0.5)).toBe(false);
    expect(isModelSize(2 ** 53)).toBe(false);
    expect(isModelSize(NaN)).toBe(false);
  });
});
