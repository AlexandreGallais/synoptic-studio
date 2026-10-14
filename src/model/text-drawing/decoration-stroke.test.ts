import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { decorationStroke } from "./decoration-stroke";

/** 'Hello' in Arial 16, aligned left, its baseline at 14.484375. */
const HELLO_LINE = { content: "Hello", left: 0, width: 36.4609375, x: 0, y: 14.484375 };

describe("decorationStroke (REF-OT-POST, REF-OT-OS2, DERIV-text-width step 6)", () => {
  it("[F05.AC4] puts an underline whose top is 2 below the baseline at y = 16.484375", () => {
    // 14.484375 − (−2) = 16.484375; as long as the line, 1 thick.
    expect(decorationStroke(HELLO_LINE, { position: -2, thickness: 1 })).toEqual({
      height: 1,
      width: 36.4609375,
      x: 0,
      y: 16.484375,
    });
  });

  it("[F05.AC4] puts a strike-through whose top is 4 above the baseline at y = 10.484375", () => {
    expect(decorationStroke(HELLO_LINE, { position: 4, thickness: 1 }).y).toBe(10.484375);
  });

  const number = fc.double({ max: 100_000, min: -100_000, noNaN: true });

  test.prop({ position: number, thickness: number, y: number })(
    "spans the line, its top the position above the baseline",
    ({ position, thickness, y }) => {
      const stroke = decorationStroke({ ...HELLO_LINE, y }, { position, thickness });

      expect([stroke.x, stroke.width, stroke.height]).toEqual([0, 36.4609375, thickness]);
      expect(y - stroke.y).toBeCloseTo(position, 6);
    },
  );
});
