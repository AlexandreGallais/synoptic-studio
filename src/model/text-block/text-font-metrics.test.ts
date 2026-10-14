import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, HELLO_TEXT } from "../../../tests";
import { DEFAULT_FONT } from "../font";

import { textFontMetrics } from "./text-font-metrics";

import type { FontFamily } from "../font";

/** Arial with one face, regular upright. */
const ARIAL: FontFamily = {
  faces: [{ isItalic: false, metrics: ARIAL_METRICS, weight: 400 }],
  name: "Arial",
  version: "5.01",
};

/** The fonts read: Arial and the default font. */
const FONTS = [ARIAL, DEFAULT_FONT];

describe("textFontMetrics (ADR-0052)", () => {
  it("measures a text with its own font's face", () => {
    expect(textFontMetrics({ ...HELLO_TEXT, font: "Arial" }, FONTS)).toBe(ARIAL_METRICS);

    // Bold italic asked of Arial's only face: still that face.
    expect(
      textFontMetrics({ ...HELLO_TEXT, font: "Arial", isItalic: true, weight: 700 }, FONTS),
    ).toBe(ARIAL_METRICS);
  });

  it("[F05.AC2] measures Inter 450 with Inter's 500 face", () => {
    // Inter Medium's H advances 1525, Regular's 1522.
    expect(textFontMetrics({ ...HELLO_TEXT, weight: 450 }, FONTS)?.advances[72]).toBe(1525);
  });

  it("measures a font without metrics with the default font's face", () => {
    const metrics = textFontMetrics({ ...HELLO_TEXT, font: "Helvetica" }, FONTS);

    // Inter Regular: 2048 units per em, H 1522.
    expect([metrics?.unitsPerEm, metrics?.advances[72]]).toEqual([2048, 1522]);

    // A family read without any face is measured the same way.
    expect(
      textFontMetrics({ ...HELLO_TEXT, font: "Arial" }, [{ ...ARIAL, faces: [] }, DEFAULT_FONT])
        ?.advances[72],
    ).toBe(1522);
  });

  it("gives no metrics when neither the font nor the default font was read", () => {
    expect(textFontMetrics({ ...HELLO_TEXT, font: "Helvetica" }, [ARIAL])).toBeUndefined();
    expect(textFontMetrics(HELLO_TEXT, [])).toBeUndefined();
  });
});
