import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { pieceLengthBound } from "./piece-length-bound";
import { pieceStepCount } from "./piece-step-count";

import type { Arc, Cubic } from "../contour";

/** R's top-left corner: a quarter of radius 49 999.5 from (0, 49 999.5) up to (49 999.5, 0). */
const CORNER: Arc = {
  center: { x: 49_999.5, y: 49_999.5 },
  end: { x: 49_999.5, y: 0 },
  kind: "arc",
  positive: true,
  radius: 49_999.5,
  start: { x: 0, y: 49_999.5 },
};

describe("pieceStepCount (DERIV-band-within-box)", () => {
  it("gives a segment one chord, itself, whatever its length", () => {
    expect(
      pieceStepCount({ end: { x: 100, y: 0 }, kind: "segment", start: { x: 0, y: 0 } }, 2),
    ).toBe(1);
  });

  it("cuts a quarter of radius 10 into 4 chords of at most 4", () => {
    // 10 · π / 2 = 15.708; ⌈15.708 / 4⌉ = ⌈3.93⌉ = 4.
    const quarter: Arc = {
      center: { x: 0, y: 0 },
      end: { x: 0, y: 10 },
      kind: "arc",
      positive: true,
      radius: 10,
      start: { x: 10, y: 0 },
    };

    expect(pieceStepCount(quarter, 4)).toBe(4);
  });

  it("cuts R's corner into 39 270 chords of 2 at 100 %, 283 at a scale of 0.0072", () => {
    // ⌈78 539.03 / 2⌉ = ⌈39 269.5⌉ = 39 270; ⌈78 539.03 / (2 / 0.0072)⌉ = ⌈282.74⌉ = 283.
    expect(pieceStepCount(CORNER, 2)).toBe(39_270);
    expect(pieceStepCount(CORNER, 2 / 0.0072)).toBe(283);
  });

  it("cuts a cubic by three times its longest side: 11 chords of at most 2", () => {
    // 3 · 7 = 21; ⌈21 / 2⌉ = 11.
    const cubic: Cubic = {
      end: { x: 10, y: 4 },
      first: { x: 3, y: 0 },
      kind: "cubic",
      second: { x: 3, y: 4 },
      start: { x: 0, y: 0 },
    };

    expect(pieceStepCount(cubic, 2)).toBe(11);
  });

  it("gives a curve of no length one chord", () => {
    const point = { x: 5, y: 5 };

    expect(
      pieceStepCount({ end: point, first: point, kind: "cubic", second: point, start: point }, 2),
    ).toBe(1);
  });
});

describe("pieceStepCount (properties)", () => {
  /** A point on integers of a drawing. */
  const point = fc.record({
    x: fc.integer({ max: 100, min: -100 }),
    y: fc.integer({ max: 100, min: -100 }),
  });

  test.prop({
    points: fc.tuple(point, point, point, point),
    step: fc.double({ max: 50, min: 0.5, noNaN: true }),
  })("gives a curve at least one chord, the bound over their number at most the step", (input) => {
    const [start, first, second, end] = input.points;
    const cubic: Cubic = { end, first, kind: "cubic", second, start };
    const count = pieceStepCount(cubic, input.step);

    expect(Number.isSafeInteger(count)).toBe(true);
    expect(count).toBeGreaterThanOrEqual(1);
    expect(pieceLengthBound(cubic) / count).toBeLessThanOrEqual(input.step);
    // The fewest such: one chord less would be over the step.
    expect(count === 1 || pieceLengthBound(cubic) / (count - 1) > input.step).toBe(true);
  });
});
