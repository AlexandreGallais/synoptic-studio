// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  clickElement,
  clickPoint,
  openOnStep,
  readDisabledState,
  readPointRows,
  readPressedState,
  readText,
  readValue,
  typeAndEnter,
} from "../user-actions";

import { writeConvertedShape } from "./write-converted-shape";
import { writeRoundedRectangle } from "./write-rounded-rectangle";

/**
 * Opens the page on a round shape 120 × 80, in Points mode.
 *
 * @param width - width of its box
 */
function openRoundShape(width: string): void {
  openOnStep(1);
  clickElement("tool-round");
  typeAndEnter("width", width);
  typeAndEnter("height", "80");
  clickElement("mode-points");
}

describe("converting a round shape (US-030)", () => {
  it("[F10.AC4] gives its 4 points with their handles, the drawing unchanged", () => {
    openRoundShape("120");

    const drawn = readText("#path-data");

    clickElement("convert");

    expect(readValue("shape")).toBe("free");
    expect(readText("#path-data")).toBe(drawn);
    expect(readPointRows()).toHaveLength(4);

    clickPoint(1, false);

    expect(readPressedState("handles-switch")).toBe("true");
    expect(readValue("handle-mode")).toBe("angle-and-length");
  });

  it("[F10.AC5] gives the 4 corners of its box rounded by 40: a capsule", () => {
    // Half the smaller side, 80 / 2 = 40 (DERIV-round-shape step 2).
    openRoundShape("120");
    clickElement("convert-rounded");

    expect(readValue("shape")).toBe("free");

    expect(readPointRows()).toEqual([
      "1 · x 0 · y 0 · radius 40",
      "2 · x 120 · y 0 · radius 40",
      "3 · x 120 · y 80 · radius 40",
      "4 · x 0 · y 80 · radius 40",
    ]);

    expect(readText("#path-data")).toBe(
      "M40 0 L80 0 A40 40 0 0 1 120 40 A40 40 0 0 1 80 80 L40 80 A40 40 0 0 1 0 40 A40 40 0 0 1 40 0 Z",
    );
  });

  it.each([
    { height: "80", width: "0" },
    { height: "0", width: "120" },
  ])("greys both buttons on a box $width × $height", ({ height, width }) => {
    openRoundShape(width);
    typeAndEnter("height", height);

    expect([
      readDisabledState("convert") === "true",
      readDisabledState("convert-rounded") === "true",
    ]).toEqual([true, true]);
  });

  it("converts nothing of width 0 even when asked directly (the buttons' guard)", () => {
    openRoundShape("0");
    writeConvertedShape(document);
    writeRoundedRectangle(document);

    expect(readValue("shape")).toBe("round");
  });

  it("[F10.AC4] does not move a turned round shape", () => {
    openRoundShape("120");
    typeAndEnter("angle", "30");

    const drawn = readText("#path-data");

    clickElement("convert");

    expect([readText("#path-data"), readValue("angle")]).toEqual([drawn, "30"]);
  });

  it("[F10.AC5] rounds the box of a turned round shape about its centre", () => {
    // The box's corners turned by 30° about (60, 40), each √(60² + 40²) ≈ 72.111 from it.
    openRoundShape("120");
    typeAndEnter("angle", "30");
    clickElement("convert-rounded");

    expect(readPointRows()).toEqual([
      "1 · x 28.03848 · y -24.64102 · radius 40",
      "2 · x 131.96152 · y 35.35898 · radius 40",
      "3 · x 91.96152 · y 104.64102 · radius 40",
      "4 · x -11.96152 · y 44.64102 · radius 40",
    ]);
  });

  it("shows the conversion line on a round shape", () => {
    openRoundShape("120");

    expect(document.querySelector<HTMLElement>("#convert-hint")?.hidden).toBe(false);

    expect(readText("#convert-hint").trim()).toBe(
      "This shape keeps its points: convert it to a free shape to move, add or delete them.",
    );
  });

  it("gives the converted shape its own radii: 0 for its points, 40 for the rounded box", () => {
    // As a converted polygon (F08): the free shape's radii are written in the Radius field.
    openRoundShape("120");
    clickElement("convert");

    expect(readValue("radius")).toBe("0");

    openRoundShape("120");
    clickElement("convert-rounded");

    expect(readValue("radius")).toBe("40");
  });

  it("hides the handles switch on every point of a round shape", () => {
    // The switch converts a polygon first (US-028); a round shape's points already have handles.
    openRoundShape("120");

    const hidden = [1, 2, 3, 4].map((point) => {
      clickPoint(point, false);

      return document.querySelector<HTMLElement>("#handles-switch")?.hidden;
    });

    expect(hidden).toEqual([true, true, true, true]);
  });

  it("shows the rounded conversion for a round shape only", () => {
    openOnStep(1);
    clickElement("tool-polygon");
    clickElement("mode-points");

    expect(document.querySelector<HTMLElement>("#convert-rounded")?.hidden).toBe(true);
    expect(document.querySelector<HTMLElement>("#convert")?.hidden).toBe(false);
  });
});
