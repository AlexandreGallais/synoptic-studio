import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { formatFontFamily } from "./format-font-family";

describe("formatFontFamily (REF-CSS-FONTS-4 §2.1)", () => {
  it("[F05.AC4] names Inter, then sans-serif", () => {
    expect(formatFontFamily("Inter", "sans-serif")).toBe('"Inter", sans-serif');
  });

  it("names the font, its fallback, then sans-serif", () => {
    expect(formatFontFamily("Arial", "Helvetica")).toBe('"Arial", "Helvetica", sans-serif');
  });

  it("names a family once, without regard to case nor to white space at its ends", () => {
    expect(formatFontFamily("Arial", "arial")).toBe('"Arial", sans-serif');
    expect(formatFontFamily("Sans-Serif", "sans-serif")).toBe("sans-serif");
    expect(formatFontFamily(" sans-serif", "sans-serif")).toBe("sans-serif");
  });

  test.prop({ fallback: fc.string(), font: fc.string() })(
    "always ends with sans-serif",
    ({ fallback, font }) => {
      expect(formatFontFamily(font, fallback).endsWith("sans-serif")).toBe(true);
    },
  );
});
