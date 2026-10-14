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

  it.each([
    { size: 19, tag: "head" },
    { size: 4, tag: "hhea" },
    { size: 35, tag: "hhea" },
    { size: 11, tag: "post" },
    { size: 29, tag: "OS/2" },
    { size: 3, tag: "cmap" },
  ])(
    "gives undefined when $tag is $size bytes long, too short for what is read",
    ({ size, tag }) => {
      const tables = { ...testFontTables() };
      const cut = { ...tables, [tag]: (tables[tag] ?? []).slice(0, size) };

      const file = fontFile(TRUE_TYPE_VERSION, cut);

      expect(parseFontTables(new DataView(file.buffer))).toBeUndefined();
    },
  );

  it("leaves out a kerning table too short for its header", () => {
    // GPOS needs its 10-byte header, kern its 4-byte one.
    const file = fontFile(TRUE_TYPE_VERSION, {
      ...testFontTables(),
      GPOS: [0, 1, 0, 0, 0, 0, 0, 0, 0],
      kern: [0, 0, 0],
    });
    const tables = parseFontTables(new DataView(file.buffer));

    expect(tables?.gpos).toBeUndefined();
    expect(tables?.kern).toBeUndefined();
  });

  it("gives undefined for a file shorter than its directory's header", () => {
    const view = new DataView(new ArrayBuffer(11));

    expect(parseFontTables(view)).toBeUndefined();
  });
});
