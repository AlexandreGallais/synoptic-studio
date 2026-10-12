// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { CURRENT_GUIDED_STEPS } from "./current-guided-steps";
import { GUIDED_STEPS } from "./guided-steps";
import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";
import { readGuidedTestSteps } from "./read-guided-test-steps";

/**
 * Opens the playground at an address, as a link would: the page as served, its head included so
 * that the style rules apply, without its script tag, mounted by the test with its steps.
 *
 * @param search - query of the address, e.g. `?guide=epic`
 */
function open(search: string): void {
  globalThis.history.replaceState(undefined, "", `/${search}`);

  const parser = new DOMParser();
  const parsed = parser.parseFromString(page, "text/html");

  parsed.querySelector("script")?.remove();
  document.head.replaceChildren(...parsed.head.childNodes);
  document.body.replaceChildren(...parsed.body.childNodes);
  mountPlayground(document, readGuidedTestSteps(document));
}

/**
 * Whether an element is out of sight: its computed display, so that a style rule overriding the
 * `hidden` attribute is caught.
 *
 * @param id - element id, without `#`
 * @returns `true` when it is not displayed
 */
function isHidden(id: string): boolean {
  const element = document.querySelector(`#${id}`);

  return element !== null && getComputedStyle(element).display === "none";
}

/**
 * Whether a toolbar button is pressed.
 *
 * @param id - button id
 * @returns its `aria-pressed` state
 */
function isPressed(id: string): boolean {
  return document.querySelector(`#${id}`)?.getAttribute("aria-pressed") === "true";
}

describe("toolbar", () => {
  beforeEach(() => {
    open("");
  });

  it("starts on the rectangle, its button pressed and the polygon's options hidden", () => {
    expect(isPressed("tool-rectangle")).toBe(true);
    expect(isPressed("tool-polygon")).toBe(false);
    expect(document.querySelector("#shape-title")?.textContent).toBe("Rectangle");
    expect(isHidden("polygon-options")).toBe(true);
  });

  it("draws a polygon from its button, its options shown", () => {
    document.querySelector<HTMLButtonElement>("#tool-polygon")?.click();

    expect(isPressed("tool-polygon")).toBe(true);
    expect(isPressed("tool-rectangle")).toBe(false);
    expect(document.querySelector("#shape-title")?.textContent).toBe("Polygon");
    expect(isHidden("polygon-options")).toBe(false);
    // A hexagon, the default number of corners, has six vertices in its contour.
    expect(document.querySelector("#contour")?.textContent.match(/"x"/gu)).toHaveLength(6);
  });
});

/**
 * Clicks an element of the page, as a mouse would.
 *
 * @param selector - CSS selector of the element
 */
function clickOn(selector: string): void {
  document.querySelector(selector)?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
}

/**
 * Value of an input of the page.
 *
 * @param id - input id
 * @returns its value, empty when absent
 */
function valueOf(id: string): string {
  return document.querySelector<HTMLInputElement>(`#${id}`)?.value ?? "";
}

describe("toolbar after a free shape", () => {
  beforeEach(() => {
    open("");
    // A pentagon 120 × 80 converted to a free shape: its width becomes 84.11698.
    clickOn("#tool-polygon");

    const corners = document.querySelector<HTMLInputElement>("#corners");

    if (corners !== null) {
      corners.value = "5";
      corners.dispatchEvent(new Event("input"));
    }

    clickOn("#mode-points");
    clickOn("#convert");
    clickOn('#points [data-vertex="1"]');

    const pointX = document.querySelector<HTMLInputElement>("#point-x");

    if (pointX === null) {
      return;
    }

    pointX.value = "30";
    pointX.dispatchEvent(new Event("change"));
  });

  it("draws a valid rectangle, its width rounded and one radius for every corner", () => {
    expect(valueOf("width")).toBe("84.11698");

    clickOn("#tool-rectangle");

    expect(valueOf("width")).toBe("84");
    // The radius field was empty (Mixed): the first radius kept, 10, goes to every corner.
    expect(valueOf("radius")).toBe("10");
    expect(document.querySelectorAll('[aria-invalid="true"]')).toHaveLength(0);
  });

  it("goes back to the shape the page opens with on Reset, in Shape mode", () => {
    clickOn("#tool-reset");

    expect([valueOf("width"), valueOf("height"), valueOf("radius")]).toEqual(["120", "80", "10"]);
    expect(isPressed("tool-rectangle")).toBe(true);
    expect(isPressed("mode-shape")).toBe(true);
    expect(document.querySelectorAll('[aria-invalid="true"]')).toHaveLength(0);
  });
});

describe("pipeline column", () => {
  it("is shown, then hidden and shown again by its button", () => {
    open("");

    expect(isHidden("output")).toBe(false);

    clickOn("#tool-output");

    expect([isHidden("output"), isPressed("tool-output")]).toEqual([true, false]);

    clickOn("#tool-output");

    expect([isHidden("output"), isPressed("tool-output")]).toEqual([false, true]);
  });
});

describe("guided test of the page", () => {
  it("is hidden when the feature being validated has no steps", () => {
    open("");

    expect(CURRENT_GUIDED_STEPS).toHaveLength(0);
    expect(isHidden("guide")).toBe(true);
  });

  it("shows every step of the epic at ?guide=epic", () => {
    open("?guide=epic");

    expect(isHidden("guide")).toBe(false);
    expect(document.querySelector("#guide-count")?.textContent).toBe(String(GUIDED_STEPS.length));
  });
});
