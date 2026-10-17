import { describe, expect, it } from "vitest";

import { cmapTable, uint16, uint32, viewOf } from "../../../tests";

import { parseUnicodeSubtables } from "./parse-unicode-subtables";

describe("parseUnicodeSubtables (REF-OT-CMAP)", () => {
  it("gives the subtables of the Unicode platform and of Windows encodings 1 and 10", () => {
    const table = cmapTable([
      { encoding: 3, platform: 0, subtable: [0, 4] },
      { encoding: 0, platform: 1, subtable: [0, 0] },
      { encoding: 1, platform: 3, subtable: [0, 4] },
      { encoding: 10, platform: 3, subtable: [0, 12] },
    ]);

    // Four records: 4 + 4 × 8 = 36 bytes, then one 2-byte subtable each at 36, 38, 40, 42; the
    // Macintosh one (38) is left out.
    expect(parseUnicodeSubtables(viewOf(table))).toEqual([36, 40, 42]);
  });

  it("leaves out a subtable whose format does not fit in the table", () => {
    const table = cmapTable([{ encoding: 1, platform: 3, subtable: [0, 4] }]);

    // The record's subtable offset, at 4 + 4 = 8, is set to 13: its 2-byte format would end at
    // 15, past the table's 4 + 8 + 2 = 14 bytes.
    table.splice(8, 4, ...uint32(13));

    expect(parseUnicodeSubtables(viewOf(table))).toEqual([]);
  });

  it("reads only the records that fit in the table", () => {
    // numTables says 3, but the table holds the header alone.
    const header = [...uint16(0), ...uint16(3)];

    expect(parseUnicodeSubtables(viewOf(header))).toEqual([]);
  });
});
