import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS } from "../../../tests";

import { DEFAULT_FONT } from "./default-font";
import { matchingFace } from "./matching-face";

import type { FontFace } from "./font-face";

/**
 * A face of a weight and a style, its metrics left aside.
 *
 * @param weight - from 100 to 900
 * @param isItalic - `true` for an italic face
 * @returns the face, with Arial's metrics
 */
function face(weight: number, isItalic: boolean): FontFace {
  return { isItalic, metrics: ARIAL_METRICS, weight };
}

/** A family of two upright faces only, 300 and 700. */
const LIGHT_AND_BOLD = [face(300, false), face(700, false)];

/**
 * The weight and style of the face chosen.
 *
 * @param faces - the family's faces
 * @param weight - the weight asked
 * @param isItalic - the style asked
 * @returns the face's weight, then `i` when italic; `none` without a face
 */
function chosen(faces: readonly FontFace[], weight: number, isItalic: boolean): string {
  const found = matchingFace(faces, { isItalic, weight });

  return found === undefined ? "none" : `${String(found.weight)}${found.isItalic ? "i" : ""}`;
}

describe("matchingFace (REF-CSS-FONTS-4, §5.2)", () => {
  it.each([
    // Inter has every weight from 100 to 900: the weight asked, in the style asked.
    { expected: "400", isItalic: false, weight: 400 },
    { expected: "700i", isItalic: true, weight: 700 },
    // 450, from 400 to 500: the heavier weights up to 500 first.
    { expected: "500", isItalic: false, weight: 450 },
    // 350, under 400: the lighter ones first, 300.
    { expected: "300", isItalic: false, weight: 350 },
    // 550, over 500: the heavier ones first, 600.
    { expected: "600i", isItalic: true, weight: 550 },
  ])(
    "[F05.AC2] measures Inter $weight with its face $expected",
    ({ expected, isItalic, weight }) => {
      expect(chosen(DEFAULT_FONT.faces, weight, isItalic)).toBe(expected);
    },
  );

  it.each([
    // 400: nothing from 400 to 500, then the lighter ones in descending order: 300.
    { expected: "300", weight: 400 },
    { expected: "300", weight: 450 },
    { expected: "300", weight: 500 },
    // 200: nothing under it, then the heavier ones in ascending order: 300.
    { expected: "300", weight: 200 },
    // 600: the heavier ones first: 700.
    { expected: "700", weight: 600 },
    // 800: nothing above it, then the lighter ones in descending order: 700.
    { expected: "700", weight: 800 },
  ])("chooses $expected for $weight in a family of 300 and 700", ({ expected, weight }) => {
    expect(chosen(LIGHT_AND_BOLD, weight, false)).toBe(expected);
  });

  it("chooses past 500 last for a weight from 400 to 500", () => {
    // 450 among 300 and 600: under it comes before past 500.
    expect(chosen([face(600, false), face(300, false)], 450, false)).toBe("300");
    // 450 among 500 and 300: up to 500 first.
    expect(chosen([face(300, false), face(500, false)], 450, false)).toBe("500");
    // 450 among 600 alone: past 500 when nothing else is left.
    expect(chosen([face(600, false)], 450, false)).toBe("600");
  });

  it("chooses the other style when the family has none of the style asked", () => {
    expect(chosen(LIGHT_AND_BOLD, 700, true)).toBe("700");
    expect(chosen([face(400, true), face(700, false)], 400, false)).toBe("700");
  });

  it("chooses the style before the weight", () => {
    // Italic 400 asked: the italic 900, though an upright 400 exists.
    expect(chosen([face(400, false), face(900, true)], 400, true)).toBe("900i");
  });

  it("chooses nothing in a family without faces", () => {
    expect(chosen([], 400, false)).toBe("none");
  });

  test.prop({
    isItalic: fc.boolean(),
    weight: fc.integer({ max: 900, min: 100 }),
    weights: fc.uniqueArray(fc.integer({ max: 9, min: 1 }), { maxLength: 9, minLength: 1 }),
  })(
    "chooses the weight asked when the family has it, else a face of the family",
    ({ isItalic, weight, weights }) => {
      const faces = weights.map((hundreds) => face(100 * hundreds, isItalic));
      const found = matchingFace(faces, { isItalic, weight });

      expect(found !== undefined && faces.includes(found)).toBe(true);
      expect(faces.some((candidate) => candidate.weight === weight)).toBe(found?.weight === weight);
    },
  );
});
