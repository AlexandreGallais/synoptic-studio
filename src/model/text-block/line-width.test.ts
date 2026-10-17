import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, ARIAL_UNKERNED } from "../../../tests";
import { DEFAULT_FONT } from "../font";
import { lineCodePoints } from "../line-characters";

import { lineWidth } from "./line-width";

/** Size 16, no spacing. */
const PLAIN = { letterSpacing: 0, size: 16, wordSpacing: 0 };

/** Inter Regular, the face of weight 400 upright. */
const INTER_REGULAR = DEFAULT_FONT.faces.find((face) => face.weight === 400 && !face.isItalic);

describe("lineWidth (DERIV-text-width, steps 1 to 3)", () => {
  it("[F05.AC3] measures 'Hello World' in Arial 16: 82.6875 without kerning, 82.39844 with it", () => {
    // 10 584 × 16 / 2048 = 82.6875; (10 584 − 37) × 16 / 2048 = 82.3984375.
    expect(lineWidth(ARIAL_UNKERNED, "Hello World", PLAIN)).toBe(82.6875);
    expect(lineWidth(ARIAL_METRICS, "Hello World", PLAIN)).toBe(82.3984375);
  });

  it("[F05.AC3] adds letter spacing 2 between adjacent characters and word spacing 4 at the space", () => {
    const spaced = { letterSpacing: 2, size: 16, wordSpacing: 4 };

    // 82.6875 + 10 × 2 + 1 × 4 = 106.6875; kerned: 82.3984375 + 24 = 106.3984375.
    expect(lineWidth(ARIAL_UNKERNED, "Hello World", spaced)).toBe(106.6875);
    expect(lineWidth(ARIAL_METRICS, "Hello World", spaced)).toBe(106.3984375);
  });

  it("[F05.AC3] gives a single character no letter spacing", () => {
    // 1479 × 16 / 2048 = 11.5546875.
    expect(lineWidth(ARIAL_METRICS, "H", { ...PLAIN, letterSpacing: 2 })).toBe(11.5546875);
  });

  it("measures 'AVATAR' in Arial 16: 64.015625 without kerning, 59.265625 with it", () => {
    // 8194 × 16 / 2048 = 64.015625; (8194 − 4 × 152) × 16 / 2048 = 59.265625.
    expect(lineWidth(ARIAL_UNKERNED, "AVATAR", PLAIN)).toBe(64.015625);
    expect(lineWidth(ARIAL_METRICS, "AVATAR", PLAIN)).toBe(59.265625);
  });

  it("adds word spacing at a no-break space too, and letter spacing may tighten", () => {
    // H, no-break space (glyph 0: 1536), H: (1479 + 1536 + 1479) × 16 / 2048 = 35.109375,
    // then 2 × (−1) + 1 × 3 = +1.
    expect(
      lineWidth(ARIAL_METRICS, "H\u{A0}H", { letterSpacing: -1, size: 16, wordSpacing: 3 }),
    ).toBe(36.109375);
  });

  it("[F05.AC3] measures Inter Regular 16 from its shipped metrics", () => {
    const metrics = INTER_REGULAR?.metrics ?? ARIAL_METRICS;

    // T 1322 + o 1228 − 160 = 2390 units: 2390 × 16 / 2048 = 18.671875.
    expect(lineWidth(metrics, "To", PLAIN)).toBe(18.671875);
    // 11 279 units of advances, W o −104 and r l +32: 11 207 × 16 / 2048 = 87.5546875.
    expect(lineWidth(metrics, "Hello World", PLAIN)).toBe(87.5546875);
  });

  it("[F05.AC3] adds letter spacing between typographic character units only", () => {
    const metrics = INTER_REGULAR?.metrics ?? ARIAL_METRICS;
    const spaced = { ...PLAIN, letterSpacing: 2 };

    // A zero-width space (Cf) is measured and spaced as if absent: "AB", A 1413 + B 1340 = 2753
    // units, 2753 × 16 / 2048 = 21.5078125, plus one letter spacing: 23.5078125.
    expect(lineWidth(metrics, "A\u{200B}B", spaced)).toBe(23.5078125);
    expect(lineWidth(metrics, "AB", spaced)).toBe(23.5078125);
    // e and a combining acute compose into é, 1194 units: 9.328125, one unit, no spacing.
    expect(lineWidth(metrics, "e\u{301}", spaced)).toBe(9.328125);
    // q and a combining acute (0 units) stay two characters but one unit: q q 2508 units,
    // 19.59375, plus one letter spacing between the two units: 21.59375.
    expect(lineWidth(metrics, "q\u{301}q", spaced)).toBe(21.59375);
  });

  it("counts a character beyond the basic plane as one character", () => {
    // a and 😀 take glyph 0, 1536 each: 3072 × 16 / 2048 = 24, plus one letter spacing.
    expect(lineWidth(ARIAL_METRICS, "a\u{1F600}", { ...PLAIN, letterSpacing: 2 })).toBe(26);
  });

  it("adds a negative word spacing at the space", () => {
    // 82.6875 − 4 = 78.6875.
    expect(lineWidth(ARIAL_UNKERNED, "Hello World", { ...PLAIN, wordSpacing: -4 })).toBe(78.6875);
  });

  it("measures a single space as its advance and one word spacing", () => {
    const metrics = INTER_REGULAR?.metrics ?? ARIAL_METRICS;

    // Inter's space, 576 units: 4.5, plus 3.
    expect(lineWidth(metrics, " ", { ...PLAIN, wordSpacing: 3 })).toBe(7.5);
  });

  it("measures an empty line as 0, whatever the spacing", () => {
    expect(lineWidth(ARIAL_METRICS, "", { letterSpacing: 5, size: 16, wordSpacing: 5 })).toBe(0);
  });

  test.prop({
    letterSpacing: fc.integer({ max: 20, min: -20 }),
    line: fc.stringMatching(/^[HeloWrdAVTR ]{1,20}$/u),
    size: fc.integer({ max: 1000, min: 1 }),
  })("grows by the letter spacing once per added character", ({ letterSpacing, line, size }) => {
    const spacing = { letterSpacing, size, wordSpacing: 0 };
    const unspaced = lineWidth(ARIAL_UNKERNED, line, { ...spacing, letterSpacing: 0 });

    expect(lineWidth(ARIAL_UNKERNED, line, spacing)).toBeCloseTo(
      unspaced + (lineCodePoints(line).length - 1) * letterSpacing,
      6,
    );
  });
});
