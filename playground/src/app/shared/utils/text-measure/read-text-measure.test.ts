import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../../../testing";

import { readTextMeasure } from "./read-text-measure";

import type { Text } from "editor";

/** Another valid value for each of a text's fields, so that every field can be changed alone. */
const CHANGED: { readonly [Name in keyof Text]: Text[Name] } = {
  alignment: "right",
  angle: 30,
  backgroundColor: "#00ff00",
  backgroundOpacity: 50,
  boxHeight: 40,
  boxWidth: 30,
  color: "#ff0000",
  content: "Hello World again",
  direction: "rtl",
  fallback: "serif",
  font: "Arial",
  hasBackground: true,
  hasEllipsis: false,
  hasStrikeThroughColor: true,
  hasUnderlineColor: true,
  isInBox: true,
  isItalic: true,
  isStruckThrough: true,
  isTrimmed: true,
  isUnderlined: true,
  letterSpacing: 3,
  lineHeight: 30,
  maximumLines: 1,
  opacity: 50,
  size: 20,
  strikeThroughColor: "#0000ff",
  strikeThroughOpacity: 50,
  textCase: "upper",
  underlineColor: "#0000ff",
  underlineOpacity: 50,
  verticalAlignment: "bottom",
  weight: 700,
  wordSpacing: 3,
  x: 10,
  y: 10,
};

/** The fields that play no part in a text's lines and block: where it is, its angle, its paint. */
const PLACEMENT_AND_PAINT: ReadonlySet<string> = new Set([
  "angle",
  "backgroundColor",
  "backgroundOpacity",
  "color",
  "hasStrikeThroughColor",
  "hasUnderlineColor",
  "opacity",
  "strikeThroughColor",
  "strikeThroughOpacity",
  "underlineColor",
  "underlineOpacity",
  "x",
  "y",
]);

describe("readTextMeasure", () => {
  it("has a changed value for every field of a text", () => {
    expect(new Set(Object.keys(CHANGED))).toEqual(new Set(Object.keys(HELLO_TEXT)));
  });

  it.each(Object.entries(CHANGED))(
    "[F17.AC5] measures again when %s changes, unless it is placement or paint",
    (name, value) => {
      const text: Text = { ...HELLO_TEXT, boxHeight: 100, boxWidth: 60 };
      const changed: Text = { ...text, [name]: value };
      const before = readTextMeasure(ARIAL_BOX_METRICS, text);

      // The measure kept is given back, the very object, only for a field that does not measure.
      expect(readTextMeasure(ARIAL_BOX_METRICS, changed) === before).toBe(
        PLACEMENT_AND_PAINT.has(name),
      );
    },
  );
});
