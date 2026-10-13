// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openPage,
  readText,
  typeAndEnter,
} from "./user-actions";

describe("editing a turned shape (US-019)", () => {
  beforeEach(() => {
    // Step 1 of the guided test: a 120 × 80 rectangle (F01), sharp, turned by 90° about (60, 40).
    openPage("?step=1");
    typeAndEnter("radius", "0");
    typeAndEnter("angle", "90");
  });

  it("[F04.AC4] keeps the turned top-left corner in place when the width is typed", () => {
    // Point 1 is drawn at (100, −20). Typed 200 wide, the shape stretches along its own top side,
    // downwards on screen: 80 wide and 200 high, point 1 still at (100, −20).
    expect(readText("#path-data")).toMatch(/^M100 -20 /u);

    typeAndEnter("width", "200");

    expect(readText("#path-data")).toMatch(/^M100 -20 /u);
    expect(readText("#straight-box")).toBe("Straight box: x 20, y -20, width 80, height 200");
  });

  it("[F04.AC4] is the straight shape with that size once turned back to 0°", () => {
    typeAndEnter("width", "200");
    typeAndEnter("angle", "0");

    expect(readText("#straight-box")).toMatch(/width 200, height 80$/u);
  });

  it("[F04.AC4] moves only the point typed, in the coordinates of the SVG", () => {
    clickElement("mode-points");
    clickPoint(1, false);

    // The field shows point 1 as drawn: x 100.
    expect(document.querySelector<HTMLInputElement>("#point-x")?.value).toBe("100");

    typeAndEnter("point-x", "110");

    // Point 1 at (110, −20); points 2 to 4 drawn where they were: (100, 100), (20, 100), (20, −20).
    expect(readText("#path-data")).toBe("M110 -20 L100 100 L20 100 L20 -20 Z");
  });

  it("[F04.AC4] converts a turned polygon to a free shape that keeps its angle and its drawing", () => {
    chooseOption("shape", "polygon");

    typeAndEnter("corners", "3");
    typeAndEnter("angle", "30");

    const before = readText("#path-data");

    clickElement("mode-points");
    clickElement("convert");

    expect(readText("#path-data")).toBe(before);
    expect(document.querySelector<HTMLInputElement>("#angle")?.value).toBe("30");
  });

  it("[F04.AC4] keeps the turned top-left in place when a width above 100 000 is capped", () => {
    typeAndEnter("width", "150000");

    // Capped at 100 000 (Q20) before the shape is placed: point 1 still at (100, −20).
    expect(readText("#path-data")).toMatch(/^M100 -20 /u);
  });

  it("[F04.AC4] places nothing after a refused shape: turned back straight, the top-left is fixed", () => {
    typeAndEnter("angle", "");
    typeAndEnter("width", "200");
    typeAndEnter("angle", "0");

    // The width typed while the shape was refused moved nothing: a 200 × 80 rectangle at (0, 0).
    expect(readText("#straight-box")).toBe("Straight box: x 0, y 0, width 200, height 80");
  });

  it("[F04.AC4] keeps the turned top-left in place when the width is emptied, then retyped", () => {
    typeAndEnter("width", "");
    typeAndEnter("width", "2");
    typeAndEnter("width", "200");

    expect(readText("#path-data")).toMatch(/^M100 -20 /u);
  });
});
