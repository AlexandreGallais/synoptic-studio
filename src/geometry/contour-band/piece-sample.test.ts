import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { pieceSample } from "./piece-sample";
import { pieceSamples } from "./piece-samples";
import { pieceStepCount } from "./piece-step-count";

import type { Arc, ContourPiece, Cubic } from "../contour";

/** The quarter of the circle of radius 10 about the origin, from (10, 0) down to (0, 10). */
const QUARTER: Arc = {
  center: { x: 0, y: 0 },
  end: { x: 0, y: 10 },
  kind: "arc",
  positive: true,
  radius: 10,
  start: { x: 10, y: 0 },
};

/** A cubic from (0, 0) to (10, 4), its control points (3, 0) and (3, 4). */
const CUBIC: Cubic = {
  end: { x: 10, y: 4 },
  first: { x: 3, y: 0 },
  kind: "cubic",
  second: { x: 3, y: 4 },
  start: { x: 0, y: 0 },
};

describe("pieceSample (DERIV-band-within-box step 5)", () => {
  it("gives a segment's start and end", () => {
    const segment: ContourPiece = {
      end: { x: 10, y: 0 },
      kind: "segment",
      start: { x: 0, y: 0 },
    };

    expect(pieceSample(segment, 1, 0)).toEqual({ x: 0, y: 0 });
    expect(pieceSample(segment, 1, 1)).toEqual({ x: 10, y: 0 });
  });

  it("gives a quarter's ends exactly and its middle at π / 4 of 4 chords", () => {
    // 2 · (π / 2) / 4 = π / 4: (10 cos π/4, 10 sin π/4) = (7.07107, 7.07107).
    expect(pieceSample(QUARTER, 4, 0)).toEqual({ x: 10, y: 0 });
    expect(pieceSample(QUARTER, 4, 4)).toEqual({ x: 0, y: 10 });

    const middle = pieceSample(QUARTER, 4, 2);

    expect(middle.x).toBeCloseTo(7.071067812, 9);
    expect(middle.y).toBeCloseTo(7.071067812, 9);
  });

  it("turns a quarter drawn the other way from its own start", () => {
    // From (0, 10) back to (10, 0): the first of 4 at −π / 8 from (0, 10), (3.82683, 9.23880).
    const back: Arc = { ...QUARTER, end: QUARTER.start, positive: false, start: QUARTER.end };
    const first = pieceSample(back, 4, 1);

    expect(first.x).toBeCloseTo(3.826834324, 9);
    expect(first.y).toBeCloseTo(9.238795325, 9);
  });

  it("gives a cubic's point at t = 1 / 2 of 2 chords", () => {
    // B(½) = (P₀ + 3P₁ + 3P₂ + P₃) / 8 = ((0 + 9 + 9 + 10) / 8, (0 + 0 + 12 + 4) / 8) = (3.5, 2).
    expect(pieceSample(CUBIC, 2, 1)).toEqual({ x: 3.5, y: 2 });
    expect(pieceSample(CUBIC, 2, 2)).toEqual({ x: 10, y: 4 });
  });
});

describe("pieceSample (properties)", () => {
  /** A point on integers of a drawing. */
  const point = fc.record({
    x: fc.integer({ max: 100, min: -100 }),
    y: fc.integer({ max: 100, min: -100 }),
  });

  test.prop({
    points: fc.tuple(point, point, point, point),
    step: fc.integer({ max: 20, min: 1 }),
  })("gives a cubic's samples as pieceSamples, bit for bit", ({ points, step }) => {
    const [start, first, second, end] = points;
    const cubic: Cubic = { end, first, kind: "cubic", second, start };
    const count = pieceStepCount(cubic, step);
    const alone = Array.from({ length: count + 1 }, (unused, index) =>
      pieceSample(cubic, count, index),
    );

    expect(alone).toEqual(pieceSamples(cubic, step));
  });

  test.prop({
    angle: fc.double({ max: 3, min: -3, noNaN: true }).filter((turn) => Math.abs(turn) > 0.01),
    radius: fc.integer({ max: 500, min: 1 }),
    step: fc.integer({ max: 20, min: 1 }),
  })("gives an arc's samples as pieceSamples, bit for bit", ({ angle, radius, step }) => {
    const arc: Arc = {
      center: { x: 0, y: 0 },
      end: { x: radius * Math.cos(angle), y: radius * Math.sin(angle) },
      kind: "arc",
      positive: angle > 0,
      radius,
      start: { x: radius, y: 0 },
    };
    const count = pieceStepCount(arc, step);
    const alone = Array.from({ length: count + 1 }, (unused, index) =>
      pieceSample(arc, count, index),
    );

    expect(alone).toEqual(pieceSamples(arc, step));
  });
});
