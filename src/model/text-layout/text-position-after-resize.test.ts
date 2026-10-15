import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { DECORATED_METRICS, HELLO_TEXT } from "../../../tests";
import { halfSize, turnAbout } from "../../geometry";
import { add, turnDirection } from "../../math";
import { boxCenter } from "../placement";
import { textDrawing } from "../text-drawing";

import { textFrame } from "./text-frame";
import { textPositionAfterResize } from "./text-position-after-resize";

import type { Size } from "../../geometry";
import type { Point } from "../../math";
import type { Text } from "../text";

/** Precision of the criteria: within 1e-9. */
const DIGITS = 9;

/** Precision of the values written at 5 decimals. */
const WRITTEN_DIGITS = 5;

/** The criteria's text: at (47, 61), turned by 23, fitting its content. */
const TURNED: Text = { ...HELLO_TEXT, angle: 23, x: 47, y: 61 };

/** The criteria's text in a box 60 × 40 at (0, 0), turned by 90. */
const BOXED: Text = { ...HELLO_TEXT, angle: 90, boxHeight: 40, boxWidth: 60, isInBox: true };

/** The criteria's block before it grows. */
const BLOCK = { height: 20, width: 100 };

/** A size from 0 to 500, integers. */
const SIZE = fc.record({
  height: fc.integer({ max: 500, min: 0 }),
  width: fc.integer({ max: 500, min: 0 }),
});

/**
 * The turned top-left corner of a text as drawn: the top-left of its frame turned about the
 * middle of its frame (`DERIV-turned-shape` step 4).
 *
 * @param text - its position, angle and box
 * @param block - its measured block
 * @returns the corner, in the SVG
 */
function turnedCorner(text: Text, block: Size): Point {
  const frame = textFrame(text, block);

  return turnAbout({ x: frame.x, y: frame.y }, boxCenter(frame), turnDirection(text.angle));
}

/**
 * Expects two points to be equal within a number of decimals.
 *
 * @param actual - the point computed
 * @param expected - the point expected
 * @param digits - decimals compared
 */
function expectClose(actual: Point, expected: Point, digits: number): void {
  expect(actual.x).toBeCloseTo(expected.x, digits);
  expect(actual.y).toBeCloseTo(expected.y, digits);
}

describe("textPositionAfterResize (ADR-0057, DERIV-turned-shape step 5)", () => {
  it("[F05.AC5] keeps the turned corner of a block 100 × 20 at (47, 61), 23°, grown 120 wide", () => {
    // cos 23° = 0.92050485, sin 23° = 0.39073113; k = (50, 10), c = (97, 71).
    // P = c + R(−k) = (97 − 46.02524 + 3.90731, 71 − 19.53656 − 9.20505) = (54.88207, 42.25840).
    // k' = (60, 10): R(k') = (55.23029 − 3.90731, 23.44387 + 9.20505) = (51.32298, 32.64892);
    // b' = P − k' + R(k') = (46.20505, 64.90731).
    const grown = { block: { height: 20, width: 120 }, text: { ...TURNED, letterSpacing: 2 } };
    const position = textPositionAfterResize({ block: BLOCK, text: TURNED }, grown);
    const corner = turnedCorner(TURNED, BLOCK);

    expectClose(position, { x: 46.20505, y: 64.90731 }, WRITTEN_DIGITS);
    expectClose(corner, { x: 54.88207, y: 42.2584 }, WRITTEN_DIGITS);
    expectClose(turnedCorner({ ...grown.text, ...position }, grown.block), corner, DIGITS);

    // Without it, about the new middle (107, 71): (107 − 51.32298, 71 − 32.64892).
    expectClose(
      turnedCorner(grown.text, grown.block),
      { x: 55.67702, y: 38.35108 },
      WRITTEN_DIGITS,
    );
  });

  it("[F05.AC5] keeps the corner when a second line makes the block 120 × 40", () => {
    // k' = (60, 20): R(k') = (55.23029 − 7.81462, 23.44387 + 18.41010) = (47.41567, 41.85397);
    // b' = (54.88207 − 60 + 47.41567, 42.25840 − 20 + 41.85397) = (42.29774, 64.11236).
    const grown = {
      block: { height: 40, width: 120 },
      text: { ...TURNED, content: "Hello\nWorld" },
    };
    const position = textPositionAfterResize({ block: BLOCK, text: TURNED }, grown);

    expectClose(position, { x: 42.29774, y: 64.11236 }, WRITTEN_DIGITS);

    expectClose(
      turnedCorner({ ...grown.text, ...position }, grown.block),
      turnedCorner(TURNED, BLOCK),
      DIGITS,
    );
  });

  it("[F05.AC5] keeps the corner when the block shrinks to 80 × 20", () => {
    // k' = (40, 10): R(k') = (36.820194 − 3.907311, 15.629245 + 9.205049) = (32.912883, 24.834294);
    // b' = (54.882069 − 40 + 32.912883, 42.258395 − 10 + 24.834294) = (47.79495, 57.09269).
    const position = textPositionAfterResize(
      { block: BLOCK, text: TURNED },
      { block: { height: 20, width: 80 }, text: TURNED },
    );

    expectClose(position, { x: 47.79495, y: 57.09269 }, WRITTEN_DIGITS);
  });

  it("[F05.AC5] does not move a text in a box when its spacing changes its block", () => {
    const before = { block: { height: 18, width: 40 }, text: { ...BOXED, angle: 23 } };
    const after = { block: { height: 37, width: 52 }, text: { ...BOXED, angle: 23, x: 0.1 + 0.2 } };

    expect(textPositionAfterResize(before, after)).toEqual({ x: 0, y: 0 });
  });

  it("[F05.AC5] keeps the corner of a box 60 × 40 at 90° resized to 80 × 40", () => {
    // c = (30, 20); P = c + R(−30, −20) = (30 + 20, 20 − 30) = (50, −10);
    // k' = (40, 20), R(k') = (−20, 40): b' = (50 − 40 − 20, −10 − 20 + 40) = (−10, 10).
    const block = { height: 18, width: 40 };
    const resized = { block, text: { ...BOXED, boxWidth: 80 } };

    expect(textPositionAfterResize({ block, text: BOXED }, resized)).toEqual({ x: -10, y: 10 });
  });

  it("[F05.AC5] leaves the position of a text at 0° as it is, bit for bit", () => {
    const text = { ...TURNED, angle: 0, x: 0.1 + 0.2 };
    const position = textPositionAfterResize(
      { block: BLOCK, text },
      { block: { height: 40, width: 120 }, text },
    );

    expect(Object.is(position.x, 0.1 + 0.2)).toBe(true);
    expect(position.y).toBe(61);
  });

  it("[F05.AC5] leaves a turned position as it is, bit for bit, when the block keeps its size", () => {
    const text = { ...TURNED, x: 0.1 + 0.2 };
    const position = textPositionAfterResize({ block: BLOCK, text }, { block: BLOCK, text });

    expect(Object.is(position.x, 0.1 + 0.2)).toBe(true);
  });

  test.prop({
    after: SIZE,
    angle: fc.integer({ max: 359, min: 0 }),
    before: SIZE,
    isInBox: fc.boolean(),
    x: fc.integer({ max: 1000, min: -1000 }),
    y: fc.integer({ max: 1000, min: -1000 }),
  })("[F05.AC5] keeps the turned corner in place, whatever the size change", (values) => {
    const text = { ...HELLO_TEXT, angle: values.angle, isInBox: values.isInBox };
    const before = {
      block: values.before,
      text: { ...text, boxHeight: values.before.height, boxWidth: values.before.width },
    };
    const after = {
      block: values.after,
      text: { ...text, boxHeight: values.after.height, boxWidth: values.after.width },
    };
    const placed = { ...before, text: { ...before.text, x: values.x, y: values.y } };
    const position = textPositionAfterResize(placed, after);

    expectClose(
      turnedCorner({ ...after.text, ...position }, after.block),
      turnedCorner(placed.text, placed.block),
      DIGITS,
    );
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 1 }),
    letterSpacing: fc.integer({ max: 20, min: -2 }),
    lines: fc.integer({ max: 3, min: 1 }),
  })(
    "[F05.AC5] turns the measured text about its new block's middle, its corner kept",
    (values) => {
      const before = { ...HELLO_TEXT, angle: values.angle, x: 47, y: 61 };
      const after = {
        ...before,
        content: Array.from({ length: values.lines }, () => "Hello World").join("\n"),
        letterSpacing: values.letterSpacing,
      };
      const drawnBefore = textDrawing(DECORATED_METRICS, before);
      const drawnAfter = textDrawing(DECORATED_METRICS, after);
      const position = textPositionAfterResize(
        { block: drawnBefore.block, text: before },
        { block: drawnAfter.block, text: after },
      );
      const moved = textDrawing(DECORATED_METRICS, { ...after, ...position });
      const direction = turnDirection(values.angle);

      expectClose(moved.center, add(position, halfSize(drawnAfter.block)), DIGITS);

      expectClose(
        turnAbout(position, moved.center, direction),
        turnAbout({ x: 47, y: 61 }, drawnBefore.center, direction),
        DIGITS,
      );
    },
  );
});
