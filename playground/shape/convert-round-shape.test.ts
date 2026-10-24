// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseShapeKind,
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

/**
 * Opens the page on a round shape 120 × 80, in Points mode.
 *
 * @param width - width of its box
 */
function openRoundShape(width: string): void {
  openOnStep(1);
  chooseShapeKind("round");
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

  it("[F10.AC5] offers no conversion to a rounded rectangle", () => {
    // Removed at VAL-012: an ellipse rounded as its box would change shape.
    openRoundShape("120");

    const buttons = [...document.querySelectorAll("button")].map((button) => button.textContent);

    expect(buttons).not.toContain("Convert to rounded rectangle");
    expect(document.querySelector("#convert-rounded")).toBeNull();
  });

  it.each([
    { height: "80", width: "0" },
    { height: "0", width: "120" },
  ])("greys the conversion on a box $width × $height", ({ height, width }) => {
    openRoundShape(width);
    typeAndEnter("height", height);

    expect(readDisabledState("convert")).toBe("true");
  });

  it("converts nothing of width 0 even when asked directly (the button's guard)", () => {
    openRoundShape("0");
    writeConvertedShape(document);

    expect(readValue("shape")).toBe("round");
  });

  it("[F10.AC4] does not move a turned round shape", () => {
    openRoundShape("120");
    typeAndEnter("angle", "30");

    const drawn = readText("#path-data");

    clickElement("convert");

    expect([readText("#path-data"), readValue("angle")]).toEqual([drawn, "30"]);
  });

  it("shows the conversion line on a round shape", () => {
    openRoundShape("120");

    expect(document.querySelector<HTMLElement>("#convert-hint")?.hidden).toBe(false);

    expect(readText("#convert-hint").trim()).toBe(
      "This shape keeps its points: convert it to a free shape to move, add or delete them.",
    );
  });

  it("gives the converted shape its own radii: 0 for its points", () => {
    // As a converted polygon (F08): the free shape's radii are written in the Radius field.
    openRoundShape("120");
    clickElement("convert");

    expect(readValue("radius")).toBe("0");
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

  it("shows the conversion for a polygon too", () => {
    openOnStep(1);
    chooseShapeKind("polygon");
    clickElement("mode-points");

    expect(document.querySelector<HTMLElement>("#convert")?.hidden).toBe(false);
  });
});
