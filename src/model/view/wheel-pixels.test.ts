import { describe, expect, it } from "vitest";

import { wheelPixels } from "./wheel-pixels";

describe("wheelPixels (REF-PENPOT-NORMALIZE-WHEEL)", () => {
  it.each([
    // Pixels kept: −100.
    { delta: -100, expected: -100, mode: 0 },
    // Lines of 40 pixels: −2.5 · 40 = −100; 3 · 40 = 120.
    { delta: -2.5, expected: -100, mode: 1 },
    { delta: 3, expected: 120, mode: 1 },
    // Pages of 800 pixels: −0.125 · 800 = −100; 1 · 800 = 800.
    { delta: -0.125, expected: -100, mode: 2 },
    { delta: 1, expected: 800, mode: 2 },
    // No delta, whatever its unit.
    { delta: 0, expected: 0, mode: 2 },
  ])("[F42.AC1] turns $delta in mode $mode into $expected pixels", ({ delta, expected, mode }) => {
    expect(wheelPixels(delta, mode)).toBe(expected);
  });
});
