import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { heldNudgeShift } from "./held-nudge-shift";
import { nudgeShift } from "./nudge-shift";

/** The four arrow keys, as `KeyboardEvent.key` names them. */
const ARROWS = ["ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp"];

describe("heldNudgeShift", () => {
  it.each([
    // A at (10, 20), Up then Right held: (+1, −1) at each key, (11, 19) after one.
    { keys: ["ArrowUp", "ArrowRight"], shift: { x: 1, y: -1 } },
    { keys: ["ArrowRight", "ArrowUp"], shift: { x: 1, y: -1 } },
    { keys: ["ArrowDown", "ArrowLeft"], shift: { x: -1, y: 1 } },
    // Left and Right cancel out, Up and Down too: 0, never −0.
    { keys: ["ArrowLeft", "ArrowRight"], shift: { x: 0, y: 0 } },
    { keys: ["ArrowUp", "ArrowDown", "ArrowRight"], shift: { x: 1, y: 0 } },
    // One arrow held, as `nudgeShift`.
    { keys: ["ArrowUp"], shift: { x: 0, y: -1 } },
    // None held, or a key other than an arrow: no shift.
    { keys: [], shift: { x: 0, y: 0 } },
    { keys: ["Shift", "a", "ArrowRight"], shift: { x: 1, y: 0 } },
  ])("[F43.AC3] nudges by $shift.x, $shift.y with $keys held", ({ keys, shift }) => {
    const { x, y } = heldNudgeShift(keys, false);

    expect(Object.is(x, shift.x) && Object.is(y, shift.y)).toBe(true);
  });

  it("[F43.AC3] nudges by 10 on both axes with Shift: (+10, −10) for Up and Right", () => {
    expect(heldNudgeShift(["ArrowUp", "ArrowRight"], true)).toEqual({ x: 10, y: -10 });
  });

  test.prop({
    isFurther: fc.boolean(),
    keys: fc.uniqueArray(fc.constantFrom(...ARROWS)),
  })("adds the nudge of each arrow held, in any order", ({ isFurther, keys }) => {
    const shift = heldNudgeShift(keys, isFurther);
    const reversed = heldNudgeShift(keys.toReversed(), isFurther);
    const shifts = keys.map((key) => nudgeShift(key, isFurther) ?? { x: NaN, y: NaN });

    expect(reversed).toEqual(shift);
    expect(shift.x).toBe(shifts.reduce((sum, each) => sum + each.x, 0));
    expect(shift.y).toBe(shifts.reduce((sum, each) => sum + each.y, 0));
  });
});
