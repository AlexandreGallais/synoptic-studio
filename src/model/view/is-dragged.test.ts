import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isDragged } from "./is-dragged";

/** The press of the examples. */
const PRESS = { x: 10, y: 10 };

describe("isDragged (SP-030's answer 8)", () => {
  it.each([
    // At 800 %, 0.375 unit is 0.375 · 8 = 3 pixels: still a click; 0.875 is 7 pixels: a drag.
    { expected: false, pointer: { x: 10.375, y: 10 }, scale: 8 },
    { expected: true, pointer: { x: 10.875, y: 10 }, scale: 8 },
    // At 10 %, 10 units are 1 pixel, a tremble: a click; 40 units are 4 pixels: a drag.
    { expected: false, pointer: { x: 20, y: 10 }, scale: 0.1 },
    { expected: true, pointer: { x: 50, y: 10 }, scale: 0.1 },
    // At 10 %, 30 units are 3 pixels, 30 · 0.1 = 3.0000000000000004 in doubles: taken as 3.
    { expected: false, pointer: { x: 40, y: 10 }, scale: 0.1 },
    // At 100 %, (2.4, 1.8) away is √(5.76 + 3.24) = 3 pixels: a click; (2.4, 1.9) is 3.06.
    { expected: false, pointer: { x: 12.4, y: 11.8 }, scale: 1 },
    { expected: true, pointer: { x: 12.4, y: 11.9 }, scale: 1 },
    // Up or left as well as down or right: 4 pixels up.
    { expected: true, pointer: { x: 10, y: 6 }, scale: 1 },
    // The press itself.
    { expected: false, pointer: PRESS, scale: 256 },
  ])("tells $pointer at $scale a drag: $expected", ({ expected, pointer, scale }) => {
    expect(isDragged(PRESS, pointer, scale)).toBe(expected);
  });

  test.prop({
    pixels: fc.double({ max: 2.999, min: 0, noNaN: true }),
    scale: fc.double({ max: 256, min: 0.01, noNaN: true }),
  })("keeps a click within 3 pixels at any zoom", ({ pixels, scale }) => {
    expect(isDragged(PRESS, { x: PRESS.x + pixels / scale, y: PRESS.y }, scale)).toBe(false);

    expect(isDragged(PRESS, { x: PRESS.x, y: PRESS.y - (pixels + 3.01) / scale }, scale)).toBe(
      true,
    );
  });
});
