import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, ARIAL_UNKERNED } from "../../../tests";

import { joinedLineMeasure } from "./joined-line-measure";
import { lineMeasureWidth } from "./line-measure-width";
import { lineWidth } from "./line-width";
import { measuredLine } from "./measured-line";

/** Arial 16 without spacing. */
const SPACING = { letterSpacing: 0, size: 16, wordSpacing: 0 };

describe("joinedLineMeasure (DERIV-text-width step 2)", () => {
  it("adds the kerning between the left line's last character and the right one's first", () => {
    // "W" 1933 and "o" 1139, kerned W o −37: 3035 units, 3035 / 128 = 23.71094.
    const joined = joinedLineMeasure(
      ARIAL_METRICS,
      measuredLine(ARIAL_METRICS, "W"),
      measuredLine(ARIAL_METRICS, "o"),
    );

    expect(joined).toEqual(measuredLine(ARIAL_METRICS, "Wo"));
    expect(lineMeasureWidth(ARIAL_METRICS, joined, SPACING)).toBe(23.7109375);
  });

  it("adds the letter spacing at the junction and the word spacing of both lines", () => {
    // "Hello " and "World": 11 units, 10 letter spacings of 2, one space with a word spacing
    // of 3: 82.6875 + 20 + 3 = 105.6875.
    const spacing = { letterSpacing: 2, size: 16, wordSpacing: 3 };
    const joined = joinedLineMeasure(
      ARIAL_UNKERNED,
      measuredLine(ARIAL_UNKERNED, "Hello "),
      measuredLine(ARIAL_UNKERNED, "World"),
    );

    expect(lineMeasureWidth(ARIAL_UNKERNED, joined, spacing)).toBe(105.6875);
  });

  it("keeps the measure of a line joined with an empty one, or an empty one with a line", () => {
    const hello = measuredLine(ARIAL_METRICS, "Hello");
    const empty = measuredLine(ARIAL_METRICS, "");

    expect(joinedLineMeasure(ARIAL_METRICS, hello, empty)).toEqual(hello);
    expect(joinedLineMeasure(ARIAL_METRICS, empty, hello)).toEqual(hello);
  });

  it("measures again a right line opening with a combining mark, composed with the left one", () => {
    // "e" then a combining acute: one unit "é", measured as composed.
    const joined = joinedLineMeasure(
      ARIAL_METRICS,
      measuredLine(ARIAL_METRICS, "e"),
      measuredLine(ARIAL_METRICS, "\u{301}"),
    );

    expect(joined).toEqual(measuredLine(ARIAL_METRICS, "e\u{301}"));
  });

  test.prop({
    left: fc.stringMatching(/^(?:[ AHTVWdelor\u{A0}\u{200D}]|\u{301}){0,12}$/u),
    letterSpacing: fc.integer({ max: 5, min: -15 }),
    right: fc.stringMatching(/^(?:[ AHTVWdelor\u{A0}\u{200D}]|\u{301}){0,12}$/u),
    wordSpacing: fc.integer({ max: 5, min: -15 }),
  })("gives the width of the joined line measured whole, kerning included", (values) => {
    const spacing = { ...values, size: 16 };
    const joined = joinedLineMeasure(
      ARIAL_METRICS,
      measuredLine(ARIAL_METRICS, values.left),
      measuredLine(ARIAL_METRICS, values.right),
    );

    expect(lineMeasureWidth(ARIAL_METRICS, joined, spacing)).toBe(
      lineWidth(ARIAL_METRICS, values.left + values.right, spacing),
    );

    expect(joined.text).toBe(values.left + values.right);
  });
});
