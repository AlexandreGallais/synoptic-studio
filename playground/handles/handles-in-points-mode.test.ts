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
  return document.querySelector<HTMLElement>(`#${id}`)?.hidden !== false;
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

  it("[F12.AC2] [F12.AC4] bends the side when the handle after is typed, the handle before mirrored", () => {
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

  it("draws the handles as marks in Points mode", () => {
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "30");

    expect(document.querySelectorAll("#canvas path.curve-handle")).toHaveLength(2);
  });

  it("refuses a value that is not an integer: the side stays straight", () => {
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "1.5");

    expect(readText("#path-data")).not.toContain("C");
  });

  it("[F12.AC4] types the ends as x and y on the page, for a point away from the origin", () => {
    // Point 2 at (120, 0): after at (150, −15) is the vector (30, −15); before, its mirror, at (90, 15).
    clickPoint(2, false);
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "150");
    typeAndEnter("handle-after-y", "-15");

    expect(readHandleFields()).toEqual(["90", "15", "150", "-15"]);
    expect(readText("#path-data")).toContain("C150 -15 ");

    expect(
      [...document.querySelectorAll("#canvas path.curve-handle")].map(
        (mark) => mark.getAttribute("d")?.split(" M", 2)[1],
      ),
    ).toEqual(
      expect.arrayContaining([
        "146 -19 L154 -19 L154 -11 L146 -11 Z",
        "86 11 L94 11 L94 19 L86 19 Z",
      ]),
    );
  });

  it("[F12.AC4] types the handle before too", () => {
    // Before at (−20, 0) from point 1: after mirrored at (20, 0).
    clickElement("handles-switch");
    typeAndEnter("handle-before-x", "-20");

    expect(readHandleFields()).toEqual(["-20", "0", "20", "0"]);
  });

  it("[F12.AC2] keeps the other handle's length in mode angle", () => {
    // Mode angle, before (−20, 0) set first, then after typed (0, −30): before keeps 20, opposite.
    clickElement("handles-switch");
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-before-x", "-20");
    chooseOption("handle-mode", "angle");
    typeAndEnter("handle-after-y", "-30");

    expect(readHandleFields()).toEqual(["0", "20", "0", "-30"]);
  });

  it("[F12.AC4] keeps the other coordinate where it is drawn on a turned shape", () => {
    // At 45° the point is drawn at decimal coordinates: After x alone is taken, After y kept.
    typeAndEnter("angle", "45");
    clickElement("handles-switch");

    const before = readHandleFields();

    typeAndEnter("handle-after-x", "80");

    expect(readHandleFields()[2]).toBe("80");
    expect(readHandleFields()[3]).toBe(before[3]);
  });

  it("shows the mode of each point when the selection changes", () => {
    clickElement("handles-switch");
    chooseOption("handle-mode", "none");
    clickPoint(2, false);
    clickElement("handles-switch");

    expect(selectChoice("handle-mode").value).toBe("angle-and-length");

    clickPoint(1, false);

    expect(selectChoice("handle-mode").value).toBe("none");
  });

  it("draws no mark for zero handles, nor for handles switched off; one mark for one handle", () => {
    clickElement("handles-switch");

    expect(document.querySelectorAll("#canvas path.curve-handle")).toHaveLength(0);

    chooseOption("handle-mode", "none");
    typeAndEnter("handle-after-x", "30");

    expect(document.querySelectorAll("#canvas path.curve-handle")).toHaveLength(1);

    clickElement("handles-switch");

    expect(document.querySelectorAll("#canvas path.curve-handle")).toHaveLength(0);
  });

  it("hides the switch for two points selected, and for a polygon's point", () => {
    clickPoint(2, true);

    expect(isHidden("handles-switch")).toBe(true);

    // Step 10: a hexagon.
    openOnStep(10);
    clickElement("mode-points");
    clickPoint(1, false);

    expect(isHidden("handles-switch")).toBe(true);
  });

  it("keeps the text of a value that is not an integer in its field", () => {
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "1.5");

    expect(selectInput("handle-after-x").value).toBe("1.5");
  });
});
