import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { norm, turnDirection } from "../../math";

import { turnedHandles } from "./turned-handles";

/** Handles on, angle and length. */
const HANDLES = {
  after: { x: 30, y: -15 },
  before: { x: -30, y: 15 },
  isOn: true,
  mode: "angle-and-length",
} as const;

describe("turnedHandles (DERIV-cubic-sides step 5)", () => {
  it("[F12.AC3] turns each handle clockwise on screen, as its point", () => {
    // 90°: (x, y) ↦ (−y, x): (30, −15) ↦ (15, 30), (−30, 15) ↦ (−15, −30).
    expect(turnedHandles(HANDLES, turnDirection(90))).toEqual({
      ...HANDLES,
      after: { x: 15, y: 30 },
      before: { x: -15, y: -30 },
    });
  });

  test.prop({ after: integerPoint(1e3), angle: fc.integer({ max: 359, min: 0 }) })(
    "keeps each handle's length",
    ({ after, angle }) => {
      const turned = turnedHandles({ ...HANDLES, after }, turnDirection(angle));

      expect(norm(turned.after)).toBeCloseTo(norm(after), 6);
    },
  );

  it("turns a handle by half a turn", () => {
    const turned = turnedHandles(HANDLES, turnDirection(180));

    expect(turned.after.x).toBeCloseTo(-30, 12);
    expect(turned.after.y).toBeCloseTo(15, 12);
  });
});
