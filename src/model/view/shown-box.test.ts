import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { shownBox } from "./shown-box";

/** The canvas of the examples, in CSS pixels. */
const CANVAS = { height: 800, width: 1200 };

/** No room added. */
const NONE = { height: 0, width: 0 };

/** Half the canvas on each side, the helps' padding (SP-030's answer 7). */
const HALF = { height: 400, width: 600 };

describe("shownBox (DERIV-band-within-box step 1)", () => {
  it("gives the area the opening view shows", () => {
    // From (−40, −40) to (−40 + 1 200, −40 + 800).
    expect(shownBox({ origin: { x: -40, y: -40 }, scale: 1 }, CANVAS, NONE)).toEqual({
      maxX: 1160,
      maxY: 760,
      minX: -40,
      minY: -40,
    });
  });

  it("widens it by half the canvas for the helps, four times its area", () => {
    // (−40 − 600, −40 − 400) to (−40 + 1 800, −40 + 1 200).
    expect(shownBox({ origin: { x: -40, y: -40 }, scale: 1 }, CANVAS, HALF)).toEqual({
      maxX: 1760,
      maxY: 1160,
      minX: -640,
      minY: -440,
    });
  });

  it("divides the canvas and the padding by the scale at 200 %", () => {
    // (180 − 600 / 2, 130 − 400 / 2) to (180 + 1 800 / 2, 130 + 1 200 / 2).
    expect(shownBox({ origin: { x: 180, y: 130 }, scale: 2 }, CANVAS, HALF)).toEqual({
      maxX: 1080,
      maxY: 730,
      minX: -120,
      minY: -70,
    });
  });

  it("gives a canvas of no size its origin alone", () => {
    expect(shownBox({ origin: { x: 5, y: 7 }, scale: 3 }, NONE, NONE)).toEqual({
      maxX: 5,
      maxY: 7,
      minX: 5,
      minY: 7,
    });
  });

  test.prop({
    padding: fc.integer({ max: 2000, min: 0 }),
    scale: fc.double({ max: 256, min: 0.01, noNaN: true }),
    x: fc.integer({ max: 100_000, min: -100_000 }),
    y: fc.integer({ max: 100_000, min: -100_000 }),
  })("holds the area shown, as wide as the canvas and twice the padding at the scale", (input) => {
    const view = { origin: { x: input.x, y: input.y }, scale: input.scale };
    const padding = { height: input.padding, width: input.padding };
    const shown = shownBox(view, CANVAS, NONE);
    const box = shownBox(view, CANVAS, padding);

    expect(box.minX).toBeLessThanOrEqual(shown.minX);
    expect(box.maxY).toBeGreaterThanOrEqual(shown.maxY);
    expect((box.maxX - box.minX) * input.scale).toBeCloseTo(1200 + 2 * input.padding, 4);
    expect((shown.maxY - shown.minY) * input.scale).toBeCloseTo(800, 4);
  });
});
