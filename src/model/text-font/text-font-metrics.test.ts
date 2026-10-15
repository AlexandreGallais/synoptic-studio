import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { DEFAULT_FONT } from "../font";

import { textFontMetrics } from "./text-font-metrics";

import type { FontFamily } from "../font";

/** Arial with one face, regular upright. */
const ARIAL: FontFamily = {
  faces: [{ file: undefined, isItalic: false, metrics: ARIAL_METRICS, weight: 400 }],
  name: "Arial",
  version: "5.01",
};

/** The fonts read: Arial and the default font. */
const FONTS = [ARIAL, DEFAULT_FONT];

/** Inter's italic 400 face. */
const INTER_ITALIC = DEFAULT_FONT.faces.find((face) => face.isItalic && face.weight === 400);

describe("textFontMetrics (ADR-0052)", () => {
  it("measures a text with its own font's face", () => {
    expect(textFontMetrics({ ...HELLO_TEXT, font: "Arial" }, FONTS)).toBe(ARIAL_METRICS);

    // Bold italic asked of Arial's only face: still that face.
    expect(
      textFontMetrics({ ...HELLO_TEXT, font: "Arial", isItalic: true, weight: 700 }, FONTS),
    ).toBe(ARIAL_METRICS);
  });

  it("matches the font's name without regard to case (REF-CSS-FONTS-4, §5.1)", () => {
    expect(textFontMetrics({ ...HELLO_TEXT, font: "arial" }, FONTS)).toBe(ARIAL_METRICS);
    expect(textFontMetrics({ ...HELLO_TEXT, font: "ARIAL" }, FONTS)).toBe(ARIAL_METRICS);
  });

  it("chooses the italic face for an italic text", () => {
    const family: FontFamily = {
      ...ARIAL,
      faces: [
        ...ARIAL.faces,
        { file: undefined, isItalic: true, metrics: ARIAL_UNKERNED, weight: 400 },
      ],
    };
    const italic = { ...HELLO_TEXT, isItalic: true };

    expect(textFontMetrics({ ...italic, font: "Arial" }, [family])).toBe(ARIAL_UNKERNED);
    // The same through the default font, for a font without metrics.
    expect(textFontMetrics({ ...italic, font: "Helvetica" }, FONTS)).toBe(INTER_ITALIC?.metrics);
  });

  it("measures Inter 450 with Inter's 500 face", () => {
    // Inter Medium's H advances 1525, Regular's 1522.
    expect(textFontMetrics({ ...HELLO_TEXT, weight: 450 }, FONTS)?.advances[72]).toBe(1525);
  });

  it("[F17.AC4] measures 450 of a family of 400 and 700 with its 400 face, as CSS draws it", () => {
    // REF-CSS-FONTS-4, §5.2: for 450, the weights up to 500 ascending (none), then the lighter
    // ones descending: 400, the face the browser draws too once the family's faces are loaded.
    const family: FontFamily = {
      ...ARIAL,
      faces: [
        ...ARIAL.faces,
        { file: undefined, isItalic: false, metrics: ARIAL_UNKERNED, weight: 700 },
      ],
    };

    expect(textFontMetrics({ ...HELLO_TEXT, font: "Arial", weight: 450 }, [family])).toBe(
      ARIAL_METRICS,
    );
  });

  it("measures a font without metrics with its fallback family's face first", () => {
    expect(textFontMetrics({ ...HELLO_TEXT, fallback: "arial", font: "Helvetica" }, FONTS)).toBe(
      ARIAL_METRICS,
    );
  });

  it("measures a font and a fallback without metrics with the default font's face", () => {
    const metrics = textFontMetrics({ ...HELLO_TEXT, font: "Helvetica" }, FONTS);

    // Inter Regular: 2048 units per em, H 1522.
    expect([metrics?.unitsPerEm, metrics?.advances[72]]).toEqual([2048, 1522]);

    // A family read without any face is measured the same way.
    expect(
      textFontMetrics({ ...HELLO_TEXT, font: "Arial" }, [{ ...ARIAL, faces: [] }, DEFAULT_FONT])
        ?.advances[72],
    ).toBe(1522);
  });

  it("gives no metrics when neither the font, its fallback nor the default font was read", () => {
    expect(textFontMetrics({ ...HELLO_TEXT, font: "Helvetica" }, [ARIAL])).toBeUndefined();
    expect(textFontMetrics(HELLO_TEXT, [])).toBeUndefined();
  });
});
