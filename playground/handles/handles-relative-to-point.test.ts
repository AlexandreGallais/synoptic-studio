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
 * The axis written before each handle field.
 *
 * @returns x and y, or dx and dy, for before then after
 */
function readAxisLabels(): readonly (string | null)[] {
  return [...document.querySelectorAll("#handle-fields .handle-axis")].map(
    (label) => label.textContent,
  );
}

describe("handles typed as dx and dy from their point (US-023)", () => {
  beforeEach(() => {
    // The rectangle 100 × 80: point 2 at (100, 0); Points mode, point 2 selected, handles on.
    openOnStep(1);
    typeAndEnter("width", "100");
    clickElement("mode-points");
    clickPoint(2, false);
    clickElement("handles-switch");
    chooseOption("handle-method", "point");
  });

  it("[F12.AC4] puts the handle's end at the point plus dx and dy", () => {
    expect(readHandleFields()).toEqual(["0", "0", "0", "0"]);

    // dx = 30, dy = −15 from (100, 0): the end at (130, −15); the mirror (−30, 15).
    typeAndEnter("handle-after-x", "30");
    typeAndEnter("handle-after-y", "-15");

    expect(readHandleFields()).toEqual(["-30", "15", "30", "-15"]);
    expect(readText("#path-data")).toContain("C130 -15 ");

    chooseOption("handle-method", "page");

    expect(readHandleFields()).toEqual(["70", "15", "130", "-15"]);
  });

  it("[F12.AC4] reads the same dx and dy once the point is moved", () => {
    typeAndEnter("handle-after-x", "30");
    typeAndEnter("handle-after-y", "-15");
    typeAndEnter("point-x", "110");

    expect(readHandleFields().slice(2)).toEqual(["30", "-15"]);
    expect(readText("#path-data")).toContain("C140 -15 ");
  });

  it("labels the fields dx and dy, back to x and y on the page", () => {
    expect(readAxisLabels()).toEqual(["dx", "dy", "dx", "dy"]);

    chooseOption("handle-method", "page");

    expect(readAxisLabels()).toEqual(["x", "y", "x", "y"]);
  });

  it("[F12.AC4] types the handle before as dx and dy too", () => {
    typeAndEnter("handle-before-x", "-20");

    expect(readHandleFields()).toEqual(["-20", "0", "20", "0"]);
  });
});
