import { describe, expect, it } from "vitest";

import { kernTable, uint16, viewOf } from "../../../tests";

import { parseKernTable } from "./parse-kern-table";

describe("parseKernTable (REF-OT-KERN)", () => {
  it("reads each subtable of horizontal kerning values as a lookup of its own", () => {
    const table = kernTable([
      {
        coverage: 1,
        pairs: [
          { left: 1, right: 2, value: -50 },
          { left: 1, right: 3, value: -20 },
          { left: 4, right: 1, value: 15 },
        ],
      },
      // Override (bit 3) set: still read, its values added.
      { coverage: 0b1001, pairs: [{ left: 1, right: 2, value: -5 }] },
    ]);

    expect(parseKernTable(viewOf(table), 0)).toEqual([
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

    expect(parseKernTable(viewOf(table), 0)).toEqual([
      [{ kind: "pairs", pairs: { 1: { 3: -20 } } }],
    ]);
  });

  it("reads nothing from a table of another version", () => {
    // Apple's version 1.0 starts with the 32-bit 0x00010000: its first 16 bits are 1.
    const table = [...uint16(1), ...uint16(0)];

    expect(parseKernTable(viewOf(table), 0)).toEqual([]);
  });
});
