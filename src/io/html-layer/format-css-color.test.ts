import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { alphaToPercent, hexPairToByte } from "../color";

import { formatCssColor } from "./format-css-color";

describe("formatCssColor (REF-CSS-COLOR-4, §5.2)", () => {
  it.each([
    // 255 · 0 / 100 = 0; 255 · 40 / 100 = 102 = 0x66; 255 · 50 / 100 = 127.5, up to 128 = 0x80.
    { opacity: 0, written: "#22c55e00" },
    { opacity: 40, written: "#22c55e66" },
    { opacity: 50, written: "#22c55e80" },
    { opacity: 100, written: "#22c55eff" },
  ])("writes #22c55e at $opacity % as $written", ({ opacity, written }) => {
    expect(formatCssColor("#22c55e", opacity)).toBe(written);
  });

  test.prop({ opacity: fc.integer({ max: 100, min: 0 }) })(
    "keeps the color and gives back each opacity from its alpha pair",
    ({ opacity }) => {
      const written = formatCssColor("#1a2b3c", opacity);

      expect(written.slice(0, 7)).toBe("#1a2b3c");

      const alpha = hexPairToByte(written.slice(7));

      expect(alphaToPercent(alpha)).toBe(opacity);
    },
  );
});
