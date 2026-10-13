// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { chooseOption, clickWithKeys, openPage, readText, typeValue } from "./user-actions";

/**
 * Numbers of the rows pressed in the list of points, from 1.
 *
 * @returns the selected vertices as the list shows them
 */
function pressedRows(): readonly string[] {
  return [
    ...document.querySelectorAll<HTMLButtonElement>('#points button[aria-pressed="true"]'),
  ].map((button) => button.dataset["vertex"] ?? "");
}

/**
 * The radius input of the points, in Points mode.
 *
 * @returns the input, or a detached one when the page has none
 */
function radiusField(): HTMLInputElement {
  return (
    document.querySelector<HTMLInputElement>("#point-radius") ?? document.createElement("input")
  );
}

/**
 * The radii kept on the shape's radius field.
 *
 * @returns them as written, separated by spaces
 */
function keptRadii(): string | undefined {
  return document.querySelector<HTMLInputElement>("#radius")?.dataset["radii"];
}

describe("list of points and selection", () => {
  beforeEach(() => {
    // Step 10 of the guided test: a hexagon in 100 × 100, radius 0 (F02); then Points mode.
    openPage("?step=10");
    clickWithKeys("#mode-points", {});
  });

  it("[F03.AC4] lists the points in contour order with x, y and radius, in Points mode only", () => {
    expect(document.querySelectorAll("#points li")).toHaveLength(6);
    expect(readText('#points [data-vertex="1"]')).toBe("1 · x 75 · y 93.30127 · radius 0");
    expect(readText('#points [data-vertex="3"]')).toBe("3 · x 0 · y 50 · radius 0");

    clickWithKeys("#mode-shape", {});

    expect(document.querySelector<HTMLElement>("#points")?.hidden).toBe(true);
  });

  it("[F03.AC5] disables the radius field until a point is selected", () => {
    expect(radiusField().disabled).toBe(true);
    expect(radiusField().placeholder).toBe("Select points");

    clickWithKeys('#canvas path.handle[data-vertex="3"]', {});

    expect(pressedRows()).toEqual(["3"]);
    expect(document.querySelectorAll("#canvas path.halo")).toHaveLength(1);
    expect(radiusField().disabled).toBe(false);
    expect(radiusField().value).toBe("0");
  });

  it("[F03.AC5] adds with Ctrl or, on the drawing, Shift; selects a range with Shift in the list", () => {
    clickWithKeys('#points [data-vertex="2"]', {});
    clickWithKeys('#points [data-vertex="5"]', { ctrlKey: true });

    expect(pressedRows()).toEqual(["2", "5"]);

    // On the drawing, Shift adds one vertex: no range.
    clickWithKeys('#canvas path.handle[data-vertex="1"]', { shiftKey: true });

    expect(pressedRows()).toEqual(["1", "2", "5"]);

    // In the list, Shift selects from the start, vertex 5 (the last clicked without Shift), to
    // vertex 4, replacing the selection.
    clickWithKeys('#points [data-vertex="4"]', { shiftKey: true });

    expect(pressedRows()).toEqual(["4", "5"]);
  });

  it("[F03.AC5] gives the radius typed to the selected points only", () => {
    // Vertex 2 first gets a radius of its own, which must stay.
    clickWithKeys('#points [data-vertex="2"]', {});
    typeValue("point-radius", "3");
    clickWithKeys('#points [data-vertex="1"]', {});
    clickWithKeys('#points [data-vertex="4"]', { ctrlKey: true });
    typeValue("point-radius", "10");

    expect(keptRadii()).toBe("10 3 0 10 0 0");
    expect(readText("#path-data").match(/A10 10/gu)).toHaveLength(2);
    expect(readText('#points [data-vertex="1"]')).toBe("1 · x 75 · y 93.30127 · radius 10");
    expect(readText('#points [data-vertex="2"]')).toBe("2 · x 25 · y 93.30127 · radius 3");

    // Vertices 1 and 2 differ (10 and 3): Mixed.
    clickWithKeys('#points [data-vertex="2"]', { ctrlKey: true });

    expect(radiusField().value).toBe("");
    expect(radiusField().placeholder).toBe("Mixed");
  });

  it("[F03.AC4] shows the effective radius of a point when it was reduced (Q8)", () => {
    // Hexagon edge 50, turning angle 60°: setback 1000 · tan 30° = 577.35 per corner; the edge
    // allows 50, so r' = 1000 · 50 / 577.35 = 86.60254.
    clickWithKeys('#points [data-vertex="1"]', {});
    typeValue("point-radius", "1000");

    expect(readText('#points [data-vertex="1"]')).toBe(
      "1 · x 75 · y 93.30127 · radius 1000 (effective 86.60254)",
    );
  });

  it("[F03.AC5] clears the selection when the number of corners changes or Shape mode is chosen", () => {
    clickWithKeys('#points [data-vertex="2"]', {});
    typeValue("corners", "5");

    expect(pressedRows()).toEqual([]);

    clickWithKeys('#points [data-vertex="2"]', {});
    clickWithKeys("#mode-shape", {});

    expect(document.querySelector<HTMLElement>("#points")?.dataset["selected"]).toBe("");
  });

  it("[F03.AC5] keeps every radius while the shape is briefly refused", () => {
    clickWithKeys('#points [data-vertex="6"]', {});
    typeValue("point-radius", "9");
    typeValue("width", "");

    // Refused: no list nor handle of the last valid shape.
    expect(document.querySelector<HTMLElement>("#points")?.hidden).toBe(true);
    expect(document.querySelectorAll("#canvas path.handle")).toHaveLength(0);

    typeValue("width", "100");

    expect(keptRadii()).toBe("0 0 0 0 0 9");
  });

  it("[F03.AC5] clears the selection at each step of the guided test and when the shape changes", () => {
    clickWithKeys('#points [data-vertex="2"]', {});
    document.querySelector<HTMLButtonElement>("#guide-next")?.click();

    expect(pressedRows()).toEqual([]);

    clickWithKeys('#points [data-vertex="2"]', {});

    chooseOption("shape", "rectangle");

    expect(pressedRows()).toEqual([]);
  });

  it("[F02.AC5] caps a radius typed in Points mode at 100 000 (Q20)", () => {
    clickWithKeys('#points [data-vertex="1"]', {});
    typeValue("point-radius", "200000");

    expect(radiusField().value).toBe("100000");
    expect(keptRadii()).toBe("100000 0 0 0 0 0");
  });

  it("[F03.AC3] keeps a radius given in Points mode by vertex number when the corners change", () => {
    clickWithKeys('#points [data-vertex="1"]', {});
    typeValue("point-radius", "10");
    clickWithKeys('#points [data-vertex="6"]', {});
    typeValue("point-radius", "9");
    typeValue("corners", "5");

    expect(keptRadii()).toBe("10 0 0 0 0");

    typeValue("corners", "6");

    expect(keptRadii()).toBe("10 0 0 0 0 0");
  });

  it("[F03.AC3] keeps the radii by vertex number when switching to the rectangle and back", () => {
    clickWithKeys('#points [data-vertex="2"]', {});
    typeValue("point-radius", "7");

    chooseOption("shape", "rectangle");
    chooseOption("shape", "polygon");

    // Vertex 2 kept its 7 through the rectangle's four corners; vertices 5 and 6 come back sharp.
    expect(keptRadii()).toBe("0 7 0 0 0 0");
  });
});
