import { describe, expect, it } from "vitest";

import { cmapFormat12, cmapFormat4, cmapTable, viewOf } from "../../../tests";

import { parseCharacterMap } from "./parse-character-map";

import type { CmapEncoding } from "../../../tests";

/** A format 4 subtable mapping A (65) to glyph 1. */
const FORMAT_4 = cmapFormat4([
  { delta: -64, end: 65, glyphs: [], start: 65 },
  { delta: 1, end: 0xff_ff, glyphs: [], start: 0xff_ff },
]);

/** A format 12 subtable mapping A (65) to glyph 7. */
const FORMAT_12 = cmapFormat12([{ end: 65, glyph: 7, start: 65 }]);

/**
 * The glyph of A in a `cmap` table.
 *
 * @param encodings - the table's encodings
 * @returns its glyph index; undefined when the table could not be read
 */
function glyphOfA(encodings: readonly CmapEncoding[]): number | undefined {
  return parseCharacterMap(viewOf(cmapTable(encodings)), [65])?.[0]?.[1];
}

describe("parseCharacterMap (REF-OT-CMAP)", () => {
  it("prefers the format 12 subtable to the format 4 one", () => {
    expect(
      glyphOfA([
        { encoding: 1, platform: 3, subtable: FORMAT_4 },
        { encoding: 10, platform: 3, subtable: FORMAT_12 },
      ]),
    ).toBe(7);
  });

  it("reads the format 4 subtable when there is no format 12", () => {
    expect(glyphOfA([{ encoding: 3, platform: 0, subtable: FORMAT_4 }])).toBe(1);
  });

  it("maps nothing from another format, nor from a platform that is not Unicode", () => {
    // Format 6 (trimmed table mapping) is not read; platform 1 (Macintosh) and Windows
    // encoding 0 (Symbol) do not map Unicode code points.
    expect(glyphOfA([{ encoding: 1, platform: 3, subtable: [0, 6, 0, 0] }])).toBe(0);
    expect(glyphOfA([{ encoding: 0, platform: 1, subtable: FORMAT_4 }])).toBe(0);
    expect(glyphOfA([{ encoding: 0, platform: 3, subtable: FORMAT_4 }])).toBe(0);
  });

  it("gives undefined when the subtable read passes the table's end", () => {
    // Format 12 cut after its header: its one group is missing.
    expect(glyphOfA([{ encoding: 10, platform: 3, subtable: FORMAT_12.slice(0, 16) }])).toBe(
      undefined,
    );
  });
});
