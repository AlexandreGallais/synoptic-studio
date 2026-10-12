// @vitest-environment happy-dom
import { beforeAll, describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/**
 * Text shown in an element of the page.
 *
 * @param id - element id, without `#`
 * @returns its text content, empty when absent
 */
function text(id: string): string {
  return document.querySelector(`#${id}`)?.textContent ?? "";
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

/** Chooses Polygon in the shape selector and fires its `change` event. */
function choosePolygon(): void {
  const select = document.querySelector<HTMLSelectElement>("#shape");

  if (select === null) {
    return;
  }

  select.value = "polygon";
  select.dispatchEvent(new Event("change"));
}

/**
 * Clicks an element.
 *
 * @param id - element id
 */
function click(id: string): void {
  document.querySelector<HTMLElement>(`#${id}`)?.click();
}

/**
 * Whether the « Stretched » check box is out of sight, as the browser computes it.
 *
 * @returns `true` when it is not displayed
 */
function isStretchedFieldHidden(): boolean {
  const element = document.querySelector("#stretched-field");

  return element !== null && getComputedStyle(element).display === "none";
}

/**
 * Whether the « Stretched » check box is ticked.
 *
 * @returns its state
 */
function isStretched(): boolean {
  return document.querySelector<HTMLInputElement>("#stretched")?.checked === true;
}

/** Stretched hexagon in 100 × 100: six equal sides of 54.85838 (`DERIV-stretched-polygon`, checks). */
const STRETCHED_HEXAGON = "M77.42919 100 L22.57081 100 L0 50 L22.57081 0 L77.42919 0 L100 50 Z";

describe("playground: stretched polygon (US-021)", () => {
  beforeAll(() => {
    // The page as served, without its script tag: the test mounts the playground itself.
    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    document.head.replaceChildren(...parsed.head.childNodes);
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document);
  });

  it("[F06.AC5] shows the « Stretched » check box for the polygon only, unticked", () => {
    expect(isStretchedFieldHidden()).toBe(true);

    type("width", "100");
    type("height", "100");
    type("radius", "0");
    type("corners", "6");
    choosePolygon();

    expect(isStretchedFieldHidden()).toBe(false);
    expect(isStretched()).toBe(false);

    // The regular hexagon of F02, centered: flat top and bottom at (100 − 50√3)/2.
    expect(text("path-data")).toBe(
      "M75 93.30127 L25 93.30127 L0 50 L25 6.69873 L75 6.69873 L100 50 Z",
    );
  });

  it("[F06.AC1] [F06.AC5] fills the box when ticked, the anchor grid disabled", () => {
    click("stretched");

    expect(isStretched()).toBe(true);
    expect(text("path-data")).toBe(STRETCHED_HEXAGON);
    expect(document.querySelector("#anchor-field")?.hasAttribute("disabled")).toBe(true);
  });

  it("[F06.AC3] stays stretched when the number of corners and the size change", () => {
    type("corners", "5");
    type("width", "300");
    type("height", "50");

    // DERIV-stretched-polygon, checks: the pentagon in 300 × 50, stopped at its corner guard.
    expect(text("path-data")).toBe("M228.85967 50 L71.14033 50 L0 12.8595 L150 0 L300 12.8595 Z");
  });

  it("[F06.AC5] says the effective radii differ when its corners are clamped unequally", () => {
    type("radius", "1000");

    expect(text("effective")).toBe("Effective radius: mixed, shown for each point in Points mode.");
  });

  it("[F06.AC1] goes back to the regular polygon, placed by its anchor, when unticked", () => {
    type("radius", "0");
    type("corners", "6");
    type("width", "100");
    type("height", "100");
    click("stretched");

    expect(isStretched()).toBe(false);
    expect(document.querySelector("#anchor-field")?.hasAttribute("disabled")).toBe(false);

    expect(text("path-data")).toBe(
      "M75 93.30127 L25 93.30127 L0 50 L25 6.69873 L75 6.69873 L100 50 Z",
    );
  });

  it("[F06.AC4] converts a stretched polygon to a free shape without moving it", () => {
    click("stretched");
    click("mode-points");
    click("convert");

    expect(text("path-data")).toBe(STRETCHED_HEXAGON);
  });

  it("ticks and unticks safely on a refused polygon, then draws again once it is valid", () => {
    choosePolygon();
    type("radius", "-5");
    click("stretched");
    click("stretched");

    expect(text("status")).not.toBe("");
    expect(text("effective")).toBe("");

    type("radius", "0");

    expect(text("status")).toBe("");
    expect(text("effective")).not.toBe("");
  });

  it("unticks the check box when a step of the guided test draws a polygon of F02", () => {
    choosePolygon();
    click(isStretched() ? "shape" : "stretched");

    expect(isStretched()).toBe(true);

    click("guide-next");

    expect(isStretched()).toBe(false);
  });
});
