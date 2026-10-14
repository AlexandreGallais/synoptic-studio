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
  it("finds cmap, head, hhea and hmtx, ignoring the other tables", () => {
    const tables = tablesWithout([]);

    // Five tables in tag order (cmap, head, hhea, hmtx, maxp): the directory takes 12 + 5 × 16 =
    // 92 bytes, so cmap starts at 92.
    expect(tables?.cmap.offset).toBe(92);
    expect(Object.keys(tables ?? {})).toEqual(["cmap", "head", "hhea", "hmtx"]);
  });

  it.each(["cmap", "head", "hhea", "hmtx"])("gives undefined when %s is missing", (tag) => {
    expect(tablesWithout([tag])).toBeUndefined();
  });

  it("gives undefined for a file shorter than its directory's header", () => {
    const view = new DataView(new ArrayBuffer(11));

    expect(parseFontTables(view)).toBeUndefined();
  });
});
