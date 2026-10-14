import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { glyphAdvance } from "./glyph-advance";

describe("glyphAdvance (REF-OT-HMTX)", () => {
  it.each([
    { expected: 500, glyph: 0 },
    { expected: 700, glyph: 2 },
    // Glyphs 3 and 4 are past the three records: the last record's advance applies.
    { expected: 700, glyph: 3 },
    { expected: 700, glyph: 4 },
  ])("[F05.AC1] gives $expected for glyph $glyph", ({ expected, glyph }) => {
    expect(glyphAdvance([500, 600, 700], glyph)).toBe(expected);
  });

  it("gives 0 for a font with no record", () => {
    expect(glyphAdvance([], 0)).toBe(0);
  });

  test.prop({
    advances: fc.array(fc.integer({ max: 0xff_ff, min: 0 }), { minLength: 1 }),
    extra: fc.nat({ max: 1000 }),
  })("gives the last record for every glyph past the records", ({ advances, extra }) => {
    expect(glyphAdvance(advances, advances.length - 1 + extra)).toBe(advances.at(-1));
  });
});
