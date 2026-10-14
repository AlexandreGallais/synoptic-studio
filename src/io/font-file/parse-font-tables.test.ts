import { describe, expect, it } from "vitest";

import { TRUE_TYPE_VERSION, fontFile, testFontTables } from "../../../tests";

import { parseFontTables } from "./parse-font-tables";

/**
 * The tables found in the test font, some left out.
 *
 * @param missing - the tags left out of the file
 * @returns where its tables sit, if it has those measuring needs
 */
function tablesWithout(missing: readonly string[]): ReturnType<typeof parseFontTables> {
  const tables = Object.entries(testFontTables()).filter(([tag]) => !missing.includes(tag));
  const file = fontFile(TRUE_TYPE_VERSION, Object.fromEntries(tables));

  return parseFontTables(new DataView(file.buffer));
}

describe("parseFontTables (REF-OT-OTFF)", () => {
  it("finds the tables measuring needs, the kerning ones absent, ignoring maxp", () => {
    const tables = tablesWithout([]);

    // Seven tables, cmap first: the directory takes 12 + 7 × 16 = 124 bytes, so cmap starts there.
    expect(tables?.cmap.offset).toBe(124);
    expect(tables?.gpos).toBeUndefined();
    expect(tables?.kern).toBeUndefined();

    expect(Object.keys(tables ?? {})).toEqual([
      "cmap",
      "head",
      "hhea",
      "hmtx",
      "gpos",
      "kern",
      "os2",
      "post",
    ]);
  });

  it.each(["cmap", "head", "hhea", "hmtx", "OS/2", "post"])(
    "gives undefined when %s is missing",
    (tag) => {
      expect(tablesWithout([tag])).toBeUndefined();
    },
  );

  it("gives undefined for a file shorter than its directory's header", () => {
    const view = new DataView(new ArrayBuffer(11));

    expect(parseFontTables(view)).toBeUndefined();
  });
});
