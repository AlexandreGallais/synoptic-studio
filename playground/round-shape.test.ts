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
 * Text shown in an element of the page.
 *
 * @param id - element id, without `#`
 * @returns its text content, empty when absent
 */
function text(id: string): string {
  return document.querySelector(`#${id}`)?.textContent ?? "";
}

/**
 * Whether an element carries the `hidden` attribute (the page's style hides it, US-007).
 *
 * @param id - element id
 * @returns `true` when it is hidden
 */
function isHidden(id: string): boolean {
  return document.querySelector<HTMLElement>(`#${id}`)?.hidden === true;
}

/**
 * Chooses a shape in the selector, as a user would.
 *
 * @param value - `rectangle`, `polygon` or `round`
 */
function chooseShape(value: string): void {
  const select = document.querySelector<HTMLSelectElement>("#shape");

  if (select === null) {
    return;
  }

  select.value = value;
  select.dispatchEvent(new Event("change"));
}

describe("round shape in the playground (US-020)", () => {
  beforeEach(() => {
    // Step 1 of the guided test: a 120 × 80 box (F01).
    globalThis.history.replaceState(undefined, "", "/?step=1");

    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document);
    chooseShape("round");
  });

  it("[F10.AC4] draws a capsule in the 120 × 80 box, without radius field nor anchor grid", () => {
    // Half circles of radius 40, straight sides from x 40 to 80 (DERIV-round-shape checks).
    expect(text("path-data")).toBe(
      "M40 0 L80 0 A40 40 0 0 1 120 40 A40 40 0 0 1 80 80 L40 80 A40 40 0 0 1 0 40 A40 40 0 0 1 40 0 Z",
    );

    expect(isHidden("radius-field")).toBe(true);
    expect(isHidden("anchor-field")).toBe(true);
    expect(isHidden("corners-field")).toBe(true);
  });

  it("[F10.AC1] draws a circle in a square box, and stays round when it grows", () => {
    type("width", "100");
    type("height", "100");

    expect(text("path-data")).toBe(
      "M50 0 A50 50 0 0 1 100 50 A50 50 0 0 1 50 100 A50 50 0 0 1 0 50 A50 50 0 0 1 50 0 Z",
    );

    type("width", "300");
    type("height", "300");

    expect(text("path-data")).toMatch(/^M150 0 A150 150 /u);
  });

  it("[F10.AC2] turns and moves as every shape", () => {
    type("angle", "90");

    // The capsule stands up about (60, 40): 80 wide, 120 high.
    expect(text("straight-box")).toBe("Straight box: x 20, y -20, width 80, height 120");

    type("shape-x", "0");

    expect(text("straight-box")).toBe("Straight box: x 0, y -20, width 80, height 120");
  });

  it("[F10.AC3] converts it to a free shape of 4 points, radii 40, drawn the same", () => {
    const before = text("path-data");

    document.querySelector<HTMLButtonElement>("#mode-points")?.click();

    expect(isHidden("convert")).toBe(false);

    document.querySelector<HTMLButtonElement>("#convert")?.click();

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("free");
    expect(text("path-data")).toBe(before);
    expect(document.querySelectorAll("#points li")).toHaveLength(4);
    expect(text("points")).toContain("1 · x 0 · y 0 · radius 40");
    expect(isHidden("radius-field")).toBe(false);
  });

  it("[F10.AC4] shows no point to edit before the conversion", () => {
    document.querySelector<HTMLButtonElement>("#mode-points")?.click();

    expect(document.querySelector<HTMLInputElement>("#point-x")?.disabled).toBe(true);
    expect(isHidden("point-radius-field")).toBe(true);
  });
});
