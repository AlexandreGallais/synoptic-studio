import { describe, expect, it } from "vitest";

import { panelValueAfterEdit } from "./panel-value-after-edit";

import type { PanelEdit } from "./panel-edit";
import type { PanelValues } from "./panel-values";

/** The largest width or height the playground accepts (Q20). */
const MAX_SIZE = 100_000;

/**
 * A value typed in a field of several shapes.
 *
 * @param key - the field
 * @param value - the integer typed, its sign included
 * @param isRelative - whether it was typed with a sign
 * @returns the edit, sizes capped at 100 000
 */
function edit(key: keyof PanelValues, value: number, isRelative: boolean): PanelEdit {
  return { key, maxSize: MAX_SIZE, typed: { isRelative, value } };
}

describe("panelValueAfterEdit (F38)", () => {
  it("[F38.AC2] adds +50 typed in W to each width: 100 gives 150, 40 gives 90", () => {
    expect(panelValueAfterEdit(100, edit("width", 50, true))).toBe(150);
    expect(panelValueAfterEdit(40, edit("width", 50, true))).toBe(90);
  });

  it("[F38.AC2] adds -20 typed in X to each x: 10 gives −10, 200 gives 180", () => {
    expect(panelValueAfterEdit(10, edit("x", -20, true))).toBe(-10);
    expect(panelValueAfterEdit(200, edit("x", -20, true))).toBe(180);
  });

  it("[F38.AC2] sets each width to 460 typed without a sign, each x to 50", () => {
    expect(panelValueAfterEdit(100, edit("width", 460, false))).toBe(460);
    expect(panelValueAfterEdit(40, edit("width", 460, false))).toBe(460);
    expect(panelValueAfterEdit(10, edit("x", 50, false))).toBe(50);
    expect(panelValueAfterEdit(4.19873, edit("x", 50, false))).toBe(50);
  });

  it("adds to a fraction as it is: a free shape 10.5 wide, +5, is 15.5 wide", () => {
    expect(panelValueAfterEdit(10.5, edit("width", 5, true))).toBe(15.5);
  });

  it("[F44.AC4] takes a width or a height below 0 at 0: 40 − 41, 0 − 1", () => {
    expect(panelValueAfterEdit(40, edit("width", -41, true))).toBe(0);
    expect(panelValueAfterEdit(0, edit("height", -1, true))).toBe(0);
  });

  it("[F44.AC4] rounds the value computed from a fraction typed, a tie up: 1.5, 10 + 1.5, 10 − 2.5", () => {
    // 1.5 → 2; 11.5 → 12; 7.5 → 8; −20.5 in X → −20, the tie toward +∞ (Q45).
    expect(panelValueAfterEdit(120, edit("width", 1.5, false))).toBe(2);
    expect(panelValueAfterEdit(10, edit("width", 1.5, true))).toBe(12);
    expect(panelValueAfterEdit(10, edit("height", -2.5, true))).toBe(8);
    expect(panelValueAfterEdit(0, edit("x", -20.5, false))).toBe(-20);
  });

  it("[F44.AC4] rounds before it caps: 99 999.6 typed in W gives 100 000, −0.4 gives 0", () => {
    expect(panelValueAfterEdit(40, edit("width", 99_999.6, false))).toBe(MAX_SIZE);
    expect(panelValueAfterEdit(40, edit("width", -0.4, false))).toBe(0);
  });

  it("keeps a width or a height of 0: 40 − 40, or 0 typed", () => {
    expect(panelValueAfterEdit(40, edit("width", -40, true))).toBe(0);
    expect(panelValueAfterEdit(50, edit("height", 0, false))).toBe(0);
  });

  it("caps a width or a height at 100 000, as one shape's field (Q20)", () => {
    expect(panelValueAfterEdit(40, edit("width", 200_000, false))).toBe(MAX_SIZE);
    // 99 990 + 50 = 100 040, over the largest.
    expect(panelValueAfterEdit(99_990, edit("height", 50, true))).toBe(MAX_SIZE);
    expect(panelValueAfterEdit(40, edit("width", MAX_SIZE, false))).toBe(MAX_SIZE);
  });

  it("caps no position: x and y take any value, negative or beyond 100 000", () => {
    expect(panelValueAfterEdit(10, edit("x", 200_000, false))).toBe(200_000);
    expect(panelValueAfterEdit(20, edit("y", -500, true))).toBe(-480);
  });
});
