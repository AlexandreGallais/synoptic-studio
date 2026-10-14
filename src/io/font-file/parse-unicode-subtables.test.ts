import { describe, expect, it } from "vitest";

import { cmapTable, uint32 } from "../../../tests";

import { parseUnicodeSubtables } from "./parse-unicode-subtables";

/**
 * The Unicode subtables of a `cmap` table lying at an offset in a file.
 *
 * @param table - the table's bytes
 * @param offset - where it starts in the file
 * @returns the subtables' offsets from the start of the file
 */
function subtablesOf(table: readonly number[], offset: number): readonly number[] {
  const file = Uint8Array.from([...Array.from({ length: offset }, () => 0), ...table]);

  return parseUnicodeSubtables(new DataView(file.buffer), { length: table.length, offset });
}

describe("parseUnicodeSubtables (REF-OT-CMAP)", () => {
  it("gives the subtables of the Unicode platform and of Windows encodings 1 and 10", () => {
    const table = cmapTable([
      { encoding: 3, platform: 0, subtable: [0, 4] },
      { encoding: 0, platform: 1, subtable: [0, 0] },
      { encoding: 1, platform: 3, subtable: [0, 4] },
      { encoding: 10, platform: 3, subtable: [0, 12] },
    ]);

    // Four records: 4 + 4 × 8 = 36 bytes, then one 2-byte subtable each at 36, 38, 40, 42; the
    // table sits at 100 in the file, and the Macintosh one (38) is left out.
    expect(subtablesOf(table, 100)).toEqual([136, 140, 142]);
  });

  it("leaves out a subtable whose offset is past the table", () => {
    const table = cmapTable([{ encoding: 1, platform: 3, subtable: [0, 4] }]);

    // The record's subtable offset, at 4 + 4 = 8, is set to 14, the table's length (4 + 8 + 2).
    table.splice(8, 4, ...uint32(14));

    expect(subtablesOf(table, 0)).toEqual([]);
  });
});
