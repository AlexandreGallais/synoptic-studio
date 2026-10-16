// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openOnStep,
  readPointRows,
  readText,
  readValue,
  typeAndEnter,
} from "../../user-actions";
import { GUIDED_STEPS_OF_EPIC_1 } from "../epic-1";

/**
 * Opens the playground on a step of F12, in Points mode.
 *
 * @param step - the step of F12, from 1
 */
function openStep(step: number): void {
  openOnStep(GUIDED_STEPS_OF_EPIC_1.length + step);
  clickElement("mode-points");
}

/**
 * Point 3 in handles mode, its handle after ending at (90, 110).
 *
 * @param mode - the handle mode chosen first
 */
function bendPointThree(mode: string): void {
  clickPoint(3, false);
  clickElement("handles-switch");
  chooseOption("handle-mode", mode);
  typeAndEnter("handle-after-x", "90");
  typeAndEnter("handle-after-y", "110");
}

/**
 * The values of the four handle fields: before, then after.
 *
 * @returns them as written in the fields
 */
function readHandleFields(): readonly string[] {
  return ["before-x", "before-y", "after-x", "after-y"].map((field) =>
    readValue(`handle-${field}`),
  );
}

describe("playground: the guided test of F12 (VAL-011)", () => {
  it("step 1: point 2 sharp with handles, its radius 20 kept and back", () => {
    openStep(1);
    clickPoint(2, false);
    clickElement("handles-switch");

    expect(readPointRows()[1]).toBe("2 · x 120 · y 0 · radius 20 (effective 0)");

    clickElement("handles-switch");

    expect(readPointRows()[1]).toBe("2 · x 120 · y 0 · radius 20");
  });

  it("step 2: the handle before mirrored, points 2 and 4 still rounded by 20", () => {
    openStep(2);
    bendPointThree("angle-and-length");

    expect(readHandleFields()).toEqual(["150", "50", "90", "110"]);
    expect(readText("#path-data").match(/A20 20 /gu)).toHaveLength(3);
  });

  it("step 3: Mirror angle keeps the other handle's own length", () => {
    openStep(3);
    bendPointThree("angle");
    typeAndEnter("handle-before-y", "40");

    expect(readHandleFields()).toEqual(["120", "40", "120", "122.42641"]);
  });

  it("step 4: No mirroring leaves the other handle", () => {
    openStep(4);
    bendPointThree("none");
    typeAndEnter("handle-before-y", "40");

    expect(readHandleFields().slice(2)).toEqual(["90", "110"]);
  });

  it("[F10.AC6] [F13.AC3] step 5: the same angle and length at both ends of a side make a symmetric arch", () => {
    openStep(5);
    clickPoint(1, false);
    clickElement("handles-switch");
    chooseOption("handle-method", "side");
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-after-x", "330");
    typeAndEnter("handle-after-y", "40");
    clickPoint(2, false);
    clickElement("handles-switch");
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-before-x", "330");
    typeAndEnter("handle-before-y", "40");

    expect(readText("#path-data")).toContain("C34.64102 -20 85.35898 -20 120 0");
    expect(readValue("height")).toBe("95");
  });

  it("[F13.AC4] step 6: a radius too large beside a curve is drawn smaller, never sharp", () => {
    openStep(6);
    bendPointThree("angle-and-length");
    clickPoint(2, false);
    typeAndEnter("point-radius", "60");

    expect(readPointRows()[1]).toBe("2 · x 120 · y 0 · radius 60 (effective 47.18996)");
    expect(readText("#path-data")).toContain("A47.18996 47.18996 ");
  });

  it("step 7: a polygon's point gets handles, the polygon converted unmoved", () => {
    openStep(7);
    clickElement("mode-shape");
    chooseOption("shape", "polygon");
    clickElement("mode-points");
    clickPoint(1, false);
    clickElement("handles-switch");

    expect(readValue("shape")).toBe("free");
    expect(readPointRows()[0]).toMatch(/\(effective 0\)$/u);
  });

  it("step 8: a point added on a curve, the drawing kept", () => {
    openStep(8);
    bendPointThree("angle-and-length");

    const drawn = readText("#path-data").split(" C", 1)[0];

    clickPoint(2, false);
    clickPoint(3, true);
    clickElement("add-midpoint");

    expect(readPointRows()[2]).toBe("3 · x 131.25 · y 28.75 · radius 0");
    expect(readText("#path-data").split(" C", 1)[0]).toBe(drawn);
  });

  it("step 9: a point deleted next to a curve, the curve kept to point 1", () => {
    openStep(9);
    bendPointThree("none");
    clickPoint(4, false);
    clickElement("delete-points");

    expect(readPointRows()).toHaveLength(3);
    expect(readText("#path-data")).toContain("C101.42051 98.57949 ");
  });
});
