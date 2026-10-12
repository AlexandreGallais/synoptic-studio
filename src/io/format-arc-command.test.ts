import { describe, expect, it } from "vitest";

import { formatArcCommand } from "./format-arc-command";

import type { Arc } from "../geometry";

/**
 * Fillet arc reduced to what the command writes: radius, end point and direction.
 *
 * @param fields - radius, end point and whether the arc turns in the positive direction
 * @returns an arc whose start and center are irrelevant to the command
 */
function arc(fields: Pick<Arc, "end" | "positive" | "radius">): Arc {
  return { ...fields, center: { x: 0, y: 0 }, kind: "arc", start: { x: 0, y: 0 } };
}

describe("formatArcCommand (REF-SVG2-PATHS §9.3.8, DERIV-fillet-arc steps 4–5)", () => {
  it("writes a small arc in the positive direction with sweep flag 1", () => {
    // rx = ry = 10, x-axis-rotation 0, large-arc 0, sweep 1, end (100, 10).
    expect(formatArcCommand(arc({ end: { x: 100, y: 10 }, positive: true, radius: 10 }))).toBe(
      "A10 10 0 0 1 100 10",
    );
  });

  it("writes an arc in the negative direction with sweep flag 0", () => {
    expect(formatArcCommand(arc({ end: { x: 10, y: -5 }, positive: false, radius: 5 }))).toBe(
      "A5 5 0 0 0 10 -5",
    );
  });

  it("writes a derived radius at fixed precision", () => {
    // 12.500000000000004 (the clamp of a 100 × 25 rectangle) → 12.5.
    expect(
      formatArcCommand(arc({ end: { x: 12.5, y: 0 }, positive: true, radius: 12.500000000000004 })),
    ).toBe("A12.5 12.5 0 0 1 12.5 0");
  });
});
