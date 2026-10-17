import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS } from "../../../tests";

import { BASIC_LATIN_LETTERS } from "./basic-latin-letters";
import { hasBasicLatinLetters } from "./has-basic-latin-letters";

import type { FontMetrics } from "../font";

/**
 * Metrics whose font maps the code points given, each 500 units wide.
 *
 * @param codePoints - the characters mapped
 * @returns Arial's metrics with those advances alone
 */
function metricsMapping(codePoints: readonly number[]): FontMetrics {
  return {
    ...ARIAL_METRICS,
    advances: Object.fromEntries(codePoints.map((codePoint) => [codePoint, 500])),
  };
}

describe("BASIC_LATIN_LETTERS (REF-WIKI-ISO-BASIC-LATIN)", () => {
  it("[F17.AC14] holds A to Z then a to z: 0x41 to 0x5A, 0x61 to 0x7A, 52 letters", () => {
    expect(String.fromCodePoint(...BASIC_LATIN_LETTERS)).toBe(
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
    );
  });
});

describe("hasBasicLatinLetters (REF-WIKI-ISO-BASIC-LATIN)", () => {
  it("[F17.AC14] tells a font mapping the 52 letters", () => {
    expect(hasBasicLatinLetters(metricsMapping(BASIC_LATIN_LETTERS))).toBe(true);
  });

  it.each([
    // A color emoji font maps the space, #, * and the digits for its keycaps, no letter.
    { case: "digits and the space, as a color emoji font", codePoints: [0x20, 0x23, 0x2a, 0x30] },
    // Arial's derivation subset: H e l o W r d A V T R, far from 52 letters.
    {
      case: "Arial's measured subset",
      codePoints: Object.keys(ARIAL_METRICS.advances).map(Number),
    },
    // Every letter but z (0x7A): one missing is enough.
    { case: "every letter but z", codePoints: BASIC_LATIN_LETTERS.slice(0, -1) },
    // Every letter but A (0x41).
    { case: "every letter but A", codePoints: BASIC_LATIN_LETTERS.slice(1) },
    // None at all.
    { case: "no character", codePoints: [] },
  ])("[F17.AC14] refuses $case", ({ codePoints }) => {
    expect(hasBasicLatinLetters(metricsMapping(codePoints))).toBe(false);
  });

  test.prop({
    letter: fc.constantFrom(...BASIC_LATIN_LETTERS),
    more: fc.array(fc.integer({ max: 0x10_ff_ff, min: 0 })),
  })("needs every letter, whatever else the font maps", ({ letter, more }) => {
    const all = [...BASIC_LATIN_LETTERS, ...more];

    expect(hasBasicLatinLetters(metricsMapping(all))).toBe(true);

    const withoutLetter = all.filter((codePoint) => codePoint !== letter);

    expect(hasBasicLatinLetters(metricsMapping(withoutLetter))).toBe(false);
  });
});
