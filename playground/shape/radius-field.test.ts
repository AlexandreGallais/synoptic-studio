// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  clickElement,
  clickPoint,
  openOnHexagon,
  pressKey,
  readText,
  selectInput,
  typeAndEnter,
  typeInField,
} from "../user-actions";

/**
 * Gives each vertex its radius through the page, as a user would: in Points mode, each point
 * selected alone and its radius typed, then back to Shape mode.
 *
 * @param radii - radius of each vertex, separated by spaces
 */
function giveRadii(radii: string): void {
  clickElement("mode-points");

  for (const [index, radius] of radii.split(" ").entries()) {
    clickPoint(index + 1, false);
    typeAndEnter("point-radius", radius);
  }

  clickElement("mode-shape");
}

describe("radius field", () => {
  beforeEach(() => {
    // A hexagon in 100 × 100, radius 0 (F02).
    openOnHexagon();
  });

  it("[F03.AC2] shows Mixed, empty, when the vertices have different radii", () => {
    giveRadii("10 0 0 0 0 0");

    expect(selectInput("radius").value).toBe("");
    expect(selectInput("radius").placeholder).toBe("Mixed");

    expect(readText("#effective")).toBe(
      "Effective radius: mixed, shown for each point in Points mode.",
    );

    // One fillet only, on vertex 1.
    expect(readText("#path-data").match(/A10 10/gu)).toHaveLength(1);
  });

  it("[F03.AC2] gives every vertex the radius typed over Mixed", () => {
    giveRadii("10 0 0 0 0 0");
    typeAndEnter("radius", "7");

    expect(selectInput("radius").dataset["radii"]).toBe("7 7 7 7 7 7");
    expect(selectInput("radius").placeholder).toBe("");
    expect(readText("#path-data").match(/A7 7/gu)).toHaveLength(6);
  });

  it("[F03.AC2] shows the common radius when the radii kept are all equal", () => {
    giveRadii("4 4 4 4 4 4");

    expect(selectInput("radius").value).toBe("4");
    expect(selectInput("radius").placeholder).toBe("");
  });

  it("[F03.AC3] keeps the radii by vertex number when the number of corners changes", () => {
    giveRadii("10 0 0 0 0 6");
    typeAndEnter("corners", "5");

    expect(selectInput("radius").dataset["radii"]).toBe("10 0 0 0 0");

    typeAndEnter("corners", "8");

    expect(selectInput("radius").dataset["radii"]).toBe("10 0 0 0 0 0 0 0");
  });

  it("[F03.AC3] never gives a removed vertex its radius back, even while the shape is refused", () => {
    giveRadii("10 0 0 0 0 6");
    typeAndEnter("width", "");
    typeAndEnter("corners", "5");
    typeAndEnter("corners", "6");
    typeAndEnter("width", "100");

    expect(selectInput("radius").dataset["radii"]).toBe("10 0 0 0 0 0");
  });

  it("[F03.AC2] types one radius for every vertex on Reset", () => {
    giveRadii("10 0 0 0 0 6");
    clickElement("tool-reset");

    // The page's rectangle, radius 10: four radii, all 10.
    expect(selectInput("radius").dataset["radii"]).toBe("10 10 10 10");
    expect(selectInput("radius").placeholder).toBe("");
    expect(selectInput("radius").value).toBe("10");
  });

  it("[F44.AC4] adds +5 typed over Mixed to each vertex's own radius, Mixed kept", () => {
    giveRadii("10 0 0 0 0 6");
    typeInField("radius", "+5");

    // 10 + 5, 0 + 5 four times, 6 + 5: each one's own, as several shapes' fields.
    expect(selectInput("radius").dataset["radii"]).toBe("15 5 5 5 5 11");
    expect([selectInput("radius").value, selectInput("radius").placeholder]).toEqual(["", "Mixed"]);
  });

  it("[F44.AC4] adds Up's 1 over Mixed to each vertex's own radius, -10 taken at 0", () => {
    giveRadii("10 0 0 0 0 6");
    selectInput("radius").focus();
    pressKey("#radius", "ArrowUp");

    expect(selectInput("radius").dataset["radii"]).toBe("11 1 1 1 1 7");

    typeInField("radius", "-10");

    // 11 − 10 = 1; 1 − 10 and 7 − 10 below 0, taken at 0.
    expect(selectInput("radius").dataset["radii"]).toBe("1 0 0 0 0 0");
  });

  it("[F44.AC4] keeps every radius for +0 typed over Mixed, a relative 0", () => {
    giveRadii("10 0 0 0 0 6");
    typeInField("radius", "+0");

    expect(selectInput("radius").dataset["radii"]).toBe("10 0 0 0 0 6");
  });

  it("[F44.AC4] refuses a text that is no number over Mixed, Mixed shown again", () => {
    giveRadii("10 0 0 0 0 6");
    typeInField("radius", "abc");

    expect(selectInput("radius").dataset["radii"]).toBe("10 0 0 0 0 6");
    expect([selectInput("radius").value, selectInput("radius").placeholder]).toEqual(["", "Mixed"]);
  });

  it("[F44.AC4] adds +2 typed in the points' field over Mixed to each selected one's own", () => {
    giveRadii("10 0 0 0 0 6");
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(6, true);
    typeInField("point-radius", "+2");

    // Vertices 1 and 6 selected: 10 + 2 and 6 + 2, the others kept.
    expect(selectInput("radius").dataset["radii"]).toBe("12 0 0 0 0 8");
  });

  it("[F03.AC2] stops reading Mixed as soon as the field is typed in", () => {
    giveRadii("10 0 0 0 0 6");
    typeAndEnter("radius", "-");

    expect(selectInput("radius").placeholder).toBe("");
  });
});
