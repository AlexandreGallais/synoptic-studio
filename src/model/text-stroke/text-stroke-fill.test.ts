import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textStrokeFill } from "./text-stroke-fill";

/** "Hello World" in black at 100 %, its underline chosen #ff0000 at 50 %, its strike-through not. */
const RED_UNDERLINE = {
  ...HELLO_TEXT,
  hasUnderlineColor: true,
  isStruckThrough: true,
  isUnderlined: true,
  underlineColor: "#ff0000",
  underlineOpacity: 50,
} as const;

describe("textStrokeFill (US-041)", () => {
  it("[F16.AC4] draws a chosen underline in its own color, the strike-through in the text's", () => {
    expect(textStrokeFill(RED_UNDERLINE, "underline")).toEqual({ color: "#ff0000", opacity: 50 });

    expect(textStrokeFill(RED_UNDERLINE, "strikeThrough")).toEqual({
      color: "#000000",
      opacity: 100,
    });
  });

  it("[F16.AC4] draws a chosen strike-through in its own color, the underline in the text's", () => {
    const text = {
      ...HELLO_TEXT,
      hasStrikeThroughColor: true,
      strikeThroughColor: "#00ff00",
      strikeThroughOpacity: 30,
      underlineColor: "#ff0000",
    };

    expect(textStrokeFill(text, "strikeThrough")).toEqual({ color: "#00ff00", opacity: 30 });
    expect(textStrokeFill(text, "underline")).toEqual({ color: "#000000", opacity: 100 });
  });

  it("[F16.AC4] follows the text's color changed to #0000ff when no stroke's color was chosen", () => {
    const text = { ...HELLO_TEXT, color: "#0000ff", opacity: 70, underlineColor: "#ff0000" };

    expect(textStrokeFill(text, "underline")).toEqual({ color: "#0000ff", opacity: 70 });
    expect(textStrokeFill(text, "strikeThrough")).toEqual({ color: "#0000ff", opacity: 70 });
  });

  it("[F16.AC4] keeps a chosen color when the text's color changes", () => {
    const text = { ...RED_UNDERLINE, color: "#00ff00", opacity: 20 };

    expect(textStrokeFill(text, "underline")).toEqual({ color: "#ff0000", opacity: 50 });
  });

  test.prop({
    color: fc.constantFrom("#000000", "#1e293b", "#0000ff"),
    isChosen: fc.boolean(),
    kind: fc.constantFrom("underline", "strikeThrough"),
    opacity: fc.integer({ max: 100, min: 0 }),
  })(
    "[F16.AC4] gives the stroke's own color once chosen, else the text's",
    ({ color, isChosen, kind, opacity }) => {
      const text = {
        ...HELLO_TEXT,
        color,
        hasStrikeThroughColor: isChosen,
        hasUnderlineColor: isChosen,
        opacity,
        strikeThroughColor: "#ff0000",
        strikeThroughOpacity: 40,
        underlineColor: "#ff0000",
        underlineOpacity: 40,
      };

      expect(textStrokeFill(text, kind)).toEqual(
        isChosen ? { color: "#ff0000", opacity: 40 } : { color, opacity },
      );
    },
  );
});
