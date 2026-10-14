import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bentRectangle } from "../../../tests";

import { drawnRadii } from "./drawn-radii";

describe("drawnRadii (ADR-0047)", () => {
  it("[F12.AC7] shows 0 at the point in handles mode and the radius asked beside it", () => {
    expect(drawnRadii(bentRectangle(1, [20, 20, 20, 20]))).toEqual([20, 20, 0, 20]);
  });

  // Twenty runs: a radius reduced beside a curve takes nested bisections.
  test.prop(
    { handle: fc.integer({ max: 80, min: 0 }), radius: fc.integer({ max: 100, min: 0 }) },
    { numRuns: 20 },
  )("never draws more than asked", ({ handle, radius }) => {
    const radii = drawnRadii(bentRectangle(handle, [radius, radius, radius, radius]));

    expect(radii.every((drawn) => drawn <= radius)).toBe(true);
  });
});
