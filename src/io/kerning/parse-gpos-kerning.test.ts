import { describe, expect, it } from "vitest";

import { X_ADVANCE_ONLY, gposTable, pairPosFormat1, uint16, viewOf } from "../../../tests";

import { parseGposKerning } from "./parse-gpos-kerning";

/** A pair adjustment subtable: glyphs 1 2, −50. */
const PAIR = pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[2, -50]] }]);

/** A pair adjustment subtable: glyphs 1 3, −20. */
const OTHER_PAIR = pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[3, -20]] }]);

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

    expect(parseGposKerning(viewOf(gpos), 0)).toEqual([
      [{ kind: "pairs", pairs: { 1: { 2: -50 } } }],
      [{ kind: "pairs", pairs: { 1: { 3: -20 } } }],
    ]);
  });

  it("follows an extension lookup to its pair adjustment subtables", () => {
    const gpos = gposTable(
      [{ lookups: [0], tag: "kern" }],
      [{ subtables: [PAIR, OTHER_PAIR], type: 9 }],
    );

    expect(parseGposKerning(viewOf(gpos), 0)).toEqual([
      [
        { kind: "pairs", pairs: { 1: { 2: -50 } } },
        { kind: "pairs", pairs: { 1: { 3: -20 } } },
      ],
    ]);
  });

  it("reads no rule from a subtable of another type or format", () => {
    // Lookup 0 is of type 1 (single adjustment); lookup 1 is a pair adjustment of format 3.
    const gpos = gposTable(
      [{ lookups: [0, 1], tag: "kern" }],
      [
        { subtables: [PAIR], type: 1 },
        { subtables: [uint16(3)], type: 2 },
      ],
    );

    expect(parseGposKerning(viewOf(gpos), 0)).toEqual([[], []]);
  });

  it("reads nothing from a GPOS table without a kern feature", () => {
    const gpos = gposTable([{ lookups: [0], tag: "cpsp" }], [{ subtables: [PAIR], type: 2 }]);

    expect(parseGposKerning(viewOf(gpos), 0)).toEqual([]);
  });
});
