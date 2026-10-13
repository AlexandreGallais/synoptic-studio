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
  selectInput,
  typeAndEnter,
  typeValue,
} from "../user-actions";

import { GUIDED_STEPS_OF_EPIC_1 } from "./guided-steps-of-epic-1";

/**
 * Opens the playground on a step of F12, Points mode on, a point selected with its handles on.
 *
 * @param step - the step of F12, from 1
 * @param point - the point selected, from 1
 */
function openWithHandles(step: number, point: number): void {
  openOnStep(GUIDED_STEPS_OF_EPIC_1.length + step);
  clickElement("mode-points");
  clickPoint(point, false);
  clickElement("handles-switch");
}

/**
 * The values of the four handle fields: before, then after.
 *
 * @returns them as written in the fields
 */
function readHandleFields(): readonly string[] {
  return ["before-x", "before-y", "after-x", "after-y"].map(
    (field) => selectInput(`handle-${field}`).value,
  );
}

/**
 * Types the two values of a handle.
 *
 * @param end - `before` or `after`
 * @param values - the first and second values
 */
function typeHandle(end: string, values: readonly [string, string]): void {
  typeAndEnter(`handle-${end}-x`, values[0]);
  typeAndEnter(`handle-${end}-y`, values[1]);
}

/** The curve pasted on the top side in steps 9, 12 and 13: an arch 22.5 high. */
const ARCH = "C 40 -30 80 -30 120 0";

/**
 * Pastes a curve on the side after the point selected.
 *
 * @param text - what is pasted
 */
function paste(text: string): void {
  typeValue("handle-paste", text);
  clickElement("paste-curve");
}

describe("playground: the guided test of F12 (VAL-011)", () => {
  it("step 1: point 2 sharp with handles, its radius 20 kept and back", () => {
    openWithHandles(1, 2);

    expect(readPointRows()[1]).toBe("2 · x 120 · y 0 · radius 20 (effective 0)");

    clickElement("handles-switch");

    expect(readPointRows()[1]).toBe("2 · x 120 · y 0 · radius 20");
  });

  it("step 2: the handle before mirrored, the box of the curve", () => {
    openWithHandles(2, 2);
    typeHandle("after", ["150", "20"]);

    expect(readHandleFields()).toEqual(["90", "-20", "150", "20"]);
    expect([readValue("width"), readValue("height")]).toEqual(["133.33333", "88.88889"]);
  });

  it("step 3: Mirror angle keeps the other handle's own length", () => {
    openWithHandles(3, 2);
    chooseOption("handle-mode", "angle");
    typeAndEnter("handle-after-x", "150");
    typeAndEnter("handle-before-y", "-20");

    expect(readHandleFields()).toEqual(["120", "-20", "120", "30"]);
  });

  it("step 4: No mirroring leaves the other handle", () => {
    openWithHandles(4, 2);
    chooseOption("handle-mode", "none");
    typeAndEnter("handle-after-x", "150");
    typeAndEnter("handle-before-y", "-20");

    expect(readHandleFields()).toEqual(["120", "-20", "150", "0"]);
  });

  it("step 5: dx, dy draw as step 2", () => {
    openWithHandles(5, 2);
    chooseOption("handle-method", "point");
    typeHandle("after", ["30", "20"]);

    expect(readHandleFields()).toEqual(["-30", "-20", "30", "20"]);
    expect(readText("#path-data")).toBe("M0 0 C0 0 90 -20 120 0 C150 20 120 80 120 80 L0 80 Z");
  });

  it("step 6: 90° from the x axis points down", () => {
    openWithHandles(6, 2);
    chooseOption("handle-method", "axis");
    typeHandle("after", ["90", "40"]);

    expect(readHandleFields()).toEqual(["270", "40", "90", "40"]);
    expect(readText("#path-data")).toContain("C120 40 120 80 120 80");
  });

  it("step 7: equal angles from the side make a symmetric arch 15 high", () => {
    openWithHandles(7, 1);
    chooseOption("handle-method", "side");
    typeHandle("after", ["330", "40"]);
    clickPoint(2, false);
    clickElement("handles-switch");
    typeHandle("before", ["330", "40"]);

    expect(readText("#path-data")).toContain("C34.64102 -20 85.35898 -20 120 0");
    expect(readValue("height")).toBe("95");
  });

  it("step 8: equal shares make a symmetric arch", () => {
    openWithHandles(8, 1);
    chooseOption("handle-method", "share");
    typeHandle("after", ["30", "20"]);
    clickPoint(2, false);
    clickElement("handles-switch");
    typeHandle("before", ["30", "20"]);

    expect(readText("#path-data")).toContain("C36 -24 84 -24 120 0");
    expect(readValue("height")).toBe("98");
  });

  it("step 9: a pasted C draws its control points", () => {
    openWithHandles(9, 1);
    paste(ARCH);

    expect(readText("#path-data")).toContain("C40 -30 80 -30 120 0");
    expect(readValue("height")).toBe("102.5");
  });

  it("step 10: ease-in-out laid on the side makes an S", () => {
    openWithHandles(10, 1);
    paste("cubic-bezier(0.42, 0, 0.58, 1)");

    expect(readText("#path-data")).toContain("C25.2 25.2 94.8 -25.2 120 0");
  });

  it("step 11: the tension rounds, tightens, then straightens", () => {
    openWithHandles(11, 2);
    chooseOption("handle-mode", "auto");

    expect(readText("#path-data")).toContain("C0 0 100 -13.33333 120 0");

    typeAndEnter("handle-tension", "50");

    expect(readText("#path-data")).toContain("C0 0 110 -6.66667 120 0");

    typeAndEnter("handle-tension", "100");

    expect(readText("#path-data")).toBe("M0 0 L120 0 L120 80 L0 80 Z");
  });

  it("step 12: a point added on the arch moves nothing", () => {
    openWithHandles(12, 1);
    paste(ARCH);
    clickPoint(2, true);
    clickElement("add-midpoint");

    expect(readPointRows()[1]).toBe("2 · x 60 · y -22.5 · radius 0");
    expect([readValue("width"), readValue("height")]).toEqual(["155.55556", "102.5"]);
  });

  it("step 13: a deleted point's side keeps point 1's handle", () => {
    openWithHandles(13, 1);
    chooseOption("handle-mode", "none");
    paste(ARCH);
    clickPoint(2, false);
    clickElement("delete-points");

    expect(readText("#path-data")).toBe("M0 0 C40 -30 120 80 120 80 L0 80 Z");
  });
});
