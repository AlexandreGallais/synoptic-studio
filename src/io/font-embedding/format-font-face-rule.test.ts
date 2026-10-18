import { describe, expect, it } from "vitest";

import { formatFontFamilyName } from "../text-style";

import { formatFontFaceRule } from "./format-font-face-rule";

import type { EmbeddedFace } from "../../model";

/** The first six bytes of a TrueType file: sfntVersion 0x00010000, then two zeros; in base 64 AAEAAAAA. */
const TRUE_TYPE_BYTES = Uint8Array.from([0, 1, 0, 0, 0, 0]);

/** The first four bytes of a file with CFF outlines, "OTTO"; in base 64 T1RUTw==. */
const CFF_BYTES = Uint8Array.from([0x4f, 0x54, 0x54, 0x4f]);

/**
 * A face of a family, weight and style, from a file of these bytes.
 *
 * @param family - its family's name
 * @param bytes - its file
 * @returns the face, italic and bold
 */
function boldItalic(family: string, bytes: Uint8Array): EmbeddedFace {
  return { family, file: { bytes, isEmbeddable: true, version: "" }, isItalic: true, weight: 700 };
}

describe("formatFontFaceRule (REF-CSS-FONTS-4 §4, §11.2; REF-BATIK-1159)", () => {
  it("[F15.AC2] writes Arial 700 italic from a TrueType file as one rule, without local()", () => {
    // 0 1 0 → 000000 000000 000100 000000: A A E A; 0 0 0 → AAAA.
    expect(formatFontFaceRule(boldItalic("Arial", TRUE_TYPE_BYTES))).toBe(
      '@font-face { font-family: "Arial"; font-style: italic; font-weight: 700; ' +
        'src: url(data:font/ttf;base64,AAEAAAAA) format("truetype"); }',
    );
  });

  it("[F15.AC2] writes a file with CFF outlines as font/otf and opentype", () => {
    // O T T → 19 (T), 3 · 16 + 5 = 53 (1), 4 · 4 + 1 = 17 (R), 20 (U); O → T, 48 (w), "==".
    expect(formatFontFaceRule(boldItalic("Arial", CFF_BYTES))).toBe(
      '@font-face { font-family: "Arial"; font-style: italic; font-weight: 700; ' +
        'src: url(data:font/otf;base64,T1RUTw==) format("opentype"); }',
    );
  });

  it("[F15.AC2] writes an upright face as normal, and a weight as read", () => {
    const upright = { ...boldItalic("Inter", TRUE_TYPE_BYTES), isItalic: false, weight: 450 };

    expect(formatFontFaceRule(upright)).toContain("font-style: normal; font-weight: 450;");
  });

  it('[F15.AC2] escapes the family My "Font" as the font-family of a text is', () => {
    const rule = formatFontFaceRule(boldItalic('My "Font"', TRUE_TYPE_BYTES));

    expect(rule).toContain(String.raw`font-family: "My \"Font\"";`);
    expect(rule).toContain(`font-family: ${formatFontFamilyName('My "Font"')};`);
  });
});
