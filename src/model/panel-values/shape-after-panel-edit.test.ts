import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "../rectangle";

import { shapeAfterPanelEdit } from "./shape-after-panel-edit";
import { shapePanelValues } from "./shape-panel-values";

import type { PanelEdit } from "./panel-edit";
import type { PanelValues } from "./panel-values";
import type { Rectangle } from "../rectangle";
import type { Shape } from "../shape";

/**
 * A sharp rectangle of the model, stored as a free shape (ADR-0032).
 *
 * @param box - its angle, position and size
 * @returns it, its four corners sharp
 */
function rectangle(box: Omit<Rectangle, "radii">): Shape {
  return rectangleAsFreeShape({ ...box, radii: [0, 0, 0, 0] });
}

/**
 * A value typed in a field of several shapes, sizes capped at 100 000 (Q20).
 *
 * @param key - the field
 * @param value - the integer typed, its sign included
 * @param isRelative - whether it was typed with a sign
 * @returns the edit, sizes capped at 100 000
 */
function edit(key: keyof PanelValues, value: number, isRelative: boolean): PanelEdit {
  return { key, maxSize: 100_000, typed: { isRelative, value } };
}

/** A at (10, 20), 100 × 50, sharp and straight. */
const SHAPE_A = rectangle({ angle: 0, height: 50, width: 100, x: 10, y: 20 });

/** B at (200, 20), 40 × 40, sharp and straight. */
const SHAPE_B = rectangle({ angle: 0, height: 40, width: 40, x: 200, y: 20 });

/** A turned by 30° about its middle (60, 45): its straight box starts at (4.19873, −1.65064). */
const TURNED_A = { ...SHAPE_A, angle: 30 };

/** A triangle of F02 in the box 90 × 60 at (10, 20), its anchor in the middle. */
const TRIANGLE: Shape = {
  anchor: { horizontal: "mid", vertical: "mid" },
  angle: 0,
  corners: 3,
  height: 60,
  radii: [0, 0, 0],
  stretched: false,
  width: 90,
  x: 10,
  y: 20,
};

describe("shapeAfterPanelEdit (F38)", () => {
  it("[F38.AC2] makes A 150 wide and B 90 for +50 in W, their positions kept", () => {
    expect(shapeAfterPanelEdit(SHAPE_A, edit("width", 50, true))).toEqual({
      ...SHAPE_A,
      width: 150,
    });

    expect(shapeAfterPanelEdit(SHAPE_B, edit("width", 50, true))).toEqual({
      ...SHAPE_B,
      width: 90,
    });
  });

  it("[F38.AC2] moves A to x −10 and B to x 180 for -20 in X, their y kept", () => {
    expect(shapeAfterPanelEdit(SHAPE_A, edit("x", -20, true))).toEqual({ ...SHAPE_A, x: -10 });
    expect(shapeAfterPanelEdit(SHAPE_B, edit("x", -20, true))).toEqual({ ...SHAPE_B, x: 180 });
  });

  it("[F38.AC2] makes A and B 460 wide for 460 in W, and puts both at x 50 for 50 in X", () => {
    expect(shapeAfterPanelEdit(SHAPE_A, edit("width", 460, false))).toEqual({
      ...SHAPE_A,
      width: 460,
    });

    expect(shapeAfterPanelEdit(SHAPE_B, edit("width", 460, false))).toEqual({
      ...SHAPE_B,
      width: 460,
    });

    expect(shapeAfterPanelEdit(SHAPE_A, edit("x", 50, false))).toEqual({ ...SHAPE_A, x: 50 });
    expect(shapeAfterPanelEdit(SHAPE_B, edit("x", 50, false))).toEqual({ ...SHAPE_B, x: 50 });
  });

  it("[F38.AC2] sets Y and H the same way: 0 in Y, +10 in H", () => {
    expect(shapeAfterPanelEdit(SHAPE_A, edit("y", 0, false))).toEqual({ ...SHAPE_A, y: 0 });

    expect(shapeAfterPanelEdit(SHAPE_B, edit("height", 10, true))).toEqual({
      ...SHAPE_B,
      height: 50,
    });
  });

  it("[F38.AC2] moves A turned by 30° so that its straight box starts at x 50", () => {
    // Its straight box starts at x 4.19873: the position moves by 50 − 4.19873 = 45.80127,
    // from 10 to 55.80127, as one shape's x typed (ADR-0035).
    const moved = shapeAfterPanelEdit(TURNED_A, edit("x", 50, false));

    expect(moved.x).toBeCloseTo(55.80127, 5);
    expect(moved.y).toBe(20);
    expect(shapePanelValues(moved).x).toBeCloseTo(50, 9);
  });

  it("[F38.AC2] moves A turned by 30° by -20 in Y, its straight box from −1.65064 to −21.65064", () => {
    const moved = shapeAfterPanelEdit(TURNED_A, edit("y", -20, true));

    expect(moved.x).toBe(10);
    expect(moved.y).toBeCloseTo(0, 9);
  });

  it("[F38.AC2] sets the width of A turned by 30°, its turned top-left corner kept (US-019)", () => {
    // Turned clockwise by 30° about (60, 45), the top-left corner (10, 20), (−50, −25) from the
    // middle, goes to (60 − 50 cos 30° + 25 sin 30°, 45 − 50 sin 30° − 25 cos 30°)
    // = (60 − 43.30127 + 12.5, 45 − 25 − 21.65064) = (29.19873, −1.65064).
    // 150 wide, the middle is (75, 25) from the top-left before the turn, which turns to
    // (75 cos 30° − 25 sin 30°, 75 sin 30° + 25 cos 30°) = (52.45191, 59.15064): the middle is at
    // (29.19873 + 52.45191, −1.65064 + 59.15064) = (81.65064, 57.5), the position (6.65064, 32.5).
    const resized = shapeAfterPanelEdit(TURNED_A, edit("width", 50, true));

    expect(resized.width).toBe(150);
    expect(resized.height).toBe(50);
    expect(resized.x).toBeCloseTo(6.65064, 5);
    expect(resized.y).toBeCloseTo(32.5, 5);
  });

  it("[F44.AC4] makes B 0 wide for -50 in W, 40 wide: −10 taken at 0", () => {
    expect(shapeAfterPanelEdit(SHAPE_B, edit("width", -50, true))).toEqual({
      ...SHAPE_B,
      width: 0,
    });
  });

  it("caps a width at 100 000, as one shape's field (Q20)", () => {
    expect(shapeAfterPanelEdit(SHAPE_B, edit("width", 250_000, false)).width).toBe(100_000);
  });

  it("moves a triangle by its contour's straight box, not its box", () => {
    // The triangle F02 places in the middle of its box does not reach every side of it: its
    // straight box's x is that of its contour, which X 50 then gives.
    const before = shapePanelValues(TRIANGLE).x;
    const moved = shapeAfterPanelEdit(TRIANGLE, edit("x", 50, false));

    expect(moved.x).toBeCloseTo(10 + 50 - before, 9);
    expect(shapePanelValues(moved).x).toBeCloseTo(50, 9);
  });

  it("moves a shape 0 × 0 at (10, 20) by its point: 50 in X puts it at x 50", () => {
    const point = rectangle({ angle: 30, height: 0, width: 0, x: 10, y: 20 });

    expect(shapeAfterPanelEdit(point, edit("x", 50, false))).toEqual({ ...point, x: 50 });
  });

  it("sets a round shape's height, 460 in H", () => {
    const round: Shape = { angle: 0, height: 80, kind: "round", width: 40, x: 0, y: 0 };

    expect(shapeAfterPanelEdit(round, edit("height", 460, false))).toEqual({
      ...round,
      height: 460,
    });
  });
});
