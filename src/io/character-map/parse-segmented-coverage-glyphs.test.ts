import { describe, expect, it } from "vitest";

import { cmapFormat12, uint32, viewOf, viewOfFirst } from "../../../tests";

import { parseSegmentedCoverageGlyphs } from "./parse-segmented-coverage-glyphs";

/** Groups: A–C from glyph 10; U+1F600–U+1F601 from glyph 40. */
const SUBTABLE = Uint8Array.from(
  cmapFormat12([
    { end: 67, glyph: 10, start: 65 },
    { end: 0x1_f6_01, glyph: 40, start: 0x1_f6_00 },
  ]),
);

describe("parseSegmentedCoverageGlyphs (REF-OT-CMAP)", () => {
  it("maps a character to the group's start glyph plus its distance from the group's start", () => {
    // C: 10 + (67 − 65) = 12; U+1F601: 40 + 1 = 41, past the Basic Multilingual Plane.
    expect(parseSegmentedCoverageGlyphs(new DataView(SUBTABLE.buffer), 0, [67, 0x1_f6_01])).toEqual(
      [
        [67, 12],
        [0x1_f6_01, 41],
      ],
    );
  });

  it("gives glyph 0 to a character no group holds", () => {
    // 64 is before A, 68 after C.
    expect(parseSegmentedCoverageGlyphs(new DataView(SUBTABLE.buffer), 0, [64, 68])).toEqual([
      [64, 0],
      [68, 0],
    ]);
  });

  it("gives undefined when the header or the groups pass the table's end", () => {
    // 15 bytes cut the 16-byte header; 30 bytes hold the header and one group of the two.
    expect(parseSegmentedCoverageGlyphs(viewOfFirst([...SUBTABLE], 15), 0, [65])).toBe(undefined);

    expect(parseSegmentedCoverageGlyphs(viewOfFirst([...SUBTABLE], 30), 0, [65])).toBe(undefined);
  });

  it("refuses a group count that would pass the table, without reading 2³² groups", () => {
    // numGroups, at 12, set to 0xFFFFFFFF.
    const table = [...SUBTABLE];

    table.splice(12, 4, ...uint32(0xff_ff_ff_ff));

    expect(parseSegmentedCoverageGlyphs(viewOf(table), 0, [65])).toBeUndefined();
  });
});
