import { describe, expect, it } from "vitest";

import { kernTable, uint16, viewOf, viewOfFirst } from "../../../tests";
import { glyphPairKey } from "../../model";

import { parseKernTable } from "./parse-kern-table";

/** The glyphs asked: 1 to 4. */
const GLYPHS = new Set([1, 2, 3, 4]);

describe("parseKernTable (REF-OT-KERN)", () => {
  it("adds up the subtables of horizontal kerning values into one lookup", () => {
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

    // 1 2: −50 + −5 = −55; 4 9 is left out: glyph 9 was not asked.
    expect(parseKernTable(viewOf(table), GLYPHS)).toEqual([
      [
        {
          kind: "pairs",
          pairs: { [glyphPairKey(1, 2)]: -55, [glyphPairKey(1, 3)]: -20, [glyphPairKey(4, 1)]: 15 },
        },
      ],
    ]);
  });

  it("replaces the value accumulated with that of a subtable whose override bit is set", () => {
    // 1 2: −50, then an override subtable (coverage bit 3) gives −20: −20, not −70; 1 3 keeps −20.
    // A plain subtable after it adds again: −20 + −5 = −25.
    const table = kernTable([
      {
        coverage: 1,
        pairs: [
          { left: 1, right: 2, value: -50 },
          { left: 1, right: 3, value: -20 },
        ],
      },
      { coverage: 0b1001, pairs: [{ left: 1, right: 2, value: -20 }] },
      { coverage: 1, pairs: [{ left: 1, right: 2, value: -5 }] },
    ]);

    expect(parseKernTable(viewOf(table), GLYPHS)).toEqual([
      [{ kind: "pairs", pairs: { [glyphPairKey(1, 2)]: -25, [glyphPairKey(1, 3)]: -20 } }],
    ]);
  });

  it.each([
    // A B −50, then an override A B −20: −20; then an override −5: −5.
    { expected: -20, override: -20 },
    { expected: -5, override: -5 },
  ])("gives $expected to a pair of −50 overridden by $override", ({ expected, override }) => {
    const table = kernTable([
      { coverage: 1, pairs: [{ left: 1, right: 2, value: -50 }] },
      { coverage: 0b1001, pairs: [{ left: 1, right: 2, value: override }] },
    ]);

    expect(parseKernTable(viewOf(table), GLYPHS)).toEqual([
      [{ kind: "pairs", pairs: { [glyphPairKey(1, 2)]: expected } }],
    ]);
  });

  it("gives no lookup when no subtable holds horizontal kerning values", () => {
    const table = kernTable([{ coverage: 0, pairs: [{ left: 1, right: 2, value: -50 }] }]);

    expect(parseKernTable(viewOf(table), GLYPHS)).toEqual([]);
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
      [{ kind: "pairs", pairs: { [glyphPairKey(1, 3)]: -20 } }],
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

  it("gives undefined for a subtable shorter than its own header, never reading it twice", () => {
    // Two subtables of A B −50; the first's length (at 4 + 2) set to 0, below its 6-byte header
    // (the length includes the header, REF-OT-KERN): the walk would start the second subtable at
    // the first's place again and count −50 twice, −100.
    const pair = { coverage: 1, pairs: [{ left: 1, right: 2, value: -50 }] };
    const table = kernTable([pair, pair]);
    const shortened = [...table.slice(0, 6), ...uint16(5), ...table.slice(8)];
    const empty = [...table.slice(0, 6), ...uint16(0), ...table.slice(8)];

    expect(parseKernTable(viewOf(shortened), GLYPHS)).toBeUndefined();
    expect(parseKernTable(viewOf(empty), GLYPHS)).toBeUndefined();
  });

  it("gives undefined at once when the table cannot hold the subtables announced", () => {
    // 0xFFFF subtables announced in a table of 4 + 20 bytes: their 6-byte headers alone need
    // 393 210 bytes; walking them one by one took seconds.
    const table = kernTable([{ coverage: 1, pairs: [{ left: 1, right: 2, value: -50 }] }]);
    const many = [...uint16(0), ...uint16(0xff_ff), ...table.slice(4)];

    expect(parseKernTable(viewOf(many), GLYPHS)).toBeUndefined();
  });
});
