import { describe, expect, it } from "vitest";

import { fractionToByte } from "./fraction-to-byte";

describe("fractionToByte (REF-CSS-COLOR-4 §16.1.1, DERIV-color-spaces step 8)", () => {
  it.each([
    // 255 · 0.5 = 127.5, a tie rounded up.
    { expected: 128, fraction: 0.5 },
    // 255 · 0.198 = 50.49, rounded down: #32.
    { expected: 50, fraction: 0.198 },
    // 255 · 0.1998 = 50.949, rounded up: #33.
    { expected: 51, fraction: 0.1998 },
    // 255 · 0.501667 = 127.925.
    { expected: 128, fraction: 0.501667 },
    { expected: 0, fraction: 0 },
    { expected: 255, fraction: 1 },
    // Just off the bytes, as binary64 leaves them: still 0 and 255.
    { expected: 0, fraction: -1e-13 },
    { expected: 255, fraction: 1 + 1e-13 },
  ])("[F16.AC3] gives $expected for $fraction", ({ expected, fraction }) => {
    expect(fractionToByte(fraction)).toBe(expected);
  });
});
