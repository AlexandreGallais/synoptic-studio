import { describe, expect, it } from "vitest";

import { isFreeShape } from "./is-free-shape";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

import type { Shape } from "./shape-kind";

describe("isFreeShape (ADR-0036)", () => {
  it("[F10.AC4] tells a free shape from a round shape, which has no anchor either", () => {
    const shapes: readonly Shape[] = [
      rectangleAsFreeShape({ angle: 0, height: 50, radii: [0, 0, 0, 0], width: 100, x: 0, y: 0 }),
      { angle: 0, height: 80, kind: "round", width: 120, x: 0, y: 0 },
      {
        anchor: { horizontal: "mid", vertical: "mid" },
        angle: 0,
        corners: 6,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0],
        width: 100,
        x: 0,
        y: 0,
      },
    ];

    expect(shapes.map((shape) => isFreeShape(shape))).toEqual([true, false, false]);
  });
});
