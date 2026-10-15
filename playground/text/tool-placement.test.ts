// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import { clickElement, openServedPage, readText, readValue, typeValue } from "../user-actions";

/**
 * Reads the `transform` of the text's group.
 *
 * @returns its value; null when it carries none
 */
function readTextTransform(): string | null | undefined {
  return document.querySelector("#canvas g:has(> text)")?.getAttribute("transform");
}

/**
 * Reads the X, Y and angle fields.
 *
 * @returns their values, in that order
 */
function readPlacement(): readonly string[] {
  return ["shape-x", "shape-y", "angle"].map((id) => readValue(id));
}

describe("Each tool keeps its own X, Y and angle (US-032)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-rectangle");
    typeValue("width", "200");
    typeValue("height", "100");
  });

  it("[F05.AC5] keeps a turned text's place when going to the rectangle and back", () => {
    clickElement("tool-text");
    // The page's content in Inter 32: happy-dom keeps the line break after <textarea>, so an
    // empty line, "Pump P-101", "Running": 3 × 2478 × 32 / 2048 = 116.15625 tall, 170.5 wide
    // ("Pump P-101"), its middle (85.25, 58.078125) at X 0, Y 0.
    typeValue("angle", "90");
    expect(readTextTransform()).toBe("rotate(90 85.25 58.07813)");

    clickElement("tool-rectangle");
    // The rectangle straight, at its own place.
    expect(readPlacement()).toEqual(["0", "0", "0"]);
    expect(document.querySelectorAll("#canvas path.turned-box")).toHaveLength(0);

    clickElement("tool-text");
    expect(readPlacement()).toEqual(["0", "0", "90"]);
    expect(readTextTransform()).toBe("rotate(90 85.25 58.07813)");
  });

  it("[F05.AC5] starts a new text straight, whatever the rectangle's angle, and gives it back", () => {
    typeValue("shape-x", "10");
    typeValue("angle", "30");

    const rectangle = [readPlacement(), readText("#path-data")];

    clickElement("tool-text");
    expect(readPlacement()).toEqual(["0", "0", "0"]);
    expect(readTextTransform()).toBeNull();

    typeValue("shape-x", "40");
    clickElement("tool-rectangle");
    // The rectangle back as it was: its fields and its drawing, still turned by 30.
    expect([readPlacement(), readText("#path-data")]).toEqual(rectangle);
    expect(readValue("angle")).toBe("30");
    expect(document.querySelectorAll("#canvas path.turned-box")).toHaveLength(1);

    clickElement("tool-text");
    expect(readPlacement()).toEqual(["40", "0", "0"]);
  });

  it("keeps the fields when the tool chosen is the same kind", () => {
    typeValue("angle", "30");
    clickElement("tool-polygon");
    expect(readValue("angle")).toBe("30");
  });
});
