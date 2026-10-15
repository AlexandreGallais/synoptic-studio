import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { isTextStrokeDrawn } from "./is-text-stroke-drawn";

describe("isTextStrokeDrawn (US-041)", () => {
  it.each([
    { drawn: [true, false], isStruckThrough: false, isUnderlined: true },
    { drawn: [false, true], isStruckThrough: true, isUnderlined: false },
    { drawn: [false, false], isStruckThrough: false, isUnderlined: false },
  ])(
    "[F16.AC4] draws the underline and the strike-through: $drawn",
    ({ drawn, isStruckThrough, isUnderlined }) => {
      const text = { ...HELLO_TEXT, isStruckThrough, isUnderlined };

      expect([
        isTextStrokeDrawn(text, "underline"),
        isTextStrokeDrawn(text, "strikeThrough"),
      ]).toEqual(drawn);
    },
  );
});
