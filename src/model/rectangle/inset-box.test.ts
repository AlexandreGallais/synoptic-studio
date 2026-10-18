import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { insetBox } from "./inset-box";

describe("insetBox (REF-WIKI-PARALLEL-CURVE)", () => {
  it.each([
    // 100 × 40 at (10, 20) inset by 3: (13, 23), 100 − 6 = 94 by 40 − 6 = 34.
    {
      box: { height: 40, width: 100, x: 10, y: 20 },
      expected: { height: 34, width: 94, x: 13, y: 23 },
      inset: 3,
    },
    // Inset by more than half its height: a height of 0, never negative.
    {
      box: { height: 4, width: 100, x: 0, y: 0 },
      expected: { height: 0, width: 94, x: 3, y: 3 },
      inset: 3,
    },
    // Inset by 0: the box itself.
    {
      box: { height: 4, width: 5, x: 1, y: 2 },
      expected: { height: 4, width: 5, x: 1, y: 2 },
      inset: 0,
    },
  ])("insets $box by $inset", ({ box, expected, inset }) => {
    expect(insetBox(box, inset)).toEqual(expected);
  });

  test.prop({
    height: fc.integer({ max: 500, min: 0 }),
    inset: fc.integer({ max: 100, min: 0 }),
    width: fc.integer({ max: 500, min: 0 }),
  })("keeps its middle while the box is wide and tall enough", ({ height, inset, width }) => {
    const inner = insetBox({ height, width, x: 0, y: 0 }, inset);

    // Both middles at (w / 2, h / 2) when w, h ≥ 2d.
    expect(inner.width >= 0 && inner.height >= 0).toBe(true);
    expect(inner.x + inner.width / 2).toBe(width >= 2 * inset ? width / 2 : inset);
    expect(inner.y + inner.height / 2).toBe(height >= 2 * inset ? height / 2 : inset);
  });
});
