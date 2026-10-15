import { describe, expect, it } from "vitest";

import { integerRange } from "../../math";

import { hexPairToByte } from "./hex-pair-to-byte";

describe("hexPairToByte (REF-CSS-COLOR-4 §5.2)", () => {
  it.each([
    // 0 · 16 + 0
    { expected: 0, pair: "00" },
    // 15 · 16 + 15
    { expected: 255, pair: "ff" },
    // 8 · 16 + 0
    { expected: 128, pair: "80" },
    // 12 · 16 + 12: CSS's #0000ffcc
    { expected: 204, pair: "cc" },
    // 14 · 16 + 12
    { expected: 236, pair: "ec" },
    // 1 · 16 + 10
    { expected: 26, pair: "1a" },
  ])("gives $expected for '$pair'", ({ expected, pair }) => {
    expect(hexPairToByte(pair)).toBe(expected);
  });

  it("gives every byte back from its two lower-case digits", () => {
    const bytes = integerRange(0, 255);

    expect(bytes.map((byte) => hexPairToByte(byte.toString(16).padStart(2, "0")))).toEqual(bytes);
  });
});
