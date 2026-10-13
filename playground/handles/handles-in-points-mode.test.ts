// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openOnStep,
  readPressedState,
  readText,
  selectChoice,
  selectInput,
  typeAndEnter,
} from "../user-actions";

/**
 * The values of the four handle fields: before x and y, after x and y.
 *
 * @returns them as written in the fields
 */
function readHandleFields(): readonly string[] {
  return ["before-x", "before-y", "after-x", "after-y"].map(
    (field) => selectInput(`handle-${field}`).value,
  );
}

/**
 * Whether an element of the page is hidden.
 *
 * @param id - its id
 * @returns its `hidden` property
 */
function isHidden(id: string): boolean {
  return document.querySelector<HTMLElement>(`#${id}`)?.hidden ?? true;
}

describe("handles in Points mode (US-022)", () => {
  beforeEach(() => {
    // Step 1: the rectangle 120 × 80, radius 10, point 1 at (0, 0); Points mode, point 1 selected.
    openOnStep(1);
    clickElement("mode-points");
    clickPoint(1, false);
  });

  it("[F12.AC1] shows the switch for the point selected, its radius field until it is pressed", () => {
    expect(isHidden("handles-switch")).toBe(false);
    expect(readPressedState("handles-switch")).toBe("false");
    expect([isHidden("point-radius-field"), isHidden("handle-fields")]).toEqual([false, true]);
  });

  it("[F12.AC1] [F12.AC2] switches to handles, mode angle and length, both handles at the point", () => {
    clickElement("handles-switch");

    expect(readPressedState("handles-switch")).toBe("true");
    expect([isHidden("point-radius-field"), isHidden("handle-fields")]).toEqual([true, false]);
    expect(selectChoice("handle-mode").value).toBe("angle-and-length");
    expect(readHandleFields()).toEqual(["0", "0", "0", "0"]);
  });

  it("[F12.AC2] [F12.AC3] bends the side when the handle after is typed, the handle before mirrored", () => {
    // After at (30, −15): the vector (30, −15); before mirrored at (−30, 15); the top side a cubic.
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "30");
    typeAndEnter("handle-after-y", "-15");

    expect(readHandleFields()).toEqual(["-30", "15", "30", "-15"]);
    expect(readText("#path-data")).toMatch(/^M.* C30 -15 /u);
  });

  it("[F12.AC2] leaves the handle before in mode none", () => {
    clickElement("handles-switch");
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-after-x", "30");

    expect(readHandleFields()).toEqual(["0", "0", "30", "0"]);
  });

  it("[F12.AC1] draws the radius again, as typed, when the switch is pressed again", () => {
    // Radius 10 at point 1: its fillet arc comes back.
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "30");
    clickElement("handles-switch");

    expect(readText("#path-data")).toContain("A10 10");
    expect(isHidden("point-radius-field")).toBe(false);
    expect(selectInput("point-radius").value).toBe("10");
  });

  it("[F12.AC3] draws the handles as marks in Points mode", () => {
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "30");

    expect(document.querySelectorAll("#canvas path.curve-handle")).toHaveLength(2);
  });

  it("refuses a value that is not an integer: the side stays straight", () => {
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "1.5");

    expect(readText("#path-data")).not.toContain("C");
  });
});
