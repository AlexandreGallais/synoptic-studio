import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { shapeMovedOnAxis } from "./shape-moved-on-axis";

import type { Shape } from "../shape";

/** A circle in the box 40 × 40 at (200, 20), turned by 30°. */
const CIRCLE: Shape = { angle: 30, height: 40, kind: "round", width: 40, x: 200, y: 20 };

describe("shapeMovedOnAxis (F38)", () => {
  it("moves a shape across by −20 and down by 45.5, the rest kept", () => {
    expect(shapeMovedOnAxis(CIRCLE, "x", -20)).toEqual({ ...CIRCLE, x: 180 });
    expect(shapeMovedOnAxis(CIRCLE, "y", 45.5)).toEqual({ ...CIRCLE, y: 65.5 });
  });

  it("moves a text the same way, its content and options kept", () => {
    expect(shapeMovedOnAxis({ ...HELLO_TEXT, x: 300 }, "x", 50)).toEqual({ ...HELLO_TEXT, x: 350 });
  });

  it("keeps a shape moved by 0 as it is", () => {
    expect(shapeMovedOnAxis(CIRCLE, "y", 0)).toEqual(CIRCLE);
  });
});
