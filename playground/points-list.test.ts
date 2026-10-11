// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/** Modifier keys of a click. */
type Keys = { readonly ctrlKey?: boolean; readonly shiftKey?: boolean };

/**
 * Clicks an element of the page, with modifier keys, as a user would.
 *
 * @param selector - CSS selector of the element
 * @param keys - keys held
 */
function clickOn(selector: string, keys: Keys): void {
  document
    .querySelector(selector)
    ?.dispatchEvent(new MouseEvent("click", { ...keys, bubbles: true }));
}

/**
 * Types a value into an input and fires its `input` event, as a user would.
 *
 * @param id - input id
 * @param value - typed text
 */
function type(id: string, value: string): void {
  const input = document.querySelector<HTMLInputElement>(`#${id}`);

  if (input === null) {
    return;
  }

  input.value = value;
  input.dispatchEvent(new Event("input"));
}

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
 * The radius input of the page.
 *
 * @returns the input, or a detached one when the page has none
 */
function radiusField(): HTMLInputElement {
  return document.querySelector<HTMLInputElement>("#radius") ?? document.createElement("input");
}

/**
 * Text shown in an element of the page.
 *
 * @param selector - CSS selector of the element
 * @returns its text content, empty when absent
 */
function text(selector: string): string {
  return document.querySelector(selector)?.textContent ?? "";
}

describe("list of points and selection", () => {
  beforeEach(() => {
    // Step 10 of the guided test: a hexagon in 100 × 100, radius 0 (F02); then Points mode.
    globalThis.history.replaceState(undefined, "", "/?step=10");

    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document);
    clickOn("#mode-points", {});
  });

  it("[F03.AC4] lists the points in contour order with x, y and radius, in Points mode only", () => {
    expect(document.querySelectorAll("#points li")).toHaveLength(6);
    expect(text('#points [data-vertex="1"]')).toBe("1 · x 25 · y 6.69873 · radius 0");
    expect(text('#points [data-vertex="3"]')).toBe("3 · x 100 · y 50 · radius 0");

    clickOn("#mode-shape", {});

    expect(document.querySelector<HTMLElement>("#points")?.hidden).toBe(true);
  });

  it("[F03.AC5] disables the radius field until a point is selected", () => {
    expect(radiusField().disabled).toBe(true);
    expect(radiusField().placeholder).toBe("Select points");

    clickOn('#canvas path.handle[data-vertex="3"]', {});

    expect(pressedRows()).toEqual(["3"]);
    expect(document.querySelectorAll("#canvas path.halo")).toHaveLength(1);
    expect(radiusField().disabled).toBe(false);
    expect(radiusField().value).toBe("0");
  });

  it("[F03.AC5] adds with Ctrl or, on the drawing, Shift; selects a range with Shift in the list", () => {
    clickOn('#points [data-vertex="2"]', {});
    clickOn('#points [data-vertex="5"]', { ctrlKey: true });

    expect(pressedRows()).toEqual(["2", "5"]);

    // On the drawing, Shift adds one vertex: no range.
    clickOn('#canvas path.handle[data-vertex="1"]', { shiftKey: true });

    expect(pressedRows()).toEqual(["1", "2", "5"]);

    // In the list, Shift selects from the start, vertex 5 (the last clicked without Shift), to
    // vertex 4, replacing the selection.
    clickOn('#points [data-vertex="4"]', { shiftKey: true });

    expect(pressedRows()).toEqual(["4", "5"]);
  });

  it("[F03.AC5] gives the radius typed to the selected points only", () => {
    // Vertex 2 first gets a radius of its own, which must stay.
    clickOn('#points [data-vertex="2"]', {});
    type("radius", "3");
    clickOn('#points [data-vertex="1"]', {});
    clickOn('#points [data-vertex="4"]', { ctrlKey: true });
    type("radius", "10");

    expect(radiusField().dataset["radii"]).toBe("10 3 0 10 0 0");
    expect(text("#path-data").match(/A10 10/gu)).toHaveLength(2);
    expect(text('#points [data-vertex="1"]')).toBe("1 · x 25 · y 6.69873 · radius 10");
    expect(text('#points [data-vertex="2"]')).toBe("2 · x 75 · y 6.69873 · radius 3");

    // Vertices 1 and 2 differ (10 and 3): Mixed.
    clickOn('#points [data-vertex="2"]', { ctrlKey: true });

    expect(radiusField().value).toBe("");
    expect(radiusField().placeholder).toBe("Mixed");
  });

  it("[F03.AC4] shows the effective radius of a point when it was reduced (Q8)", () => {
    // Hexagon edge 50, turning angle 60°: setback 1000 · tan 30° = 577.35 per corner; the edge
    // allows 50, so r' = 1000 · 50 / 577.35 = 86.60254.
    clickOn('#points [data-vertex="1"]', {});
    type("radius", "1000");

    expect(text('#points [data-vertex="1"]')).toBe(
      "1 · x 25 · y 6.69873 · radius 1000 (effective 86.60254)",
    );
  });

  it("[F03.AC5] clears the selection when the number of corners changes or Shape mode is chosen", () => {
    clickOn('#points [data-vertex="2"]', {});
    type("corners", "5");

    expect(pressedRows()).toEqual([]);

    clickOn('#points [data-vertex="2"]', {});
    clickOn("#mode-shape", {});

    expect(document.querySelector<HTMLElement>("#points")?.dataset["selected"]).toBe("");
  });

  it("[F03.AC5] keeps every radius while the shape is briefly refused", () => {
    clickOn('#points [data-vertex="6"]', {});
    type("radius", "9");
    type("width", "");

    // Refused: no list nor handle of the last valid shape.
    expect(document.querySelector<HTMLElement>("#points")?.hidden).toBe(true);
    expect(document.querySelectorAll("#canvas path.handle")).toHaveLength(0);

    type("width", "100");

    expect(radiusField().dataset["radii"]).toBe("0 0 0 0 0 9");
  });

  it("[F03.AC5] clears the selection at each step of the guided test and when the shape changes", () => {
    clickOn('#points [data-vertex="2"]', {});
    document.querySelector<HTMLButtonElement>("#guide-next")?.click();

    expect(pressedRows()).toEqual([]);

    clickOn('#points [data-vertex="2"]', {});

    const select = document.querySelector<HTMLSelectElement>("#shape");

    if (select !== null) {
      select.value = "rectangle";
      select.dispatchEvent(new Event("change"));
    }

    expect(pressedRows()).toEqual([]);
  });
});
