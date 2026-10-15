import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { characterUnits } from "../line-characters";
import { lineWidth } from "../text-block";

import { ellipsisCut } from "./ellipsis-cut";
import { textEllipsis } from "./text-ellipsis";
import { withoutTrailingSpaces } from "./without-trailing-spaces";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** Arial 16 without kerning in a box 60 wide. */
const BOXED: Text = { ...HELLO_TEXT, boxHeight: 1000, boxWidth: 60, isInBox: true };

/** The same face without a glyph for U+2026, which then writes three full stops. */
const WITHOUT_ELLIPSIS: FontMetrics = {
  ...ARIAL_BOX_METRICS,
  advances: Object.fromEntries(
    Object.entries(ARIAL_BOX_METRICS.advances).filter(([codePoint]) => codePoint !== "8230"),
  ),
};

/** A line cut with its ellipsis, and its width. */
type MeasuredCut = {
  /** The line cut, its ellipsis after it. */
  readonly line: string;
  /** Its width in user units. */
  readonly width: number;
};

/** The ellipsis U+2026. */
const ELLIPSIS = "\u{2026}";

/**
 * A line cut with an ellipsis in a box of a width, and its width.
 *
 * @param line - a line shown, in the text's case
 * @param boxWidth - how wide the box is
 * @param metrics - the metrics of the face it is drawn with
 * @returns the line cut and its width
 */
function cut(line: string, boxWidth: number, metrics: FontMetrics): MeasuredCut {
  const text = { ...BOXED, boxWidth };
  const ellipsized = ellipsisCut({ metrics, text }, line) + textEllipsis(metrics);

  return { line: ellipsized, width: lineWidth(metrics, ellipsized, text) };
}

describe("ellipsisCut (DERIV-text-box step 6)", () => {
  it("[F14.AC2] ends a line that fits with its ellipsis", () => {
    // "Hello" + "…" = (4667 + 2048) / 128 = 52.46094 ≤ 60; "World…" (5348 + 2048) / 128 = 57.78125.
    expect(cut("Hello", 60, ARIAL_BOX_METRICS)).toEqual({ line: "Hello…", width: 52.4609375 });
    expect(cut("World", 60, ARIAL_BOX_METRICS)).toEqual({ line: "World…", width: 57.78125 });
  });

  it("[F14.AC2] removes characters from the end until the line and its ellipsis fit", () => {
    // 52.46094 > 50; without o: (4667 − 1139 + 2048) / 128 = 43.5625 ≤ 50.
    expect(cut("Hello", 50, ARIAL_BOX_METRICS)).toEqual({ line: "Hell…", width: 43.5625 });
    // "runni…" (6832 − 2278 + 2048) / 128 = 51.57813 > 50; "runn…" 6147 / 128 = 48.02344.
    expect(cut("running", 50, ARIAL_BOX_METRICS)).toEqual({ line: "runn…", width: 48.0234375 });
    // "P-101…" 7513 / 128 = 58.69531 > 50; "P-10…" 6374 / 128 = 49.79688.
    expect(cut("P-101", 50, ARIAL_BOX_METRICS)).toEqual({ line: "P-10…", width: 49.796875 });
  });

  it("[F14.AC2] removes the spaces before a character removed, and those ending the line", () => {
    // "Hello W…" (4667 + 569 + 1933 + 2048) / 128 = 72.00781 > 60, so W goes with the space before
    // it: "Hello…", never "Hello …".
    expect(cut("Hello World", 60, ARIAL_BOX_METRICS).line).toBe("Hello…");
    expect(cut("Hello  ", 60, ARIAL_BOX_METRICS).line).toBe("Hello…");
  });

  it("[F14.AC2] keeps the first character, and the ellipsis, past the box's edge", () => {
    // (1479 + 2048) / 128 = 27.55469 > 10: the first character stays (REF-CSS-OVERFLOW-3).
    expect(cut("Hello", 10, ARIAL_BOX_METRICS)).toEqual({ line: "H…", width: 27.5546875 });
  });

  it("[F14.AC2] never counts the spaces opening a line as the character kept", () => {
    expect(cut("  Hello", 10, ARIAL_BOX_METRICS).line).toBe("  H…");
  });

  it("[F14.AC2] never keeps a start ending with a no-break space", () => {
    // In a box 70: "Hello\u{A0}W…" (4667 + 569 + 1933 + 2048) / 128 = 72.00781 > 70;
    // "Hello\u{A0}…" 56.91 would fit, but the no-break space stays with "W": "Hello…", 52.46094.
    expect(cut("Hello\u{A0}World", 70, ARIAL_BOX_METRICS)).toEqual({
      line: "Hello…",
      width: 52.4609375,
    });

    // In a box 75 "Hello\u{A0}W…" fits.
    expect(cut("Hello\u{A0}World", 75, ARIAL_BOX_METRICS).line).toBe("Hello\u{A0}W…");
    // The first unit kept takes the no-break spaces before it.
    expect(cut("\u{A0}\u{A0}Hello", 10, ARIAL_BOX_METRICS).line).toBe("\u{A0}\u{A0}H…");
  });

  it("[F14.AC2] gives a line of spaces, or an empty one, its ellipsis alone", () => {
    expect(cut("", 60, ARIAL_BOX_METRICS).line).toBe(ELLIPSIS);
    expect(cut(" ".repeat(3), 60, ARIAL_BOX_METRICS).line).toBe(ELLIPSIS);
  });

  it("[F14.AC2] writes three full stops when the face lacks the ellipsis", () => {
    // "..." 3 × 569 / 128 = 13.33594; "Hello..." (4667 + 1707) / 128 = 49.79688 ≤ 50.
    expect(cut("", 60, WITHOUT_ELLIPSIS)).toEqual({ line: "...", width: 13.3359375 });
    expect(cut("Hello", 50, WITHOUT_ELLIPSIS)).toEqual({ line: "Hello...", width: 49.796875 });
    // In a box 45: "Hell..." (3528 + 1707) / 128 = 40.89844.
    expect(cut("Hello", 45, WITHOUT_ELLIPSIS)).toEqual({ line: "Hell...", width: 40.8984375 });
  });

  test.prop({
    boxWidth: fc.integer({ max: 80, min: 0 }),
    letterSpacing: fc.integer({ max: 5, min: -5 }),
    line: fc.stringMatching(/^(?:[ HWdelor\u{A0}]|\u{301}){0,30}$/u),
  })(
    "[F14.AC2] cuts a line no wider than the box unless one character is left, never after a no-break space",
    ({ boxWidth, letterSpacing, line }) => {
      const text = { ...BOXED, boxWidth, letterSpacing };
      const kept = ellipsisCut({ metrics: ARIAL_BOX_METRICS, text }, line);
      const ellipsized = kept + ELLIPSIS;
      const units = characterUnits(kept).filter((unit) => unit !== " " && unit !== "\u{A0}");

      expect(kept.endsWith("\u{A0}")).toBe(false);
      // What is kept opens the line, without trailing spaces.
      expect(withoutTrailingSpaces(line).startsWith(kept)).toBe(true);
      expect(withoutTrailingSpaces(kept)).toBe(kept);

      expect(units.length <= 1 || lineWidth(ARIAL_BOX_METRICS, ellipsized, text) <= boxWidth).toBe(
        true,
      );
    },
  );
});
