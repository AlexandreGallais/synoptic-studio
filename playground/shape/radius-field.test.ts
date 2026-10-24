// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  clickElement,
  clickPoint,
  openPage,
  readText,
  selectInput,
  typeAndEnter,
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
    // Step 10 of the guided test: a hexagon in 100 × 100, radius 0 (F02).
    openPage("?step=10");
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

  it("[F03.AC2] types one radius for every vertex at each step of the guided test", () => {
    giveRadii("10 0 0 0 0 6");
    clickElement("guide-next");

    // Step 11 is a triangle with radius 0: three radii, all 0.
    expect(selectInput("radius").dataset["radii"]).toBe("0 0 0");
    expect(selectInput("radius").placeholder).toBe("");
    expect(selectInput("radius").value).toBe("0");
  });

  it("[F03.AC2] stops reading Mixed as soon as the field is typed in", () => {
    giveRadii("10 0 0 0 0 6");
    typeAndEnter("radius", "-");

    expect(selectInput("radius").placeholder).toBe("");
  });
});
