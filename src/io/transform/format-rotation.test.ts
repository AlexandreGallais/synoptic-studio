import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { formatRotation } from "./format-rotation";

/** Pattern of a rotation about a point: `rotate(<angle> <cx> <cy>)` (`REF-SVG11-TRANSFORM`, §7.6). */
const ROTATION = /^rotate\((?<angle>\S+) (?<x>\S+) (?<y>\S+)\)$/u;

describe("formatRotation (REF-SVG11-TRANSFORM, §7.6)", () => {
  it("[F05.AC5] writes a rotation by 90 about the middle of a block 41.78125 × 36.796875", () => {
    // Middle (41.78125 / 2, 36.796875 / 2) = (20.890625, 18.3984375), written at 5 decimals.
    expect(formatRotation(90, { x: 20.890625, y: 18.3984375 })).toBe(
      "rotate(90 20.89063 18.39844)",
    );
  });

  it("writes a negative zero of the centre as 0", () => {
    expect(formatRotation(359, { x: -0, y: 0 })).toBe("rotate(359 0 0)");
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    x: fc.integer({ max: 100_000, min: -100_000 }),
    y: fc.integer({ max: 100_000, min: -100_000 }),
  })("reads back as the angle and the centre written, for integers", ({ angle, x, y }) => {
    const groups = ROTATION.exec(formatRotation(angle, { x, y }))?.groups;

    expect([groups?.["angle"], groups?.["x"], groups?.["y"]].map(Number)).toEqual([
      angle,
      x + 0,
      y + 0,
    ]);
  });
});
