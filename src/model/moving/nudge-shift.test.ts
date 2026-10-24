import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { nudgeShift } from "./nudge-shift";

/** The four arrow keys, as `KeyboardEvent.key` names them. */
const ARROWS = ["ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp"];

describe("nudgeShift (US-073)", () => {
  it.each([
    // A at (10, 20): Right to (11, 20), Up to (10, 19), the y axis pointing down.
    { key: "ArrowRight", shift: { x: 1, y: 0 } },
    { key: "ArrowLeft", shift: { x: -1, y: 0 } },
    { key: "ArrowDown", shift: { x: 0, y: 1 } },
    { key: "ArrowUp", shift: { x: 0, y: -1 } },
  ])("[F39.AC4] nudges by 1 on $key", ({ key, shift }) => {
    expect(nudgeShift(key, false)).toEqual(shift);
  });

  it.each([
    // A at (10, 20): Shift + Right to (20, 20), Shift + Left to (0, 20).
    { key: "ArrowRight", shift: { x: 10, y: 0 } },
    { key: "ArrowLeft", shift: { x: -10, y: 0 } },
    { key: "ArrowDown", shift: { x: 0, y: 10 } },
    { key: "ArrowUp", shift: { x: 0, y: -10 } },
  ])("[F39.AC4] nudges by 10 on Shift + $key", ({ key, shift }) => {
    expect(nudgeShift(key, true)).toEqual(shift);
  });

  it.each([
    // The nudge itself, before it lands (`landedShift`, F46): 4.19873 + 1, − 1, − 10.
    { isFurther: false, key: "ArrowRight", x: 5.19873 },
    { isFurther: false, key: "ArrowLeft", x: 3.19873 },
    { isFurther: true, key: "ArrowLeft", x: -5.80127 },
  ])("[F39.AC4] nudges A′'s straight box from x 4.19873 to $x on $key", ({ isFurther, key, x }) => {
    const { x: across } = nudgeShift(key, isFurther) ?? { x: NaN };

    expect(4.19873 + across).toBeCloseTo(x, 9);
  });

  it.each(["a", "Enter", "Escape", "Delete", "Home", "PageUp", "Right", ""])(
    "gives no shift for %j, a key other than an arrow",
    (key) => {
      expect(nudgeShift(key, false)).toBeUndefined();
      expect(nudgeShift(key, true)).toBeUndefined();
    },
  );

  test.prop({ isFurther: fc.boolean(), key: fc.constantFrom(...ARROWS) })(
    "moves along one axis by the step, the other kept at 0, never −0",
    ({ isFurther, key }) => {
      const shift = nudgeShift(key, isFurther) ?? { x: NaN, y: NaN };
      const step = isFurther ? 10 : 1;

      expect(Math.abs(shift.x) + Math.abs(shift.y)).toBe(step);
      expect(Object.is(shift.x, 0) || Object.is(shift.y, 0)).toBe(true);
    },
  );
});
