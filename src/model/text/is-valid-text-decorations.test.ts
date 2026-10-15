import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { TEXT_DECORATION_DEFAULTS } from "../text-stroke";

import { isValidText } from "./is-valid-text";
import { isValidTextDecorations } from "./is-valid-text-decorations";

/** "Hello World", underlined and struck through, both strokes in colors of their own. */
const CHOSEN = {
  ...HELLO_TEXT,
  hasStrikeThroughColor: true,
  hasUnderlineColor: true,
  isStruckThrough: true,
  isUnderlined: true,
  strikeThroughColor: "#000000",
  strikeThroughOpacity: 100,
  underlineColor: "#ff0000",
  underlineOpacity: 50,
} as const;

describe("isValidTextDecorations (US-041)", () => {
  it("[F16.AC4] gives a text of F05 strokes that follow its color, black and opaque kept", () => {
    expect(TEXT_DECORATION_DEFAULTS).toEqual({
      hasStrikeThroughColor: false,
      hasUnderlineColor: false,
      strikeThroughColor: "#000000",
      strikeThroughOpacity: 100,
      underlineColor: "#000000",
      underlineOpacity: 100,
    });

    expect(isValidText({ ...HELLO_TEXT, ...TEXT_DECORATION_DEFAULTS })).toBe(true);
  });

  it("[F16.AC1] accepts an underline in #ff0000 at 50 % and a strike-through in #000000", () => {
    expect(isValidTextDecorations(CHOSEN)).toBe(true);
    expect(isValidText(CHOSEN)).toBe(true);
  });

  it.each([0, 100])("[F16.AC1] accepts %d %, an end of 0 .. 100, as each opacity", (opacity) => {
    const text = { ...CHOSEN, strikeThroughOpacity: opacity, underlineOpacity: opacity };

    expect(isValidTextDecorations(text)).toBe(true);
  });

  it.each(["#FF0000", "red", "#ff000", "ff0000", "#ff0000ff", ""])(
    "[F16.AC1] refuses '%s' as the underline's or the strike-through's color",
    (color) => {
      expect(isValidTextDecorations({ ...CHOSEN, underlineColor: color })).toBe(false);
      expect(isValidTextDecorations({ ...CHOSEN, strikeThroughColor: color })).toBe(false);
      expect(isValidText({ ...CHOSEN, underlineColor: color })).toBe(false);
    },
  );

  it.each([-1, 101, 50.5, NaN])(
    "[F16.AC1] refuses %d % as the underline's or the strike-through's opacity",
    (opacity) => {
      expect(isValidTextDecorations({ ...CHOSEN, underlineOpacity: opacity })).toBe(false);
      expect(isValidTextDecorations({ ...CHOSEN, strikeThroughOpacity: opacity })).toBe(false);
      expect(isValidText({ ...CHOSEN, strikeThroughOpacity: opacity })).toBe(false);
    },
  );

  it("[F16.AC1] refuses a color left unchosen that is no stored color, as read from outside", () => {
    const text = { ...HELLO_TEXT, hasUnderlineColor: false, underlineColor: "red" };

    expect(isValidText(text)).toBe(false);
  });

  it.each(["hasUnderlineColor", "hasStrikeThroughColor"] as const)(
    "[F16.AC1] refuses a %s that is not true or false",
    (key) => {
      // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- a value read from outside the type
      const text = { ...CHOSEN, [key]: "yes" as unknown as boolean };
      // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- a value read from outside the type
      const missing = { ...CHOSEN, [key]: undefined as unknown as boolean };

      expect(isValidTextDecorations(text)).toBe(false);
      expect(isValidTextDecorations(missing)).toBe(false);
    },
  );

  test.prop({
    isChosen: fc.boolean(),
    opacity: fc.integer({ max: 100, min: 0 }),
    rgb: fc.integer({ max: 0xff_ff_ff, min: 0 }),
  })(
    "[F16.AC1] accepts every stored color at every whole opacity",
    ({ isChosen, opacity, rgb }) => {
      const color = `#${rgb.toString(16).padStart(6, "0")}`;
      const text = {
        ...HELLO_TEXT,
        hasStrikeThroughColor: isChosen,
        hasUnderlineColor: !isChosen,
        strikeThroughColor: color,
        strikeThroughOpacity: opacity,
        underlineColor: color,
        underlineOpacity: opacity,
      };

      expect(isValidTextDecorations(text)).toBe(true);
    },
  );
});
