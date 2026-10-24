import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { shownViewBox } from "./shown-view-box";

/** The canvas of the examples, in CSS pixels. */
const CANVAS = { height: 800, width: 1200 };

describe("shownViewBox (DERIV-view-zoom step 1)", () => {
  it("gives the opening view today's viewBox", () => {
    // (−40, −40, 1 200 / 1, 800 / 1).
    expect(shownViewBox({ origin: { x: -40, y: -40 }, scale: 1 }, CANVAS)).toEqual({
      height: 800,
      width: 1200,
      x: -40,
      y: -40,
    });
  });

  it("shows 600 × 400 units from (180, 130) at scale 2", () => {
    // (180, 130, 1 200 / 2, 800 / 2).
    expect(shownViewBox({ origin: { x: 180, y: 130 }, scale: 2 }, CANVAS)).toEqual({
      height: 400,
      width: 600,
      x: 180,
      y: 130,
    });
  });

  test.prop({
    height: fc.integer({ max: 4000, min: 1 }),
    scale: fc.double({ max: 256, min: 0.01, noNaN: true }),
    width: fc.integer({ max: 4000, min: 1 }),
    x: fc.integer({ max: 100_000, min: -100_000 }),
    y: fc.integer({ max: 100_000, min: -100_000 }),
  })("keeps the canvas's aspect and its scale, the view unchanged", (input) => {
    const view = { origin: { x: input.x, y: input.y }, scale: input.scale };
    const canvas = { height: input.height, width: input.width };
    const before = JSON.stringify({ canvas, view });
    const box = shownViewBox(view, canvas);

    expect(box.width * input.scale).toBeCloseTo(input.width, 6);
    expect(box.height * input.scale).toBeCloseTo(input.height, 6);
    expect(JSON.stringify({ canvas, view })).toBe(before);
  });
});
