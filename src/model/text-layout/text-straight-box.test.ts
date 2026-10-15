import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textStraightBox } from "./text-straight-box";

import type { BoundingBox } from "../../geometry";

/**
 * Asserts each side of a box to 5 decimals, the precision the SVG is written with (Q10).
 *
 * @param actual - box computed
 * @param expected - box computed by hand
 */
function expectBox(actual: BoundingBox, expected: BoundingBox): void {
  expect(actual.minX).toBeCloseTo(expected.minX, 5);
  expect(actual.minY).toBeCloseTo(expected.minY, 5);
  expect(actual.maxX).toBeCloseTo(expected.maxX, 5);
  expect(actual.maxY).toBeCloseTo(expected.maxY, 5);
}

describe("textStraightBox (DERIV-turned-shape step 4)", () => {
  it("[F14.AC7] boxes a block 40 × 20 at (0, 0) turned by 45: at (−1.21320, −11.21320), 42.42641 square", () => {
    // Corners (12.92893, −11.21320), (41.21320, 17.07107), (27.07107, 31.21320),
    // (−1.21320, 2.92893); half sizes 20c + 10s = 20s + 10c = 30·√2/2 = 21.21320 about (20, 10).
    const text = { ...HELLO_TEXT, angle: 45, x: 0, y: 0 };

    expectBox(textStraightBox(text, { height: 20, width: 40 }), {
      maxX: 41.2132,
      maxY: 31.2132,
      minX: -1.2132,
      minY: -11.2132,
    });
  });

  it("[F14.AC7] boxes a block 100 × 20 at (47, 61) turned by 23: 99.86511 × 57.48321", () => {
    // Half sizes about (97, 71): 50c + 10s = 46.02524 + 3.90731 = 49.93255,
    // 50s + 10c = 19.53656 + 9.20505 = 28.74160; c = cos 23°, s = sin 23°.
    const text = { ...HELLO_TEXT, angle: 23, x: 47, y: 61 };

    expectBox(textStraightBox(text, { height: 20, width: 100 }), {
      maxX: 146.93255,
      maxY: 99.7416,
      minX: 47.06745,
      minY: 42.2584,
    });
  });

  it("[F14.AC7] boxes a box 60 × 40 at (0, 0) turned by 90: at (10, −10), 40 × 60, exactly", () => {
    // Corners (50, −10), (50, 50), (10, 50), (10, −10).
    const text = {
      ...HELLO_TEXT,
      angle: 90,
      boxHeight: 40,
      boxWidth: 60,
      isInBox: true,
      x: 0,
      y: 0,
    };

    expect(textStraightBox(text, { height: 7, width: 9 })).toEqual({
      maxX: 50,
      maxY: 50,
      minX: 10,
      minY: -10,
    });
  });

  it("[F14.AC7] is the block itself at 0°, exactly", () => {
    const text = { ...HELLO_TEXT, angle: 0, x: 10, y: 20 };

    expect(textStraightBox(text, { height: 36.796875, width: 41.78125 })).toEqual({
      maxX: 51.78125,
      maxY: 56.796875,
      minX: 10,
      minY: 20,
    });
  });

  it("[F14.AC7] has no area for a box 0 wide", () => {
    const text = {
      ...HELLO_TEXT,
      angle: 0,
      boxHeight: 40,
      boxWidth: 0,
      isInBox: true,
      x: 0,
      y: 0,
    };

    expect(textStraightBox(text, { height: 7, width: 9 })).toEqual({
      maxX: 0,
      maxY: 40,
      minX: 0,
      minY: 0,
    });
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    height: fc.integer({ max: 1000, min: 0 }),
    width: fc.integer({ max: 1000, min: 0 }),
  })(
    "is w|cos α| + h|sin α| wide and w|sin α| + h|cos α| tall, about the frame's middle",
    ({ angle, height, width }) => {
      // The extents of a rectangle turned by α about its middle, its corners at ± half sizes.
      const radians = (angle * Math.PI) / 180;
      const cos = Math.abs(Math.cos(radians));
      const sin = Math.abs(Math.sin(radians));
      const box = textStraightBox({ ...HELLO_TEXT, angle, x: 3, y: -4 }, { height, width });

      expect(box.maxX - box.minX).toBeCloseTo(width * cos + height * sin, 6);
      expect(box.maxY - box.minY).toBeCloseTo(width * sin + height * cos, 6);
      expect((box.minX + box.maxX) / 2).toBeCloseTo(3 + width / 2, 6);
      expect((box.minY + box.maxY) / 2).toBeCloseTo(-4 + height / 2, 6);
    },
  );
});
