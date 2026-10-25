// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseShapeKind,
  clickElement,
  clickPoint,
  openPage,
  pressKey,
  readDisabledState,
  readPointRows,
  readText,
  readValue,
  selectInput,
  typeAndEnter,
} from "../../../../../testing";

/**
 * Whether an element of the page is hidden.
 *
 * @param id - its id
 * @returns its `hidden` property
 */
function isHidden(id: string): boolean {
  return document.querySelector<HTMLElement>(`#${id}`)?.hidden === true;
}

describe("the Round tool", () => {
  it("[F10.AC1] draws a circle in a box 100 × 100: four cubic sides from its top", () => {
    // DERIV-round-shape-curve: handles 50κ = 27.61424, so 50 + 27.61424 = 77.61424.
    openPage("");
    typeAndEnter("width", "100");
    typeAndEnter("height", "100");
    chooseShapeKind("round");

    expect(readText("#shape-title")).toBe("Round shape");

    expect(readText("#path-data")).toBe(
      "M50 0 C77.61424 0 100 22.38576 100 50 C100 77.61424 77.61424 100 50 100 C22.38576 100 0 77.61424 0 50 C0 22.38576 22.38576 0 50 0 Z",
    );
  });

  it("[F10.AC1] draws an ellipse touching the four sides of a box 120 × 80", () => {
    // Handles 60κ = 33.13708 across, 40κ = 22.09139 down.
    openPage("");
    chooseShapeKind("round");
    typeAndEnter("width", "120");
    typeAndEnter("height", "80");

    expect(readText("#path-data")).toBe(
      "M60 0 C93.13708 0 120 17.90861 120 40 C120 62.09139 93.13708 80 60 80 C26.86292 80 0 62.09139 0 40 C0 17.90861 26.86292 0 60 0 Z",
    );

    expect(readText("#straight-box")).toBe("Straight box: x 0, y 0, width 120, height 80");
  });

  it("[F10.AC2] has no radius nor anchor, and shows its 4 points without a handles switch", () => {
    openPage("");
    chooseShapeKind("round");

    expect([
      isHidden("corners-section"),
      isHidden("anchor-field"),
      isHidden("corners-field"),
    ]).toEqual([true, true, true]);

    clickElement("mode-points");
    clickPoint(1, false);

    expect(readPointRows()).toEqual([
      "1 · x 60 · y 0 · radius 0",
      "2 · x 120 · y 40 · radius 0",
      "3 · x 60 · y 80 · radius 0",
      "4 · x 0 · y 40 · radius 0",
    ]);

    expect(isHidden("handles-switch")).toBe(true);
    expect(isHidden("point-radius-field")).toBe(true);

    expect([selectInput("point-x").disabled, selectInput("point-y").disabled]).toEqual([
      true,
      true,
    ]);
  });

  it("[F10.AC2] greys adding and deleting points, and keeps the curve; offers converting", () => {
    openPage("");
    chooseShapeKind("round");
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);

    const drawn = readText("#path-data");

    expect([
      readDisabledState("add-midpoint") === "true",
      readDisabledState("delete-points") === "true",
      isHidden("convert"),
    ]).toEqual([true, true, false]);

    clickElement("add-midpoint");
    clickElement("delete-points");
    pressKey("body", "Delete");

    expect(readText("#path-data")).toBe(drawn);
  });

  it("gives the rectangle back its radius after Round, and the polygon its own", () => {
    // The page opens on the rectangle 120 × 80, radius 10.
    openPage("");

    const rectangle = readText("#path-data");

    chooseShapeKind("round");
    chooseShapeKind("rectangle");

    expect(readText("#path-data")).toBe(rectangle);
    expect([readValue("radius"), isHidden("corners-section")]).toEqual(["10", false]);

    chooseShapeKind("polygon");

    const polygon = readText("#path-data");

    chooseShapeKind("round");
    chooseShapeKind("polygon");

    expect(readText("#path-data")).toBe(polygon);
  });

  it("turns about the centre of its box: at 30°, its top point at (80, 5.35898)", () => {
    // (60, 0) turned by 30° about (60, 40): (60 + 40 sin 30°, 40 − 40 cos 30°).
    openPage("");
    chooseShapeKind("round");
    typeAndEnter("angle", "30");

    expect(readText("#path-data")).toMatch(/^M80 5\.35898 C/u);
  });

  it("[F44.AC4] takes a width below 0 at 0, the round shape 0 wide", () => {
    openPage("");
    chooseShapeKind("round");
    // -200 is relative: 120 − 200 = −80, taken at 0.
    typeAndEnter("width", "-200");

    expect([readValue("width"), readText("#status")]).toEqual(["0", ""]);
  });

  it("refuses an angle emptied", () => {
    openPage("");
    chooseShapeKind("round");
    typeAndEnter("angle", "");

    expect(readText("#status")).toBe(
      "Width and height must be integers ≥ 0, and the angle an integer.",
    );
  });
});
