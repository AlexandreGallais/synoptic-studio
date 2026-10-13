import { describe, expect, it } from "vitest";

import { expectPoints } from "../../../tests";

import { pastedControlPoints } from "./pasted-control-points";

import type { PastedCurve } from "./pasted-curve";
import type { Point } from "../../math";

/** The side from (100, 0) to (200, 0). */
const START = { x: 100, y: 0 };

/** Its end. */
const END = { x: 200, y: 0 };

/**
 * The control points of a curve, or two NaN points when it was refused.
 *
 * @param curve - what the paste gave
 * @returns its two control points
 */
function controlsOf(curve: PastedCurve): readonly [Point, Point] {
  return "first" in curve
    ? [curve.first, curve.second]
    : [
        { x: NaN, y: NaN },
        { x: NaN, y: NaN },
      ];
}

describe("pastedControlPoints (DERIV-handle-share step 4)", () => {
  it("[F12.AC4] takes one SVG C command ending on the side's end, decimals kept", () => {
    expect(pastedControlPoints("C 130 -15 170 -15 200 0", START, END)).toEqual({
      first: { x: 130, y: -15 },
      second: { x: 170, y: -15 },
    });

    expect(pastedControlPoints("C130.5,-15.25 170 -15 200 0", START, END)).toEqual({
      first: { x: 130.5, y: -15.25 },
      second: { x: 170, y: -15 },
    });
  });

  it("[F12.AC4] lays CSS's cubic-bezier() on the side by its diagonal", () => {
    // ease-in-out: (0.42, 0) → 0.21 along, −0.21 across: (121, 21); (0.58, 1) → (179, −21).
    expectPoints(
      controlsOf(pastedControlPoints("cubic-bezier(0.42, 0, 0.58, 1)", START, END)),
      [
        { x: 121, y: 21 },
        { x: 179, y: -21 },
      ],
      9,
    );
  });

  it("refuses a C that does not end on the side's end, an x outside 0 to 1, and any other text", () => {
    expect(pastedControlPoints("C 130 -15 170 -15 190 0", START, END)).toEqual({
      refusal: "The C must end on the side's last point.",
    });

    expect(pastedControlPoints("cubic-bezier(1.2, 0, 0.5, 1)", START, END)).toEqual({
      refusal: "A cubic-bezier() takes x values from 0 to 1.",
    });

    expect(pastedControlPoints("cubic-bezier(0.5, 0, -0.1, 1)", START, END)).toEqual({
      refusal: "A cubic-bezier() takes x values from 0 to 1.",
    });

    expect(pastedControlPoints("M 0 0 L 10 10", START, END)).toEqual({
      refusal: "Paste one C command (C x1 y1 x2 y2 x y) or a cubic-bezier() value.",
    });
  });
});
