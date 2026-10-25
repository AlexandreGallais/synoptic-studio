// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openOnStep,
  readText,
  selectInput,
  typeAndEnter,
} from "../user-actions";

/**
 * The values of the four handle fields: before angle and length, after angle and length.
 *
 * @returns them as numbers
 */
function readHandleFields(): readonly number[] {
  return ["before-x", "before-y", "after-x", "after-y"].map((field) =>
    Number(selectInput(`handle-${field}`).value),
  );
}

/** Switches the point selected to handles, typed as an angle and a length from the side. */
function openAngleFields(): void {
  clickElement("handles-switch");
  chooseOption("handle-method", "side");
}

describe("handles typed as an angle and a length", () => {
  beforeEach(() => {
    // The rectangle 100 × 80: point 1 (0, 0), point 2 (100, 0), point 3 (100, 80); Points mode.
    // Radius 0: the control points typed are drawn as they are, no fillet cutting a side.
    openOnStep(1);
    typeAndEnter("radius", "0");
    typeAndEnter("width", "100");
    clickElement("mode-points");
    clickPoint(1, false);
  });

  it("[F12.AC4] counts the angle from the side, clockwise on screen", () => {
    // Point 1's side goes right: 330°, 30 from (0, 0) ends at (25.98076, −15).
    openAngleFields();
    typeAndEnter("handle-after-x", "330");
    typeAndEnter("handle-after-y", "30");

    expect(readText("#path-data")).toContain("C25.98076 -15 ");
    expect(readHandleFields().slice(2)).toEqual([330, 30]);
  });

  it("[F12.AC4] counts it from the side, so that a side going down bends the same way", () => {
    // Point 2 (100, 0), its side going down to (100, 80): 330°, 30 from the side ends at (115, 25.98076).
    clickPoint(2, false);
    openAngleFields();
    typeAndEnter("handle-after-x", "330");
    typeAndEnter("handle-after-y", "30");

    expect(readText("#path-data")).toContain("C115 25.98076 ");
  });

  it("[F12.AC4] makes a symmetric arch with the same values at both ends of a side", () => {
    // Point 1 after and point 2 before, 330°, 30 from the side: both handles above the top side.
    openAngleFields();
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-after-x", "330");
    typeAndEnter("handle-after-y", "30");
    clickPoint(2, false);
    openAngleFields();
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-before-x", "330");
    typeAndEnter("handle-before-y", "30");

    expect(readText("#path-data")).toContain("C25.98076 -15 74.01924 -15 100 0");
    expect(readHandleFields().slice(0, 2)).toEqual([330, 30]);
  });

  it("[F12.AC4] brings 370° back to 10°, and refuses a negative length", () => {
    openAngleFields();
    typeAndEnter("handle-after-y", "30");
    typeAndEnter("handle-after-x", "370");

    expect(readHandleFields()[2]).toBeCloseTo(10, 5);

    typeAndEnter("handle-after-y", "-5");

    expect(readHandleFields()[3]).toBe(-5);
    expect(readText("#path-data")).toContain("C29.54423 5.20945 ");
  });

  it("labels the fields angle and length", () => {
    openAngleFields();

    expect(
      [...document.querySelectorAll("#handle-fields .handle-axis")].map(
        (label) => label.textContent,
      ),
    ).toEqual(["angle", "length", "angle", "length"]);
  });

  it("[F12.AC4] reads point 1's handle before against the side back to the last point", () => {
    // Point 1's side before goes down to (0, 80): mirrored, 330°, 30 gives (−15, 25.98076).
    openAngleFields();
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-before-x", "330");
    typeAndEnter("handle-before-y", "30");

    expect(readText("#path-data")).toContain("-15 25.98076 0 0");
  });

  it("never takes an angle kept for another point", () => {
    // Point 1 keeps 90° for its handle of length 0; point 2's length typed alone gives 0° from its
    // side going down, the handle (0, 30).
    openAngleFields();
    typeAndEnter("handle-after-x", "90");
    clickPoint(2, false);
    openAngleFields();
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-after-y", "30");

    expect(readText("#path-data")).toContain("C100 30 ");
  });
});
