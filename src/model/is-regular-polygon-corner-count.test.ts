import { describe, expect, it } from "vitest";

import { isRegularPolygonCornerCount } from "./is-regular-polygon-corner-count";

describe("isRegularPolygonCornerCount", () => {
  it("[F02.AC1] accepts 3 to 12 corners (Q19)", () => {
    expect(isRegularPolygonCornerCount(3)).toBe(true);
    expect(isRegularPolygonCornerCount(12)).toBe(true);
  });

  it("[F02.AC1] refuses 2, 13, a fraction and an unsafe integer", () => {
    expect(isRegularPolygonCornerCount(2)).toBe(false);
    expect(isRegularPolygonCornerCount(13)).toBe(false);
    expect(isRegularPolygonCornerCount(4.5)).toBe(false);
    expect(isRegularPolygonCornerCount(Infinity)).toBe(false);
  });
});
