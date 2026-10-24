import { describe, expect, it } from "vitest";

import { readInsetBoxPathData } from "./read-inset-box-path-data";

describe("readInsetBoxPathData (US-086, AUD-046)", () => {
  it("[F44.AC3] lays the line of the box from (0, 0) to (250, 100) half a pixel inside it", () => {
    // Each side 0.5 inward: (0.5, 0.5) to (249.5, 99.5).
    expect(readInsetBoxPathData({ maxX: 250, maxY: 100, minX: 0, minY: 0 }, 0.5)).toBe(
      "M0.5 0.5H249.5V99.5H0.5Z",
    );
  });

  it("moves each side by at most half the box across it: a box 0.4 wide gives its middle", () => {
    // 0.4 / 2 = 0.2 < 0.5 across: both sides at x 0.2; 10 tall: 0.5 down.
    expect(readInsetBoxPathData({ maxX: 0.4, maxY: 10, minX: 0, minY: 0 }, 0.5)).toBe(
      "M0.2 0.5H0.2V9.5H0.2Z",
    );
  });

  it("gives the box's area for an inset of 0", () => {
    expect(readInsetBoxPathData({ maxX: 3, maxY: 2, minX: 1, minY: 1 }, 0)).toBe("M1 1H3V2H1Z");
  });
});
