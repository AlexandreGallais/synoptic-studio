// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { clickElement, clickPoint, openPage, readText, typeValue } from "./user-actions";

/**
 * The radius input of the page.
 *
 * @returns the input, or a detached one when the page has none
 */
function radiusField(): HTMLInputElement {
  return document.querySelector<HTMLInputElement>("#radius") ?? document.createElement("input");
}

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
    typeValue("point-radius", radius);
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

    expect(radiusField().value).toBe("");
    expect(radiusField().placeholder).toBe("Mixed");

    expect(readText("#effective")).toBe(
      "Effective radius: mixed, shown for each point in Points mode.",
    );

    // One fillet only, on vertex 1.
    expect(readText("#path-data").match(/A10 10/gu)).toHaveLength(1);
  });

  it("[F03.AC2] gives every vertex the radius typed over Mixed", () => {
    giveRadii("10 0 0 0 0 0");
    typeValue("radius", "7");

    expect(radiusField().dataset["radii"]).toBe("7 7 7 7 7 7");
    expect(radiusField().placeholder).toBe("");
    expect(readText("#path-data").match(/A7 7/gu)).toHaveLength(6);
  });

  it("[F03.AC2] shows the common radius when the radii kept are all equal", () => {
    giveRadii("4 4 4 4 4 4");

    expect(radiusField().value).toBe("4");
    expect(radiusField().placeholder).toBe("");
  });

  it("[F03.AC3] keeps the radii by vertex number when the number of corners changes", () => {
    giveRadii("10 0 0 0 0 6");
    typeValue("corners", "5");

    expect(radiusField().dataset["radii"]).toBe("10 0 0 0 0");

    typeValue("corners", "8");

    expect(radiusField().dataset["radii"]).toBe("10 0 0 0 0 0 0 0");
  });

  it("[F03.AC3] never gives a removed vertex its radius back, even while the shape is refused", () => {
    giveRadii("10 0 0 0 0 6");
    typeValue("width", "");
    typeValue("corners", "5");
    typeValue("corners", "6");
    typeValue("width", "100");

    expect(radiusField().dataset["radii"]).toBe("10 0 0 0 0 0");
  });

  it("[F03.AC2] types one radius for every vertex at each step of the guided test", () => {
    giveRadii("10 0 0 0 0 6");
    document.querySelector<HTMLButtonElement>("#guide-next")?.click();

    // Step 11 is a triangle with radius 0: three radii, all 0.
    expect(radiusField().dataset["radii"]).toBe("0 0 0");
    expect(radiusField().placeholder).toBe("");
    expect(radiusField().value).toBe("0");
  });

  it("[F03.AC2] stops reading Mixed as soon as the field is typed in", () => {
    giveRadii("10 0 0 0 0 6");
    typeValue("radius", "-");

    expect(radiusField().placeholder).toBe("");
  });
});
