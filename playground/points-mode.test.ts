// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/**
 * Clicks a button of the page.
 *
 * @param id - button id
 */
function click(id: string): void {
  document.querySelector<HTMLButtonElement>(`#${id}`)?.click();
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
 * Handles drawn on the canvas, in drawing order (the last one on top).
 *
 * @returns the handle paths
 */
function handles(): readonly SVGPathElement[] {
  return [...document.querySelectorAll<SVGPathElement>("#canvas path.handle")];
}

/**
 * Whether a mode button is pressed.
 *
 * @param id - `mode-shape` or `mode-points`
 * @returns its `aria-pressed` value
 */
function pressed(id: string): string {
  return document.querySelector(`#${id}`)?.getAttribute("aria-pressed") ?? "";
}

describe("points mode", () => {
  beforeEach(() => {
    // Step 10 of the guided test: a hexagon in 100 × 100 (F02).
    globalThis.history.replaceState(undefined, "", "/?step=10");

    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document);
  });

  it("[F03.AC4] opens on Shape mode, without handles", () => {
    expect(pressed("mode-shape")).toBe("true");
    expect(pressed("mode-points")).toBe("false");
    expect(handles()).toHaveLength(0);
  });

  it("[F03.AC4] shows one handle per vertex in Points mode, vertex 1 red on top, vertex 2 green", () => {
    click("mode-points");

    expect(pressed("mode-points")).toBe("true");
    expect(pressed("mode-shape")).toBe("false");

    expect(handles().map((handle) => handle.dataset["vertex"])).toEqual([
      "6",
      "5",
      "4",
      "3",
      "2",
      "1",
    ]);

    expect(handles().at(-1)?.classList.contains("first")).toBe(true);
    expect(handles().at(-2)?.classList.contains("second")).toBe(true);
    // Only one red and one green handle; vertices 3 to 6 neutral.
    expect(document.querySelectorAll("#canvas path.handle.first")).toHaveLength(1);
    expect(document.querySelectorAll("#canvas path.handle.second")).toHaveLength(1);

    // Vertex 1 of the hexagon in 100 × 100 is (25, 6.69873): a circle of radius 8 around it.
    expect(handles().at(-1)?.getAttribute("d")).toBe(
      "M17 6.69873 A8 8 0 1 1 33 6.69873 A8 8 0 1 1 17 6.69873 Z",
    );
  });

  it("[F03.AC4] hides the handles back in Shape mode", () => {
    click("mode-points");
    click("mode-shape");

    expect(handles()).toHaveLength(0);
  });

  it("[F03.AC4] draws the red handle on top when handles coincide (0 × 50 rectangle)", () => {
    const select = document.querySelector<HTMLSelectElement>("#shape");

    if (select !== null) {
      select.value = "rectangle";
      select.dispatchEvent(new Event("change"));
    }

    type("width", "0");
    type("height", "50");
    click("mode-points");

    // Vertices 1 (0, 0) and 2 (0, 0) coincide: the red one is the last drawn.
    expect(handles().at(-1)?.classList.contains("first")).toBe(true);
    expect(handles()).toHaveLength(4);
  });
});
