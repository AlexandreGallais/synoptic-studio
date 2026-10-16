// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  openServedPage,
  readDisplay,
  readText,
  selectPageElement,
  typeValue,
} from "../user-actions";

// "H" over "HH" in Inter Regular 16: block 23.78125 × 2 × 19.359375 = 38.71875 at (0, 0), its
// middle (11.890625, 19.359375).

/**
 * Types a content in the Text tool's text area, as the designer would.
 *
 * @param content - the lines, a line feed between two
 */
function typeContent(content: string): void {
  const area = selectPageElement("#text-content");

  if (area instanceof HTMLTextAreaElement) {
    area.value = content;
  }

  area.dispatchEvent(new Event("input", { bubbles: true }));
}

/**
 * Reads the dashed paths drawn upright, outside the text's turned frame: its straight box.
 *
 * @returns the path data of each
 */
function readStraightBoxes(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas > svg > path.straight-box")].map((path) =>
    path.getAttribute("d"),
  );
}

/**
 * Puts the text in a box, its width and height typed.
 *
 * @param width - the value typed in Width
 * @param height - the value typed in Height
 */
function typeBox(width: string, height: string): void {
  chooseOption("text-fit", "box");
  typeValue("text-box-width", width);
  typeValue("text-box-height", height);
}

describe("Text tool: the straight box of a turned text (US-039)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    typeValue("text-size", "16");
    // happy-dom loses the option marked `selected` once the page is moved into its document.
    chooseOption("text-weight", "400");
    chooseOption("text-vertical-alignment", "top");
    typeContent("H\nHH");
  });

  it("[F14.AC7] boxes a box 40 × 20 turned by 45: x −1.2132, y −11.2132, 42.42641 square", () => {
    // About (20, 10), half sizes 20c + 10s = 20s + 10c = 30·√2/2 = 21.21320.
    typeBox("40", "20");
    typeValue("angle", "45");

    expect(readText("#straight-box")).toBe(
      "Straight box: x -1.2132, y -11.2132, width 42.42641, height 42.42641",
    );

    expect(readDisplay("straight-box") === "none").toBe(false);
    // Its first dash, along the top from x −1.2132 to 2.7868, 1 thick.
    expect(readStraightBoxes()[0]).toMatch(/^M-1\.2132 -11\.2132H2\.7868V-10\.2132H-1\.2132Z /u);
  });

  it("[F14.AC7] boxes a box 60 × 40 turned by 90 at (10, −10), 40 × 60, upright, beside its frame", () => {
    // R(90°) about (30, 20): (0, 0) ↦ (50, −10), (60, 40) ↦ (10, 50).
    typeBox("60", "40");
    typeValue("angle", "90");

    expect(readText("#straight-box")).toBe("Straight box: x 10, y -10, width 40, height 60");
    expect(readStraightBoxes()).toHaveLength(1);
    expect(readStraightBoxes()[0]).toMatch(/^M10 -10H14V-9H10Z /u);

    expect(selectPageElement("#canvas g.text-frame").getAttribute("transform")).toBe(
      "rotate(90 30 20)",
    );
  });

  it("[F14.AC7] draws the straight box first, behind the turned frame and the text", () => {
    typeBox("60", "40");
    typeValue("angle", "90");

    // Painted in document order (SVG 1.1 §3.3): the first child is the furthest behind.
    const children = [...document.querySelectorAll("#canvas > svg > *")];
    const straight = children.findIndex((child) => child.matches("path.straight-box"));
    const frame = children.findIndex((child) => child.matches("g.text-frame"));

    expect(straight).toBeGreaterThanOrEqual(0);
    expect(frame).toBe(straight + 1);
  });

  it("[F14.AC7] boxes the block fitting its content turned by 90, as tall as the text is long", () => {
    // Half sizes swapped about (11.890625, 19.359375): x 11.890625 − 19.359375 = −7.46875,
    // y 19.359375 − 11.890625 = 7.46875.
    typeValue("angle", "90");

    expect(readText("#straight-box")).toBe(
      "Straight box: x -7.46875, y 7.46875, width 38.71875, height 23.78125",
    );

    // X and Y show the turned top-left corner (US-054): the middle plus R(90°) of (−11.890625,
    // −19.359375), (19.359375, −11.890625): (31.25, 7.46875), the straight box's top-right.
    expect(
      [selectPageElement("#shape-x"), selectPageElement("#shape-y")].map((field) =>
        field instanceof HTMLInputElement ? field.value : "",
      ),
    ).toEqual(["31.25", "7.46875"]);
  });

  it("[F14.AC7] shows one frame only at 0°, and no line", () => {
    typeValue("angle", "30");
    typeValue("angle", "0");

    expect(readStraightBoxes()).toHaveLength(0);
    expect(document.querySelectorAll("#canvas path.straight-box")).toHaveLength(1);
    expect(readDisplay("straight-box")).toBe("none");
  });

  it.each([
    { content: "", height: "", name: "an empty text", width: "" },
    { content: "H\nHH", height: "40", name: "a box 0 wide", width: "0" },
    { content: "H\nHH", height: "0", name: "a box 0 tall", width: "60" },
  ])("[F14.AC7] shows no straight box for $name turned by 30", ({ content, height, width }) => {
    typeContent(content);

    if (width !== "") {
      typeBox(width, height);
    }

    typeValue("angle", "30");

    expect(document.querySelectorAll("#canvas path.straight-box")).toHaveLength(0);
    expect(readDisplay("straight-box")).toBe("none");
  });

  it("hides the line of a turned text refused", () => {
    typeValue("angle", "30");
    typeValue("angle", "1.5");

    expect(readDisplay("straight-box")).toBe("none");
  });

  it("[F14.AC7] draws a turned shape's straight box as before once the text is left", () => {
    typeValue("angle", "30");
    clickElement("tool-rectangle");
    typeValue("angle", "90");

    // The rectangle 120 × 80 at (0, 0) turned by 90 about (60, 40): x 20, y −20, 80 × 120.
    expect(readText("#straight-box")).toBe("Straight box: x 20, y -20, width 80, height 120");
    expect(readDisplay("straight-box") === "none").toBe(false);
  });
});
