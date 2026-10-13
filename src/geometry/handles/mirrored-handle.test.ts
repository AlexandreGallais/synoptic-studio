import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { dot, norm, perpDot } from "../../math";

import { mirroredHandle } from "./mirrored-handle";

/** Handle with integer components. */
const HANDLE = integerPoint(1e3);

describe("mirroredHandle (REF-FIGMA-HANDLE-MIRRORING)", () => {
  it("[F12.AC2] reverses the typed handle in mode angle and length", () => {
    // −(30, −15) = (−30, 15): the two handles on one line, equidistant (EN-024).
    expect(mirroredHandle("angle-and-length", { x: 30, y: -15 }, { x: -1, y: 0 })).toEqual({
      x: -30,
      y: 15,
    });
  });

  it("[F12.AC2] keeps the other handle's length in mode angle", () => {
    // |t| = 50, |o| = 10: −(30, −40) · 10 / 50 = (−6, 8).
    expect(mirroredHandle("angle", { x: 30, y: -40 }, { x: -10, y: 0 })).toEqual({ x: -6, y: 8 });
  });

  it("[F12.AC2] leaves the other handle in mode none", () => {
    expect(mirroredHandle("none", { x: 30, y: -15 }, { x: -5, y: 7 })).toEqual({ x: -5, y: 7 });
  });

  it("keeps the other handle when the typed one has no direction, in mode angle", () => {
    expect(mirroredHandle("angle", { x: 0, y: 0 }, { x: -5, y: 7 })).toEqual({ x: -5, y: 7 });
  });

  it("writes zeros, never −0, for a zero handle", () => {
    // A zero other handle in mode angle, and a zero typed one in angle and length: (0, 0).
    const zeros = [
      mirroredHandle("angle", { x: 30, y: -15 }, { x: 0, y: 0 }),
      mirroredHandle("angle-and-length", { x: 0, y: 0 }, { x: 4, y: 4 }),
    ];

    expect(zeros.flatMap(({ x, y }) => [Object.is(x, 0), Object.is(y, 0)])).toEqual([
      true,
      true,
      true,
      true,
    ]);
  });

  test.prop({ other: HANDLE, typed: HANDLE })(
    "puts the other handle on the typed handle's line, opposite, of its own length in mode angle",
    ({ other, typed }) => {
      fc.pre(norm(typed) > 0);

      const mirrored = mirroredHandle("angle", typed, other);

      expect(norm(mirrored)).toBeCloseTo(norm(other), 6);
      expect(perpDot(mirrored, typed)).toBeCloseTo(0, 3);
      expect(dot(mirrored, typed)).toBeLessThanOrEqual(0);
    },
  );

  it("[F12.AC4] mirrors as mode angle in mode auto, keeping the other handle when the typed one has no length", () => {
    expect(mirroredHandle("auto", { x: 30, y: -40 }, { x: -10, y: 0 })).toEqual({ x: -6, y: 8 });
    expect(mirroredHandle("auto", { x: 0, y: 0 }, { x: -5, y: 7 })).toEqual({ x: -5, y: 7 });
  });
});
