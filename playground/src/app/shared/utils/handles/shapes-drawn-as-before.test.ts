// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openPage,
  readPointRows,
  readText,
  readValue,
  typeAndEnter,
} from "../../../../../testing";

/** Opens the page on its 120 × 80 rectangle rounded by 20, in Points mode: point 1 top left. */
function openRoundedRectangle(): void {
  openPage("");
  typeAndEnter("radius", "20");
  clickElement("mode-points");
}

describe("shapes of E01 and F12 drawn as before", () => {
  it("[F10.AC6] [F13.AC3] widens a rounded body, its radius kept", () => {
    openPage("");
    typeAndEnter("width", "160");
    typeAndEnter("height", "100");
    typeAndEnter("radius", "12");
    typeAndEnter("width", "200");

    // 200 × 100 rounded by 12: each side shortened by 12 at both ends, 200 − 12 = 188,
    // 100 − 12 = 88.
    expect(readText("#path-data")).toBe(
      "M12 0 L188 0 A12 12 0 0 1 200 12 L200 88 A12 12 0 0 1 188 100 L12 100 A12 12 0 0 1 0 88 L0 12 A12 12 0 0 1 12 0 Z",
    );

    expect(readText("#effective")).toBe("Effective radius: 12 (as requested)");
  });

  it("[F10.AC6] [F13.AC3] makes a symmetric arch of the same angle and length at both ends of a side", () => {
    openRoundedRectangle();
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

    // Angle 330° (−30°) and length 40 from each end of the 120 top side: 40 cos 30° = 34.64102,
    // 40 sin 30° = 20, mirrored at the other end, 120 − 34.64102 = 85.35898.
    expect(readText("#path-data")).toContain("C34.64102 -20 85.35898 -20 120 0");
    expect(readValue("height")).toBe("95");
  });

  it("[F13.AC4] draws a radius too large beside a curve smaller, never sharp", () => {
    openRoundedRectangle();
    clickPoint(3, false);
    clickElement("handles-switch");
    chooseOption("handle-mode", "angle-and-length");
    typeAndEnter("handle-after-x", "90");
    typeAndEnter("handle-after-y", "110");
    clickPoint(2, false);
    typeAndEnter("point-radius", "60");

    // The radius the curve beside point 2 leaves room for, carried over from F13's first test of it.
    expect(readPointRows()[1]).toBe("2 · x 120 · y 0 · radius 60 (effective 47.18996)");
    expect(readText("#path-data")).toContain("A47.18996 47.18996 ");
  });
});
