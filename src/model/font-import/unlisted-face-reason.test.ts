import { describe, expect, it } from "vitest";

import { ARIAL_METRICS } from "../../../tests";

import { BASIC_LATIN_LETTERS } from "./basic-latin-letters";
import { unlistedFaceReason } from "./unlisted-face-reason";

import type { FontFace, FontFile } from "../font";

/** Arial's metrics with the 52 letters of the ISO basic Latin alphabet mapped. */
const LATIN_METRICS = {
  ...ARIAL_METRICS,
  advances: Object.fromEntries(BASIC_LATIN_LETTERS.map((codePoint) => [codePoint, 1139])),
};

/** Arial's metrics as Apple Color Emoji maps the Latin script: the space and digits, no letter. */
const NO_LETTER_METRICS = { ...ARIAL_METRICS, advances: { 32: 800, 48: 800 } };

/**
 * A file whose `fsType` permits embedding or not.
 *
 * @param isEmbeddable - true for an `fsType` of 0, 4 or 8, false for 2
 * @returns a file of one byte, its version empty
 */
function file(isEmbeddable: boolean): FontFile {
  return { bytes: Uint8Array.from([0]), isEmbeddable, version: "" };
}

/**
 * A regular face.
 *
 * @param metrics - its advances, listing the characters its font maps
 * @param held - its file; undefined for metrics shipped without it, as Inter's
 * @returns an upright face of weight 400
 */
function face(metrics: FontFace["metrics"], held: FontFile | undefined): FontFace {
  return { file: held, isItalic: false, metrics, weight: 400 };
}

describe("unlistedFaceReason (docs/domain/shapes.md, Installed fonts)", () => {
  it.each([
    // Helvetica: fsType 0, every letter mapped.
    {
      case: "embeddable with the letters",
      expected: undefined,
      tested: face(LATIN_METRICS, file(true)),
    },
    // Inter's shipped metrics: no file to judge, every letter mapped.
    { case: "no file, the letters", expected: undefined, tested: face(LATIN_METRICS, undefined) },
    // fsType 2, restricted licence.
    {
      case: "not embeddable",
      expected: "not-embeddable",
      tested: face(LATIN_METRICS, file(false)),
    },
    // Apple Color Emoji: fsType 4 permits embedding, no letter mapped.
    {
      case: "no letter",
      expected: "no-latin-letters",
      tested: face(NO_LETTER_METRICS, file(true)),
    },
    // Both: the licence is said first.
    { case: "neither", expected: "not-embeddable", tested: face(NO_LETTER_METRICS, file(false)) },
  ])("[F17.AC14] gives $expected for a face $case", ({ expected, tested }) => {
    expect(unlistedFaceReason(tested)).toBe(expected);
  });
});
