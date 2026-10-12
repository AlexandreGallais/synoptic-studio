import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "./rectangle-as-free-shape";
import { shapeKind } from "./shape-kind";

describe("shapeKind (ADR-0036)", () => {
  it("tells a free shape, a regular polygon and a round shape apart", () => {
    const free = rectangleAsFreeShape({
      angle: 0,
      height: 50,
      radii: [0, 0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    });
    const polygon = {
      anchor: { horizontal: "mid", vertical: "mid" },
      angle: 0,
      corners: 6,
      height: 100,
      radii: [0, 0, 0, 0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    } as const;

    expect(shapeKind(free)).toBe("free");
    expect(shapeKind(polygon)).toBe("polygon");

    expect(shapeKind({ angle: 0, height: 80, kind: "round", width: 120, x: 0, y: 0 })).toBe(
      "round",
    );
  });
});
