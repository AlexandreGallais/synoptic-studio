import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isPointInBox } from "./is-point-in-box";
import { widenedBox } from "./widened-box";

/** A box on integers, its minimum and its maximum drawn on each axis. */
const BOX = fc
  .tuple(
    fc.integer({ max: 50, min: -50 }),
    fc.integer({ max: 50, min: -50 }),
    fc.integer({ max: 50, min: 0 }),
    fc.integer({ max: 50, min: 0 }),
  )
  .map(([minX, minY, width, height]) => ({
    maxX: minX + width,
    maxY: minY + height,
    minX,
    minY,
  }));

describe("widenedBox (DERIV-box-overlap step 10)", () => {
  it("moves each side outward by the margin", () => {
    // [0, 100] × [−30, 0] widened by 2: [0 − 2, 100 + 2] × [−30 − 2, 0 + 2].
    expect(widenedBox({ maxX: 100, maxY: 0, minX: 0, minY: -30 }, 2)).toEqual({
      maxX: 102,
      maxY: 2,
      minX: -2,
      minY: -32,
    });
  });

  it("moves each side inward by a negative margin, and keeps the box by 0", () => {
    // [0, 10]² by −3: [3, 7]²; by 0 unchanged.
    const box = { maxX: 10, maxY: 10, minX: 0, minY: 0 };

    expect(widenedBox(box, -3)).toEqual({ maxX: 7, maxY: 7, minX: 3, minY: 3 });
    expect(widenedBox(box, 0)).toEqual(box);
  });
});

describe("widenedBox (properties)", () => {
  test.prop({ box: BOX, margin: fc.integer({ max: 20, min: 0 }) })(
    "holds the box's corners, and its own sides lie the margin away",
    ({ box, margin }) => {
      const widened = widenedBox(box, margin);

      expect(isPointInBox(widened, { x: box.minX, y: box.minY })).toBe(true);
      expect(isPointInBox(widened, { x: box.maxX, y: box.maxY })).toBe(true);
      expect(widened.maxX - widened.minX).toBe(box.maxX - box.minX + 2 * margin);
      expect(widened.maxY - widened.minY).toBe(box.maxY - box.minY + 2 * margin);
    },
  );
});
