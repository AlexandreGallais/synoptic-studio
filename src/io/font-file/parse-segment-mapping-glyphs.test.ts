import { describe, expect, it } from "vitest";

import { cmapFormat4 } from "../../../tests";

import { parseSegmentMappingGlyphs } from "./parse-segment-mapping-glyphs";

/**
 * Segments: A–C by a delta of −64 (glyphs 1 to 3); a–b through the glyph array (glyphs 4 and 0,
 * with a delta of 2); é (233) by a delta of −236, which wraps below 0; then 0xFFFF.
 */
const SUBTABLE = Uint8Array.from(
  cmapFormat4([
    { delta: -64, end: 67, glyphs: [], start: 65 },
    { delta: 2, end: 98, glyphs: [4, 0], start: 97 },
    { delta: -236, end: 233, glyphs: [], start: 233 },
    { delta: 1, end: 0xff_ff, glyphs: [], start: 0xff_ff },
  ]),
);

/**
 * The glyph of each character in the subtable.
 *
 * @param characters - code points
 * @returns their glyphs
 */
function glyphsOf(characters: readonly number[]): readonly number[] {
  return parseSegmentMappingGlyphs(new DataView(SUBTABLE.buffer), 0, characters).map(
    ([, glyph]) => glyph,
  );
}

describe("parseSegmentMappingGlyphs (REF-OT-CMAP)", () => {
  it("maps a character by the code plus the delta", () => {
    // 65 − 64 = 1, 67 − 64 = 3.
    expect(glyphsOf([65, 67])).toEqual([1, 3]);
  });

  it("maps a character through the glyph array, adding the delta to a glyph other than 0", () => {
    // a: 4 + 2 = 6; b: the array holds 0, the missing glyph, which stays 0.
    expect(glyphsOf([97, 98])).toEqual([6, 0]);
  });

  it("reckons the glyph modulo 65 536", () => {
    // 233 − 236 = −3 ≡ 65 533 (mod 65 536).
    expect(glyphsOf([233])).toEqual([65_533]);
  });

  it("gives glyph 0 to a character between segments or past the last", () => {
    // 68 lies between C and a; 0x10000 is past every segment of the BMP.
    expect(glyphsOf([68, 0x1_00_00])).toEqual([0, 0]);
  });

  it("keeps each character beside its glyph, in the order asked", () => {
    expect(parseSegmentMappingGlyphs(new DataView(SUBTABLE.buffer), 0, [66, 65])).toEqual([
      [66, 2],
      [65, 1],
    ]);
  });
});
