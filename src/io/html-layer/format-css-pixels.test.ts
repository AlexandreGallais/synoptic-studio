import { describe, expect, it } from "vitest";

import { formatCssPixels } from "./format-css-pixels";

describe("formatCssPixels (REF-SVG2-EMBEDDED)", () => {
  it.each([
    // 18.3984375 at 5 decimals, as SVG numbers are written (Q10).
    { length: 18.3984375, written: "18.39844px" },
    { length: 16, written: "16px" },
    { length: -4, written: "-4px" },
    { length: 0, written: "0px" },
  ])("writes $length as $written", ({ length, written }) => {
    expect(formatCssPixels(length)).toBe(written);
  });
});
