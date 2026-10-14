import { describe, expect, it } from "vitest";

import { kernTable, uint16, viewOf, viewOfFirst } from "../../../tests";

import { parseKernTable } from "./parse-kern-table";

/** The glyphs asked: 1 to 4. */
const GLYPHS = new Set([1, 2, 3, 4]);

describe("parseKernTable (REF-OT-KERN)", () => {
  it("reads each subtable of horizontal kerning values as a lookup of its own", () => {
    const table = kernTable([
      {
        coverage: 1,
        pairs: [
          { left: 1, right: 2, value: -50 },
          { left: 1, right: 3, value: -20 },
          { left: 4, right: 1, value: 15 },
          { left: 4, right: 9, value: 15 },
        ],
      },
      { coverage: 1, pairs: [{ left: 1, right: 2, value: -5 }] },
    ]);

    // 4 9 is left out: glyph 9 was not asked.
    expect(parseKernTable(viewOf(table), GLYPHS)).toEqual([
      [{ kind: "pairs", pairs: { 1: { 2: -50, 3: -20 }, 4: { 1: 15 } } }],
      [{ kind: "pairs", pairs: { 1: { 2: -5 } } }],
    ]);
  });

  it.each([
    { coverage: 0b0000, what: "vertical" },
    { coverage: 0b0011, what: "minimum values" },
    { coverage: 0b0101, what: "cross-stream" },
    { coverage: 0x02_01, what: "format 2" },
  ])("skips a subtable of $what", ({ coverage }) => {
    const table = kernTable([
      { coverage, pairs: [{ left: 1, right: 2, value: -50 }] },
      { coverage: 1, pairs: [{ left: 1, right: 3, value: -20 }] },
    ]);

    expect(parseKernTable(viewOf(table), GLYPHS)).toEqual([
      [{ kind: "pairs", pairs: { 1: { 3: -20 } } }],
    ]);
  });

  it("reads nothing from a table of another version", () => {
    // Apple's version 1.0 starts with the 32-bit 0x00010000: its first 16 bits are 1, then
    // nTables as 32 bits.
    const table = [...uint16(1), ...uint16(0), 0, 0, 0, 1];

    expect(parseKernTable(viewOf(table), GLYPHS)).toEqual([]);
  });

  it("gives undefined when a subtable passes the table's end", () => {
    const table = kernTable([{ coverage: 1, pairs: [{ left: 1, right: 2, value: -50 }] }]);

    // Cut in the pair; cut in the subtable's header; a second subtable announced but absent.
    expect(parseKernTable(viewOfFirst(table, -1), GLYPHS)).toBeUndefined();
    expect(parseKernTable(viewOfFirst(table, 8), GLYPHS)).toBeUndefined();
    // Cut in the format 0 header, after the subtable's own 6-byte header: 4 + 8 = 12 bytes.
    expect(parseKernTable(viewOfFirst(table, 12), GLYPHS)).toBeUndefined();

    const twoAnnounced = [...uint16(0), ...uint16(2), ...table.slice(4)];

    expect(parseKernTable(viewOf(twoAnnounced), GLYPHS)).toBeUndefined();
  });
});
