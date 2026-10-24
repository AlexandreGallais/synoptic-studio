// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  openServedPage,
  typeAndEnter,
  typeValue,
} from "../user-actions";

/**
 * Reads the path data of the canvas's paths of a class, in order.
 *
 * @param look - the class: `turned-box` for a turned box, `straight-box` for a dashed one
 * @returns the path data of each
 */
function readFrames(look: string): readonly (string | null)[] {
  return [...document.querySelectorAll(`#canvas path.${look}`)].map((path) =>
    path.getAttribute("d"),
  );
}

/**
 * Places the text tool's text in a box at (0, 0), as the designer types it.
 *
 * @param box - its width and height, and its angle
 * @param box.width - the value typed in Width
 * @param box.height - the value typed in Height
 * @param box.angle - the value typed in Angle
 */
function typeTextBox(box: {
  readonly width: string;
  readonly height: string;
  readonly angle: string;
}): void {
  chooseShapeKind("text");
  chooseOption("text-fit", "box");
  typeAndEnter("text-box-width", box.width);
  typeAndEnter("text-box-height", box.height);
  typeAndEnter("shape-x", "0");
  typeAndEnter("shape-y", "0");
  typeValue("angle", box.angle);
}

/**
 * Draws a rectangle at (0, 0), as the designer types it.
 *
 * @param angle - the value typed in Angle
 * @returns the paths of its turned box and of its straight box
 */
function readRectangleFrames(angle: string): readonly (readonly (string | null)[])[] {
  chooseShapeKind("rectangle");
  typeAndEnter("width", "60");
  typeAndEnter("height", "40");
  typeValue("radius", "0");
  typeAndEnter("shape-x", "0");
  typeAndEnter("shape-y", "0");
  typeValue("angle", angle);

  return [readFrames("turned-box"), readFrames("straight-box")];
}

describe("A turned text's frames drawn as a turned shape's (US-050)", () => {
  beforeEach(() => {
    openServedPage("");
  });

  it("[F17.AC7] draws a box 60 × 40 turned by 30 with the blue turned box and the dashed straight box of a rectangle", () => {
    const [, rectangleStraight] = readRectangleFrames("30");

    // A page opened again: the text drawn alone, not placed from the rectangle (EN-142).
    openServedPage("");
    typeTextBox({ angle: "30", height: "40", width: "60" });

    // The straight box, upright, the same dashes as the rectangle's.
    expect(readFrames("straight-box")).toEqual(rectangleStraight);

    // The turned box, a thin blue frame between the box's contour and one inset by 1, turned by
    // 30 about the box's middle (30, 20) as the text is.
    expect(readFrames("turned-box")).toEqual([
      "M0 0 L60 0 L60 40 L0 40 Z M1 1 L59 1 L59 39 L1 39 Z",
    ]);

    expect(document.querySelector("#canvas g.text-frame")?.getAttribute("transform")).toBe(
      "rotate(30 30 20)",
    );
  });

  it("[F17.AC7] keeps the dashed frame of a text at 0°, no turned box", () => {
    typeTextBox({ angle: "0", height: "40", width: "60" });

    expect(readFrames("turned-box")).toEqual([]);
    expect(readFrames("straight-box")).toHaveLength(1);
  });

  it("[F17.AC7] draws no frame for a box 0 wide, turned or not", () => {
    typeTextBox({ angle: "30", height: "40", width: "0" });

    expect([readFrames("turned-box"), readFrames("straight-box")]).toEqual([[], []]);
  });
});
