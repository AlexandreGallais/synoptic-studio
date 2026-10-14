import { describe, expect, it } from "vitest";

import {
  X_ADVANCE_ONLY,
  gposTable,
  pairPosFormat1,
  uint16,
  viewOf,
  viewOfFirst,
} from "../../../tests";

import { parseGposKerning } from "./parse-gpos-kerning";

/** The glyphs asked: 1 to 3. */
const GLYPHS = new Set([1, 2, 3]);

/** A pair adjustment subtable: glyphs 1 2, −50. */
const PAIR = pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[2, -50]] }]);

/** A pair adjustment subtable: glyphs 1 3, −20. */
const OTHER_PAIR = pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[3, -20]] }]);

/** The rule of `PAIR`. */
const PAIR_RULE = { kind: "pairs", pairs: { 1: { 2: -50 } } };

/** The rule of `OTHER_PAIR`. */
const OTHER_RULE = { kind: "pairs", pairs: { 1: { 3: -20 } } };

describe("parseGposKerning (REF-OT-GPOS)", () => {
  it("reads the kern feature's lookups once each, in lookup list order", () => {
    // Two kern features (two scripts) list lookups 2, 0 and 0; lookup 1 belongs to another feature.
    const gpos = gposTable(
      [
        { lookups: [2, 0], tag: "kern" },
        { lookups: [1], tag: "mark" },
        { lookups: [0], tag: "kern" },
      ],
      [
        { subtables: [PAIR], type: 2 },
        { subtables: [PAIR], type: 2 },
        { subtables: [OTHER_PAIR], type: 2 },
      ],
    );

    expect(parseGposKerning(viewOf(gpos), GLYPHS)).toEqual([[PAIR_RULE], [OTHER_RULE]]);
  });

  it("follows an extension lookup to its pair adjustment subtables", () => {
    const gpos = gposTable(
      [{ lookups: [0], tag: "kern" }],
      [{ subtables: [PAIR, OTHER_PAIR], type: 9 }],
    );

    expect(parseGposKerning(viewOf(gpos), GLYPHS)).toEqual([[PAIR_RULE, OTHER_RULE]]);
  });

  it("reads the extension's offset as 32 bits from the extension's start", () => {
    // Offsets 8 + 4 = 12, and 8 + 65 536 = 0x00010008, whose low 16 bits alone would be 8.
    const near = gposTable(
      [{ lookups: [0], tag: "kern" }],
      [{ extensionPadding: 4, subtables: [PAIR], type: 9 }],
    );
    const far = gposTable(
      [{ lookups: [0], tag: "kern" }],
      [{ extensionPadding: 0x1_00_00, subtables: [PAIR], type: 9 }],
    );

    expect(parseGposKerning(viewOf(near), GLYPHS)).toEqual([[PAIR_RULE]]);
    expect(parseGposKerning(viewOf(far), GLYPHS)).toEqual([[PAIR_RULE]]);
  });

  it("reads no rule from an extension of another type, a lookup of another type or format", () => {
    // Lookup 0 extends type 1 (single adjustment); lookup 1 is of type 1; lookup 2 is a pair
    // adjustment of format 3.
    const gpos = gposTable(
      [{ lookups: [0, 1, 2], tag: "kern" }],
      [
        { extensionType: 1, subtables: [PAIR], type: 9 },
        { subtables: [PAIR], type: 1 },
        { subtables: [uint16(3)], type: 2 },
      ],
    );

    expect(parseGposKerning(viewOf(gpos), GLYPHS)).toEqual([[], [], []]);
  });

  it("reads nothing from a GPOS table without a kern feature", () => {
    const gpos = gposTable([{ lookups: [0], tag: "cpsp" }], [{ subtables: [PAIR], type: 2 }]);

    expect(parseGposKerning(viewOf(gpos), GLYPHS)).toEqual([]);
  });

  it("gives undefined when a part passes the table's end", () => {
    const gpos = gposTable([{ lookups: [0], tag: "kern" }], [{ subtables: [PAIR], type: 2 }]);

    // Cut by one byte, the subtable's coverage; cut in the lookup list; cut in the feature list.
    expect(parseGposKerning(viewOfFirst(gpos, -1), GLYPHS)).toBeUndefined();
    expect(parseGposKerning(viewOfFirst(gpos, 26), GLYPHS)).toBeUndefined();
    expect(parseGposKerning(viewOfFirst(gpos, 11), GLYPHS)).toBeUndefined();
  });

  it("gives undefined for a kern feature naming a lookup the list lacks", () => {
    const gpos = gposTable([{ lookups: [3], tag: "kern" }], [{ subtables: [PAIR], type: 2 }]);

    expect(parseGposKerning(viewOf(gpos), GLYPHS)).toBeUndefined();
  });
});
