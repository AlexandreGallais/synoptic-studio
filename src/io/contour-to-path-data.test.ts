import { describe, expect, it } from "vitest";

import { contourToPathData } from "./contour-to-path-data";

describe("contourToPathData", () => {
  it("moves to the first vertex, draws lines to the others, then closes", () => {
    // Rectangle 100 × 50 of US-001: three explicit edges, the fourth drawn by Z (SVG 2 §9.3.4).
    const contour = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ];

    expect(contourToPathData(contour)).toBe("M0 0 L100 0 L100 50 L0 50 Z");
  });

  it("writes derived coordinates with fixed precision (EN-001)", () => {
    // Equilateral triangle of side 100: apex height 100 × √3 / 2 = 86.602540378… → 86.60254.
    const contour = [
      { x: 50, y: 0 },
      { x: 100, y: (100 * Math.sqrt(3)) / 2 },
      { x: 0, y: (100 * Math.sqrt(3)) / 2 },
    ];

    expect(contourToPathData(contour)).toBe("M50 0 L100 86.60254 L0 86.60254 Z");
  });

  it("closes a single-vertex contour on itself", () => {
    expect(contourToPathData([{ x: 5, y: 5 }])).toBe("M5 5 Z");
  });

  it("writes nothing for an empty contour", () => {
    // An empty `d` draws nothing, which is valid SVG.
    expect(contourToPathData([])).toBe("");
  });
});
