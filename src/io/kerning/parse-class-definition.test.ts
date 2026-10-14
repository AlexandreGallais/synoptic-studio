import { describe, expect, it } from "vitest";

import { rangeTable, uint16, viewOf } from "../../../tests";

import { parseClassDefinition } from "./parse-class-definition";

describe("parseClassDefinition (REF-OT-LAYOUT-COMMON)", () => {
  it("reads a class array (format 1): one class per glyph from the start glyph", () => {
    // Start glyph 10, three classes: 10 → 2, 11 → 0, 12 → 1.
    const table = [
      ...uint16(1),
      ...uint16(10),
      ...uint16(3),
      ...uint16(2),
      ...uint16(0),
      ...uint16(1),
    ];

    expect(parseClassDefinition(viewOf(table), 0)).toEqual({ 10: 2, 11: 0, 12: 1 });
  });

  it("reads class ranges (format 2): each glyph of a range in its class", () => {
    const table = rangeTable([
      { end: 5, start: 4, value: 1 },
      { end: 8, start: 8, value: 3 },
    ]);

    expect(parseClassDefinition(viewOf(table), 0)).toEqual({ 4: 1, 5: 1, 8: 3 });
  });
});
