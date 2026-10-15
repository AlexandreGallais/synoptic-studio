import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, HELLO_TEXT } from "../../../tests";
import { DEFAULT_FONT } from "../font";

import { textFontFace } from "./text-font-face";

import type { FontFamily } from "../font";

/** Arial with one face, regular upright. */
const ARIAL: FontFamily = {
  faces: [{ file: undefined, isItalic: false, metrics: ARIAL_METRICS, weight: 400 }],
  name: "Arial",
  version: "5.01",
};

/** The fonts read: Arial and the default font. */
const FONTS = [ARIAL, DEFAULT_FONT];

describe("textFontFace (ADR-0052, ADR-0053)", () => {
  it("gives the face a text is measured with and its family's name as read", () => {
    // The text names "arial": the family read is named "Arial".
    expect(textFontFace({ ...HELLO_TEXT, font: "arial" }, FONTS)).toEqual({
      face: ARIAL.faces[0],
      family: "Arial",
    });
  });

  it("gives the fallback's face, then the default font's", () => {
    expect(
      textFontFace({ ...HELLO_TEXT, fallback: "Arial", font: "Helvetica" }, FONTS)?.family,
    ).toBe("Arial");

    // Inter 700 italic, the default font's face for a text in an unknown font.
    const face = textFontFace(
      { ...HELLO_TEXT, font: "Helvetica", isItalic: true, weight: 700 },
      FONTS,
    );

    expect([face?.family, face?.face.weight, face?.face.isItalic]).toEqual(["Inter", 700, true]);
  });

  it("gives no face when no family read has one", () => {
    expect(textFontFace({ ...HELLO_TEXT, font: "Arial" }, [{ ...ARIAL, faces: [] }])).toBe(
      undefined,
    );

    expect(textFontFace(HELLO_TEXT, [])).toBeUndefined();
  });
});
