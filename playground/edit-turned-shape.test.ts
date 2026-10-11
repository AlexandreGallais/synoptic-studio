// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/**
 * Types a value into an input and fires its `input` then `change` events, as a user would who
 * presses Enter.
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
  input.dispatchEvent(new Event("change"));
}

/**
 * Clicks a button.
 *
 * @param id - button id
 */
function click(id: string): void {
  document.querySelector<HTMLButtonElement>(`#${id}`)?.click();
}

/**
 * Selects a point by clicking its row in the list of points.
 *
 * @param vertex - number of the point, from 1, as the list shows it
 */
function clickPoint(vertex: number): void {
  document
    .querySelector(`#points [data-vertex="${CSS.escape(String(vertex))}"]`)
    ?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
}

/**
 * Text shown in an element of the page.
 *
 * @param id - element id, without `#`
 * @returns its text content, empty when absent
 */
function text(id: string): string {
  return document.querySelector(`#${id}`)?.textContent ?? "";
}

describe("editing a turned shape (US-019)", () => {
  beforeEach(() => {
    // Step 1 of the guided test: a 120 × 80 rectangle (F01), sharp, turned by 90° about (60, 40).
    globalThis.history.replaceState(undefined, "", "/?step=1");

    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document);
    type("radius", "0");
    type("angle", "90");
  });

  it("[F04.AC4] keeps the turned top-left corner in place when the width is typed", () => {
    // Point 1 is drawn at (100, −20). Typed 200 wide, the shape stretches along its own top side,
    // downwards on screen: 80 wide and 200 high, point 1 still at (100, −20).
    expect(text("path-data")).toMatch(/^M100 -20 /u);

    type("width", "200");

    expect(text("path-data")).toMatch(/^M100 -20 /u);
    expect(text("straight-box")).toBe("Straight box: x 20, y -20, width 80, height 200");
  });

  it("[F04.AC4] is the straight shape with that size once turned back to 0°", () => {
    type("width", "200");
    type("angle", "0");

    expect(text("straight-box")).toMatch(/width 200, height 80$/u);
  });

  it("[F04.AC4] moves only the point typed, in the coordinates of the SVG", () => {
    click("mode-points");
    clickPoint(1);

    // The field shows point 1 as drawn: x 100.
    expect(document.querySelector<HTMLInputElement>("#point-x")?.value).toBe("100");

    type("point-x", "110");

    // Point 1 at (110, −20); points 2 to 4 drawn where they were: (100, 100), (20, 100), (20, −20).
    expect(text("path-data")).toBe("M110 -20 L100 100 L20 100 L20 -20 Z");
  });

  it("[F04.AC4] converts a turned polygon to a free shape that keeps its angle and its drawing", () => {
    const select = document.querySelector<HTMLSelectElement>("#shape");

    if (select !== null) {
      select.value = "polygon";
      select.dispatchEvent(new Event("change"));
    }

    type("corners", "3");
    type("angle", "30");

    const before = text("path-data");

    click("mode-points");
    click("convert");

    expect(text("path-data")).toBe(before);
    expect(document.querySelector<HTMLInputElement>("#angle")?.value).toBe("30");
  });
});
