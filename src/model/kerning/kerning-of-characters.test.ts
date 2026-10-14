import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { glyphPairKerning } from "./glyph-pair-kerning";
import { glyphPairKey } from "./glyph-pair-key";
import { kerningOfCharacters } from "./kerning-of-characters";
import { pairKerning } from "./pair-kerning";

import type { CharacterGlyph } from "../font";
import type { KerningLookup } from "./kerning-lookup";

/** A, V, T, o mapped to glyphs 1 to 4. */
const GLYPHS: readonly CharacterGlyph[] = [
  [65, 1],
  [86, 2],
  [84, 3],
  [111, 4],
];

/** A V −80, V A −80, A T −40, T A −40, T o −60. */
const LOOKUPS: readonly KerningLookup[] = [
  [
    {
      kind: "pairs",
      pairs: {
        [glyphPairKey(1, 2)]: -80,
        [glyphPairKey(1, 3)]: -40,
        [glyphPairKey(2, 1)]: -80,
        [glyphPairKey(3, 1)]: -40,
        [glyphPairKey(3, 4)]: -60,
      },
    },
  ],
];

/** A glyph from 1 to 6. */
const GLYPH = fc.integer({ max: 6, min: 1 });

/** A kerning pair of those glyphs. */
const PAIR = fc.record({ first: GLYPH, second: GLYPH, value: fc.integer({ max: 200, min: -200 }) });

describe("kerningOfCharacters (REF-OT-GPOS)", () => {
  it("groups characters that kern alike, and keeps each pair's kerning", () => {
    const kerning = kerningOfCharacters(LOOKUPS, GLYPHS);

    // Rows (left A, V, T, o): A (0, −80, −40, 0), V (−80, 0, 0, 0), T (−40, 0, 0, −60), o zeros:
    // classes 1, 2, 3, 0. Columns (right A, V, T, o): A (0, −80, −40, 0), V (−80, 0, 0, 0),
    // T (−40, 0, 0, 0), o (0, 0, −60, 0): classes 1, 2, 3, 4.
    expect(kerning).toEqual({
      leftClasses: { 65: 1, 84: 3, 86: 2 },
      rightClasses: { 65: 1, 84: 3, 86: 2, 111: 4 },
      values: [
        [0, 0, 0, 0, 0],
        [0, 0, -80, -40, 0],
        [0, -80, 0, 0, 0],
        [0, -40, 0, 0, -60],
      ],
    });

    expect(pairKerning(kerning, 84, 111)).toBe(-60);
  });

  it("gives one class 0 to characters that kern with nothing", () => {
    expect(kerningOfCharacters([], GLYPHS)).toEqual({
      leftClasses: {},
      rightClasses: {},
      values: [[0]],
    });
  });

  it("gives one class 0 and its one value for no character at all", () => {
    expect(kerningOfCharacters([], [])).toEqual({
      leftClasses: {},
      rightClasses: {},
      values: [[0]],
    });
  });

  test.prop({ pairs: fc.array(PAIR, { maxLength: 20 }) })(
    "gives every pair of characters the kerning of their glyphs",
    ({ pairs }) => {
      const glyphs = Array.from({ length: 6 }, (unused, index): CharacterGlyph => [
        64 + index,
        index + 1,
      ]);
      const lookups: readonly KerningLookup[] = pairs.map((pair) => [
        { kind: "pairs", pairs: { [glyphPairKey(pair.first, pair.second)]: pair.value } },
      ]);
      const kerning = kerningOfCharacters(lookups, glyphs);

      for (const [left, first] of glyphs) {
        for (const [right, second] of glyphs) {
          expect(pairKerning(kerning, left, right)).toBe(glyphPairKerning(lookups, first, second));
        }
      }
    },
  );
});
