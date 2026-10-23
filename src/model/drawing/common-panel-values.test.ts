import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { commonPanelValues } from "./common-panel-values";

/** A at (10, 20), 100 × 50. */
const SHAPE_A = { height: 50, width: 100, x: 10, y: 20 };

/** B at (200, 20), 40 × 40. */
const SHAPE_B = { height: 40, width: 40, x: 200, y: 20 };

describe("commonPanelValues (F38)", () => {
  it("[F38.AC1] gives A and B Y 20, X, W and H Mixed", () => {
    expect(commonPanelValues([SHAPE_A, SHAPE_B])).toEqual({
      height: undefined,
      width: undefined,
      x: undefined,
      y: 20,
    });
  });

  it("[F38.AC1] gives B and a text in a box 40 × 40 at (200, 20) every value", () => {
    expect(commonPanelValues([SHAPE_B, { ...SHAPE_B }])).toEqual(SHAPE_B);
  });

  it("[F38.AC1] gives A turned by 30° and B every value Mixed", () => {
    const turned = { height: 50, width: 100, x: 4.198729810778062, y: -1.650635094610969 };

    expect(commonPanelValues([turned, SHAPE_B])).toEqual({
      height: undefined,
      width: undefined,
      x: undefined,
      y: undefined,
    });
  });

  it("takes values within EPSILON as common, the first one's shown", () => {
    // 1.1 × 10⁻¹⁵ and 0 differ by less than 10⁻⁹; 0 and 2 · 10⁻⁹ by more.
    const off = { ...SHAPE_A, x: 1.1e-15 };

    expect(commonPanelValues([off, { ...SHAPE_A, x: 0 }]).x).toBe(1.1e-15);

    expect(
      commonPanelValues([
        { ...SHAPE_A, x: 0 },
        { ...SHAPE_A, x: 1e-9 },
      ]).x,
    ).toBe(0);

    expect(
      commonPanelValues([
        { ...SHAPE_A, x: 0 },
        { ...SHAPE_A, x: 2e-9 },
      ]).x,
    ).toBeUndefined();
  });

  it("compares every shape with the first, a third one differing making it Mixed", () => {
    expect(commonPanelValues([SHAPE_A, SHAPE_A, { ...SHAPE_A, height: 51 }])).toEqual({
      ...SHAPE_A,
      height: undefined,
    });
  });

  it("gives one shape its own values, and no shape none", () => {
    expect(commonPanelValues([SHAPE_A])).toEqual(SHAPE_A);

    expect(commonPanelValues([])).toEqual({
      height: undefined,
      width: undefined,
      x: undefined,
      y: undefined,
    });
  });

  test.prop({
    count: fc.integer({ max: 6, min: 2 }),
    index: fc.nat({ max: 5 }),
    value: fc.integer({ max: 1000, min: -1000 }),
  })("gives values alike, and Mixed once one of them differs by 1", ({ count, index, value }) => {
    const values = Array.from({ length: count }, () => ({
      height: value,
      width: value,
      x: value,
      y: value,
    }));
    const changed = values.with(index % count, {
      height: value,
      width: value,
      x: value + 1,
      y: value,
    });

    expect(commonPanelValues(values)).toEqual(values[0]);

    expect(commonPanelValues(changed)).toEqual({
      height: value,
      width: value,
      x: undefined,
      y: value,
    });
  });
});
