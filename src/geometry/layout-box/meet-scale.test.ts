import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { meetScale } from "./meet-scale";

describe("meetScale (REF-SVG2-COORDS, §8.2 step 7)", () => {
  it("is 2.8 for A, B and C, 400 × 200, in 1 120 × 720", () => {
    // min(1 120 / 400, 720 / 200) = min(2.8, 3.6) = 2.8 (F42.AC3).
    expect(meetScale({ height: 720, width: 1120 }, { height: 200, width: 400 })).toBe(2.8);
  });

  it("is 0.0072 for R, 99 999 × 100 000, in 1 120 × 720, its height the bound", () => {
    // min(1 120 / 99 999, 720 / 100 000) = min(0.0112001, 0.0072) = 0.0072.
    expect(meetScale({ height: 720, width: 1120 }, { height: 100_000, width: 99_999 })).toBe(
      0.0072,
    );
  });

  it("skips a span of 0: a horizontal segment 400 long in 1 120 × 720 is 2.8", () => {
    expect(meetScale({ height: 720, width: 1120 }, { height: 0, width: 400 })).toBe(2.8);
    // 720 / 360 = 2.
    expect(meetScale({ height: 720, width: 1120 }, { height: 360, width: 0 })).toBe(2);
  });

  it("bounds no scale for a span of 0 × 0", () => {
    expect(meetScale({ height: 720, width: 1120 }, { height: 0, width: 0 })).toBe(Infinity);
  });

  it("is 0 for a room of 0", () => {
    expect(meetScale({ height: 0, width: 1120 }, { height: 50, width: 100 })).toBe(0);
  });

  test.prop({
    room: fc.record({ height: fc.nat(1e5), width: fc.nat(1e5) }),
    span: fc.record({
      height: fc.integer({ max: 1e5, min: 1 }),
      width: fc.integer({ max: 1e5, min: 1 }),
    }),
  })("fits the span within the room, reaching one of its sides", ({ room, span }) => {
    const scale = meetScale(room, span);
    const reaches = [scale * span.width - room.width, scale * span.height - room.height];

    // Within the room on both axes, on its side on one, within a rounding error.
    expect(Math.max(...reaches)).toBeCloseTo(0, 6);

    expect(reaches.every((excess) => excess <= 1e-9 * Math.max(room.width, room.height, 1))).toBe(
      true,
    );
  });
});
