import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { fittedView } from "./fitted-view";

/** The canvas of the examples and the fits' padding, in CSS pixels (SP-030's answer 6). */
const FRAME = { canvas: { height: 800, width: 1200 }, padding: 40 };

/** The view as the page opens (ADR-0098, decision 1). */
const OPENING = { origin: { x: -40, y: -40 }, scale: 1 };

describe("fittedView (DERIV-view-zoom step 7)", () => {
  it("[F42.AC3] fits the box from (10, 20) to (410, 220) at 280 %", () => {
    // s = min(1 120 / 400, 720 / 200) = 2.8; c = (210, 120);
    // o = (210 − 600 / 2.8, 120 − 400 / 2.8) = (−4.28571, −22.85714).
    const view = fittedView(OPENING, { maxX: 410, maxY: 220, minX: 10, minY: 20 }, FRAME);

    expect(view.scale).toBeCloseTo(2.8, 9);
    expect(view.origin.x).toBeCloseTo(-4.28571, 5);
    expect(view.origin.y).toBeCloseTo(-22.85714, 5);
  });

  it("fits a box 0 wide by its height alone", () => {
    // s = 720 / 200 = 3.6; c = (10, 120); o = (10 − 600 / 3.6, 120 − 400 / 3.6).
    const view = fittedView(OPENING, { maxX: 10, maxY: 220, minX: 10, minY: 20 }, FRAME);

    expect(view.scale).toBeCloseTo(3.6, 9);
    expect(view.origin.x).toBeCloseTo(-156.66667, 5);
    expect(view.origin.y).toBeCloseTo(8.88889, 5);
  });

  it("fits a box 0 high by its width alone", () => {
    // s = 1 120 / 400 = 2.8; c = (210, 20); o = (210 − 600 / 2.8, 20 − 400 / 2.8).
    const view = fittedView(OPENING, { maxX: 410, maxY: 20, minX: 10, minY: 20 }, FRAME);

    expect(view.scale).toBeCloseTo(2.8, 9);
    expect(view.origin.x).toBeCloseTo(-4.28571, 5);
    expect(view.origin.y).toBeCloseTo(-122.85714, 5);
  });

  it("centres a box of 0 × 0 at the view's scale", () => {
    // s = 2; o = (50 − 600 / 2, 60 − 400 / 2).
    expect(
      fittedView(
        { origin: { x: 0, y: 0 }, scale: 2 },
        { maxX: 50, maxY: 60, minX: 50, minY: 60 },
        FRAME,
      ),
    ).toEqual({ origin: { x: -250, y: -140 }, scale: 2 });
  });

  it("fits a box 1 × 1 at the upper bound", () => {
    // min(1 120, 720) = 720, taken at 256; o = (0.5 − 600 / 256, 0.5 − 400 / 256).
    expect(fittedView(OPENING, { maxX: 1, maxY: 1, minX: 0, minY: 0 }, FRAME)).toEqual({
      origin: { x: -1.84375, y: -1.0625 },
      scale: 256,
    });
  });

  it("fits at the lower bound in a canvas narrower than its padding", () => {
    // min(−20 / 400, 720 / 200) = −0.05, taken at 0.01; o = (210 − 30 / 0.01, 120 − 400 / 0.01).
    const view = fittedView(
      OPENING,
      { maxX: 410, maxY: 220, minX: 10, minY: 20 },
      { canvas: { height: 800, width: 60 }, padding: 40 },
    );

    expect(view.scale).toBe(0.01);
    expect(view.origin.x).toBeCloseTo(-2790, 6);
    expect(view.origin.y).toBeCloseTo(-39_880, 6);
  });

  test.prop({
    height: fc.integer({ max: 2000, min: 1 }),
    width: fc.integer({ max: 2000, min: 1 }),
    x: fc.integer({ max: 10_000, min: -10_000 }),
    y: fc.integer({ max: 10_000, min: -10_000 }),
  })(
    "[F42.AC3] draws a box within the padded canvas, centred, the inputs unchanged",
    ({ height, width, x, y }) => {
      const box = { maxX: x + width, maxY: y + height, minX: x, minY: y };
      const before = JSON.stringify({ box, view: OPENING });
      const view = fittedView(OPENING, box, FRAME);
      const left = (box.minX - view.origin.x) * view.scale;
      const right = (box.maxX - view.origin.x) * view.scale;
      const top = (box.minY - view.origin.y) * view.scale;
      const bottom = (box.maxY - view.origin.y) * view.scale;

      // Within [40, 1 160] × [40, 760], within a rounding error.
      expect(left).toBeGreaterThanOrEqual(40 - 1e-6);
      expect(right).toBeLessThanOrEqual(1160 + 1e-6);
      expect(top).toBeGreaterThanOrEqual(40 - 1e-6);
      expect(bottom).toBeLessThanOrEqual(760 + 1e-6);
      expect(left + right).toBeCloseTo(1200, 6);
      expect(top + bottom).toBeCloseTo(800, 6);
      expect(JSON.stringify({ box, view: OPENING })).toBe(before);
    },
  );
});
