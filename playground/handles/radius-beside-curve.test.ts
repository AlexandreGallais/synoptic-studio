// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  clickElement,
  clickPoint,
  openOnStep,
  readPointRows,
  readText,
  typeAndEnter,
} from "../user-actions";

describe("a corner beside a curved side keeps its radius (US-027)", () => {
  it("[F12.AC7] keeps points 2 and 4 rounded by 20 when point 3 gets handles", () => {
    // The rectangle 120 × 80, radius 20; point 3 (120, 80) in handles mode, its handle after
    // ending at (90, 110), the one before mirrored: the sides 2 → 3 and 3 → 4 are curves.
    openOnStep(1);
    typeAndEnter("radius", "20");
    clickElement("mode-points");
    clickPoint(3, false);
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "90");
    typeAndEnter("handle-after-y", "110");

    expect(readPointRows()).toEqual([
      "1 · x 0 · y 0 · radius 20",
      "2 · x 120 · y 0 · radius 20",
      "3 · x 120 · y 80 · radius 20 (effective 0)",
      "4 · x 0 · y 80 · radius 20",
    ]);

    expect(readText("#path-data").match(/A20 20 /gu)).toHaveLength(3);
  });

  it("[F12.AC7] reduces a radius the curve cannot hold, never to a sharp corner", () => {
    // Radius 60 at point 2 beside the same curve: drawn smaller, but drawn.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(3, false);
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "90");
    typeAndEnter("handle-after-y", "110");
    clickPoint(2, false);
    typeAndEnter("point-radius", "60");

    expect(readPointRows()[1]).toMatch(/^2 · x 120 · y 0 · radius 60 \(effective [1-5]\d\.\d+\)$/u);
  });
});
