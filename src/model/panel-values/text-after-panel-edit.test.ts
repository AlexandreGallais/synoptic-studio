import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textAfterPanelEdit } from "./text-after-panel-edit";

import type { PanelEdit } from "./panel-edit";
import type { PanelValues } from "./panel-values";

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

/** A block measured 80 × 30. */
const BLOCK = { height: 30, width: 80 };

/** A text fitting its content, its block 80 × 30 at (300, 20). */
const FITTING = { ...HELLO_TEXT, x: 300, y: 20 };

/** A text in a box 40 × 40 at (200, 20). */
const IN_BOX = { ...HELLO_TEXT, boxHeight: 40, boxWidth: 40, isInBox: true, x: 200, y: 20 };

describe("textAfterPanelEdit (F38)", () => {
  it("[F38.AC2] keeps the size of a text fitting its content: 460 in W, +10 in H", () => {
    expect(textAfterPanelEdit(FITTING, BLOCK, edit("width", 460, false))).toEqual(FITTING);
    expect(textAfterPanelEdit(FITTING, BLOCK, edit("height", 10, true))).toEqual(FITTING);
  });

  it("[F38.AC2] keeps a text fitting its content as it is for -100 in W, not refused", () => {
    // 80 − 100 is below 0, but the text has no width to set.
    expect(textAfterPanelEdit(FITTING, BLOCK, edit("width", -100, true))).toEqual(FITTING);
  });

  it("[F38.AC2] moves a text fitting its content: 50 in X, -20 in Y", () => {
    expect(textAfterPanelEdit(FITTING, BLOCK, edit("x", 50, false))).toEqual({ ...FITTING, x: 50 });
    expect(textAfterPanelEdit(FITTING, BLOCK, edit("y", -20, true))).toEqual({ ...FITTING, y: 0 });
  });

  it("[F38.AC2] moves the block turned by 90° by its straight box: 0 in X", () => {
    // About its middle (340, 35), the frame turned by 90° starts at x 325 (`textPanelValues`):
    // the position moves by 0 − 325, from 300 to −25.
    const turned = { ...FITTING, angle: 90 };
    const moved = textAfterPanelEdit(turned, BLOCK, edit("x", 0, false));

    expect(moved.x).toBeCloseTo(-25, 9);
    expect(moved.y).toBe(20);
  });

  it("[F38.AC2] sets the box of a text in a box: 460 in W, +10 in H", () => {
    expect(textAfterPanelEdit(IN_BOX, BLOCK, edit("width", 460, false))).toEqual({
      ...IN_BOX,
      boxWidth: 460,
    });

    expect(textAfterPanelEdit(IN_BOX, BLOCK, edit("height", 10, true))).toEqual({
      ...IN_BOX,
      boxHeight: 50,
    });
  });

  it("[F38.AC2] moves a text in a box by its box: -20 in X gives x 180", () => {
    expect(textAfterPanelEdit(IN_BOX, BLOCK, edit("x", -20, true))).toEqual({ ...IN_BOX, x: 180 });
  });

  it("[F38.AC2] keeps the turned top-left corner of a box turned by 23° made 120 wide (ADR-0057)", () => {
    // ADR-0057's example: 100 × 20 at (47, 61) turned by 23°, its turned top-left corner at
    // (54.88207, 42.25840); 120 wide, its position becomes (46.20505, 64.90731).
    const turned = { ...IN_BOX, angle: 23, boxHeight: 20, boxWidth: 100, x: 47, y: 61 };
    const resized = textAfterPanelEdit(turned, BLOCK, edit("width", 120, false));

    expect(resized.boxWidth).toBe(120);
    expect(resized.x).toBeCloseTo(46.20505, 5);
    expect(resized.y).toBeCloseTo(64.90731, 5);
  });

  it("[F44.AC4] makes a box 40 wide 0 wide for -50 in W: −10 taken at 0", () => {
    expect(textAfterPanelEdit(IN_BOX, BLOCK, edit("width", -50, true))).toEqual({
      ...IN_BOX,
      boxWidth: 0,
    });
  });

  it("caps a box's width at 100 000, as the model's range of a box (Q20)", () => {
    expect(textAfterPanelEdit(IN_BOX, BLOCK, edit("width", 250_000, false)).boxWidth).toBe(100_000);
  });

  it("caps a box's height at a box's own largest side, 100 000, whatever the interface's cap", () => {
    // An interface capping at 200 000: the box still stops at TEXT_RANGES.boxSide.last, 100 000.
    const larger: PanelEdit = {
      key: "height",
      maxSize: 200_000,
      typed: { isRelative: false, value: 150_000 },
    };

    expect(textAfterPanelEdit(IN_BOX, BLOCK, larger).boxHeight).toBe(100_000);
  });

  it("caps a box's height at the interface's cap when smaller than a box's own", () => {
    // An interface capping at 500: 600 typed gives 500.
    const smaller: PanelEdit = {
      key: "height",
      maxSize: 500,
      typed: { isRelative: false, value: 600 },
    };

    expect(textAfterPanelEdit(IN_BOX, BLOCK, smaller).boxHeight).toBe(500);
  });
});
