import { describe, expect, it } from "vitest";

import { cmapFormat4, viewOf, viewOfFirst } from "../../../tests";

import { parseSegmentMappingGlyphs } from "./parse-segment-mapping-glyphs";

import type { CmapSegment } from "../../../tests";

/**
 * Segments: A–C by a delta of −64 (glyphs 1 to 3); a–b through the glyph array (glyphs 4 and 0,
 * with a delta of 2); x–y through the glyph array after a–b's two entries (glyphs 7 and 8); é
 * (233) by a delta of −236, which wraps below 0; then 0xFFFF.
 */
const SEGMENTS: readonly CmapSegment[] = [
  { delta: -64, end: 67, glyphs: [], start: 65 },
  { delta: 2, end: 98, glyphs: [4, 0], start: 97 },
  { delta: 0, end: 121, glyphs: [7, 8], start: 120 },
  { delta: -236, end: 233, glyphs: [], start: 233 },
  { delta: 1, end: 0xff_ff, glyphs: [], start: 0xff_ff },
];

/**
 * The glyph of each character in a subtable.
 *
 * @param characters - code points
 * @param segments - the subtable's segments
 * @returns their glyphs; undefined when the subtable could not be read
 */
function glyphsOf(
  characters: readonly number[],
  segments: readonly CmapSegment[],
): readonly number[] | undefined {
  return parseSegmentMappingGlyphs(viewOf(cmapFormat4(segments)), 0, characters)?.map(
    ([, glyph]) => glyph,
  );
}

describe("parseSegmentMappingGlyphs (REF-OT-CMAP)", () => {
  it("maps a character by the code plus the delta", () => {
    // 65 − 64 = 1, 67 − 64 = 3.
    expect(glyphsOf([65, 67], SEGMENTS)).toEqual([1, 3]);
  });

  it("maps the specification's example of three segments", () => {
    // REF-OT-CMAP, format 4: 10 − 9 = 1, 20 − 9 = 11, 30 − 18 = 12, 90 − 18 = 72, 153 − 80 = 73,
    // 480 − 80 = 400; 0xFFFF + 1 = 65 536 ≡ 0.
    const example = [
      { delta: -9, end: 20, glyphs: [], start: 10 },
      { delta: -18, end: 90, glyphs: [], start: 30 },
      { delta: -80, end: 480, glyphs: [], start: 153 },
      { delta: 1, end: 0xff_ff, glyphs: [], start: 0xff_ff },
    ];

    expect(glyphsOf([10, 20, 30, 90, 153, 480, 0xff_ff], example)).toEqual([
      1, 11, 12, 72, 73, 400, 0,
    ]);
  });

  it("maps a character through the glyph array, adding the delta to a glyph other than 0", () => {
    // a: 4 + 2 = 6; b: the array holds 0, the missing glyph, which stays 0.
    expect(glyphsOf([97, 98], SEGMENTS)).toEqual([6, 0]);
  });

  it("reads a second segment's glyphs past the first one's in the glyph array", () => {
    // x and y read entries 2 and 3 of the array.
    expect(glyphsOf([120, 121], SEGMENTS)).toEqual([7, 8]);
  });

  it("reckons the glyph modulo 65 536, by the code and through the glyph array", () => {
    // 233 − 236 = −3 ≡ 65 533; through the array, 4 − 10 = −6 ≡ 65 530.
    expect(glyphsOf([233], SEGMENTS)).toEqual([65_533]);

    expect(
      glyphsOf(
        [97],
        [
          { delta: -10, end: 97, glyphs: [4], start: 97 },
          { delta: 1, end: 0xff_ff, glyphs: [], start: 0xff_ff },
        ],
      ),
    ).toEqual([65_530]);
  });

  it("gives glyph 0 to a character between segments or past the last", () => {
    // 68 lies between C and a; 0x10000 is past every segment of the BMP.
    expect(glyphsOf([68, 0x1_00_00], SEGMENTS)).toEqual([0, 0]);
  });

  it("keeps each character beside its glyph, in the order asked", () => {
    const table = viewOf(cmapFormat4(SEGMENTS));

    expect(parseSegmentMappingGlyphs(table, 0, [66, 65])).toEqual([
      [66, 2],
      [65, 1],
    ]);
  });

  it("gives undefined when a glyph array entry lies outside the table", () => {
    // The table cut by its last 2 bytes: y's entry, the array's last, passes the end; x's does not.
    const cut = cmapFormat4(SEGMENTS).slice(0, -2);

    expect(parseSegmentMappingGlyphs(viewOf(cut), 0, [120])).toEqual([[120, 7]]);
    expect(parseSegmentMappingGlyphs(viewOf(cut), 0, [121])).toBeUndefined();
  });

  it("gives undefined when the header or the segment arrays pass the table's end", () => {
    const table = cmapFormat4(SEGMENTS);

    // 13 bytes: the header is cut; 20 bytes: the five segments' arrays need 14 + 2 + 4 × 10.
    expect(parseSegmentMappingGlyphs(viewOfFirst(table, 13), 0, [65])).toBeUndefined();
    expect(parseSegmentMappingGlyphs(viewOfFirst(table, 20), 0, [65])).toBeUndefined();
  });
});
