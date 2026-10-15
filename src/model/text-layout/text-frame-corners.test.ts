import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textFrameCorners } from "./text-frame-corners";

import type { Point } from "../../math";

/** A point computed by hand, its x then its y. */
type Expected = readonly [number, number];

/**
 * Asserts each point to 5 decimals, the precision the SVG is written with (Q10).
 *
 * @param actual - points computed
 * @param expected - points computed by hand
 */
function expectPoints(actual: readonly Point[], expected: readonly Expected[]): void {
  expect(actual).toHaveLength(expected.length);

  for (const [index, [x, y]] of expected.entries()) {
    expect(actual[index]?.x).toBeCloseTo(x, 5);
    expect(actual[index]?.y).toBeCloseTo(y, 5);
  }
}

describe("textFrameCorners (DERIV-turned-shape step 4)", () => {
  it("[F14.AC7] turns a block 40 × 20 at (0, 0) by 45 about its middle (20, 10)", () => {
    // R(45°) on screen: (dx, dy) ↦ (dx·c − dy·s, dx·s + dy·c), c = s = √2/2 ≈ 0.70711.
    // (−20, −10) ↦ (−7.07107, −21.21320); (20, −10) ↦ (21.21320, 7.07107);
    // (20, 10) ↦ (7.07107, 21.21320); (−20, 10) ↦ (−21.21320, −7.07107); plus (20, 10).
    const text = { ...HELLO_TEXT, angle: 45, x: 0, y: 0 };

    expectPoints(textFrameCorners(text, { height: 20, width: 40 }), [
      [12.92893, -11.2132],
      [41.2132, 17.07107],
      [27.07107, 31.2132],
      [-1.2132, 2.92893],
    ]);
  });

  it("[F14.AC7] turns a block 100 × 20 at (47, 61) by 23 about its middle (97, 71)", () => {
    // c = cos 23° ≈ 0.920505, s = sin 23° ≈ 0.390731; half sizes (50, 10).
    // (−50, −10) ↦ (−50c + 10s, −50s − 10c) = (−42.11793, −28.74160);
    // (50, −10) ↦ (50c + 10s, 50s − 10c) = (49.93255, 10.33151);
    // (50, 10) ↦ (50c − 10s, 50s + 10c) = (42.11793, 28.74160);
    // (−50, 10) ↦ (−50c − 10s, −50s + 10c) = (−49.93255, −10.33151); plus (97, 71).
    const text = { ...HELLO_TEXT, angle: 23, x: 47, y: 61 };

    expectPoints(textFrameCorners(text, { height: 20, width: 100 }), [
      [54.88207, 42.2584],
      [146.93255, 81.33151],
      [139.11793, 99.7416],
      [47.06745, 60.66849],
    ]);
  });

  it("[F14.AC7] turns a box 60 × 40 at (0, 0) by 90 about (30, 20), whatever the block", () => {
    // R(90°): (dx, dy) ↦ (−dy, dx); (−30, −20) ↦ (20, −30), plus (30, 20) = (50, −10).
    const text = {
      ...HELLO_TEXT,
      angle: 90,
      boxHeight: 40,
      boxWidth: 60,
      isInBox: true,
      x: 0,
      y: 0,
    };

    expect(textFrameCorners(text, { height: 7, width: 9 })).toEqual([
      { x: 50, y: -10 },
      { x: 50, y: 50 },
      { x: 10, y: 50 },
      { x: 10, y: -10 },
    ]);
  });

  it("[F14.AC7] gives the block's corners at 0°, exactly", () => {
    const text = { ...HELLO_TEXT, angle: 0, x: 10, y: 20 };

    expect(textFrameCorners(text, { height: 36.796875, width: 41.78125 })).toEqual([
      { x: 10, y: 20 },
      { x: 51.78125, y: 20 },
      { x: 51.78125, y: 56.796875 },
      { x: 10, y: 56.796875 },
    ]);
  });

  it("gives four corners on one point for an empty block", () => {
    const text = { ...HELLO_TEXT, angle: 30, x: 5, y: 7 };

    expectPoints(textFrameCorners(text, { height: 0, width: 0 }), [
      [5, 7],
      [5, 7],
      [5, 7],
      [5, 7],
    ]);
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    height: fc.integer({ max: 1000, min: 0 }),
    width: fc.integer({ max: 1000, min: 0 }),
  })(
    "keeps the frame's middle and its sides' lengths when it turns",
    ({ angle, height, width }) => {
      const text = { ...HELLO_TEXT, angle, x: 3, y: -4 };
      const [a, b, c] = textFrameCorners(text, { height, width });
      const first = a ?? { x: NaN, y: NaN };
      const second = b ?? first;
      const third = c ?? first;

      // A rotation about the middle keeps it the middle of the diagonal, and every distance.
      expect((first.x + third.x) / 2).toBeCloseTo(3 + width / 2, 6);
      expect((first.y + third.y) / 2).toBeCloseTo(-4 + height / 2, 6);
      expect(Math.hypot(second.x - first.x, second.y - first.y)).toBeCloseTo(width, 6);
      expect(Math.hypot(third.x - second.x, third.y - second.y)).toBeCloseTo(height, 6);
    },
  );
});
