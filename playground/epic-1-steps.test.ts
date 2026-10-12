// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/**
 * Opens the playground on a step of the guided test, as its address does.
 *
 * @param step - number of the step, from 1
 */
function openOnStep(step: number): void {
  globalThis.history.replaceState(undefined, "", `/?step=${String(step)}`);

  const parser = new DOMParser();
  const parsed = parser.parseFromString(page, "text/html");

  parsed.querySelector("script")?.remove();
  document.body.replaceChildren(...parsed.body.childNodes);
  mountPlayground(document);
}

/**
 * Types a value into an input and fires `input` then `change`, as typing then Enter does.
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
 * Clicks an element.
 *
 * @param id - element id
 */
function click(id: string): void {
  document.querySelector<HTMLElement>(`#${id}`)?.click();
}

/**
 * Clicks a row of the points list, adding it to the selection when Ctrl is held.
 *
 * @param vertex - point number, from 1
 * @param isCtrlHeld - whether Ctrl (Cmd on a Mac) is held
 */
function clickPoint(vertex: number, isCtrlHeld: boolean): void {
  document
    .querySelector(`#points [data-vertex="${CSS.escape(String(vertex))}"]`)
    ?.dispatchEvent(new MouseEvent("click", { bubbles: true, ctrlKey: isCtrlHeld }));
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
 * Value shown in an input.
 *
 * @param id - input id
 * @returns its value, empty when absent
 */
function valueOf(id: string): string {
  return document.querySelector<HTMLInputElement>(`#${id}`)?.value ?? "";
}

describe("playground: the user test of E01 (REV-001)", () => {
  it("step 67: the rounded body widens, its radius kept", () => {
    openOnStep(67);
    type("width", "200");

    expect(text("path-data")).toBe(
      "M12 0 L188 0 A12 12 0 0 1 200 12 L200 88 A12 12 0 0 1 188 100 L12 100 A12 12 0 0 1 0 88 L0 12 A12 12 0 0 1 12 0 Z",
    );

    expect(text("effective")).toBe("Effective radius: 12 (as requested)");
  });

  it("step 68: the capsule turned by 90° keeps its width and height", () => {
    openOnStep(68);
    type("angle", "90");

    expect(text("effective")).toBe("Effective radius: 20 (requested 1000, reduced to fit)");
    expect([valueOf("width"), valueOf("height")]).toStrictEqual(["160", "40"]);
    expect(text("straight-box")).toBe("Straight box: x 60, y -60, width 40, height 160");
  });

  it("step 69: only the base corners of the anchored triangle are rounded, its base kept", () => {
    openOnStep(69);
    click("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);
    type("point-radius", "15");

    // The base from 25.98 to 94.02 at y 120: a flat face keeps touching its anchored side.
    expect(text("path-data")).toBe(
      "M94.01924 120 L25.98076 120 A15 15 0 0 1 12.99038 97.5 L60 16.07695 L107.00962 97.5 A15 15 0 0 1 94.01924 120 Z",
    );

    expect(text("effective")).toBe("Effective radius: mixed, shown for each point in Points mode.");
  });

  it("step 70: the stretched hexagon, converted, gets a point under its base", () => {
    openOnStep(70);
    click("mode-points");

    const before = text("path-data");

    click("convert");

    expect(text("path-data")).toBe(before);

    clickPoint(1, false);
    clickPoint(2, true);
    click("add-midpoint");
    type("point-y", "120");

    expect(text("path-data")).toBe(
      "M113.97802 100 L80 120 L46.02198 100 L0 50 L46.02198 0 L113.97802 0 L160 50 Z",
    );

    expect(valueOf("height")).toBe("120");
  });

  it("step 71: the turned hexagon, converted then widened, keeps its left side", () => {
    openOnStep(71);
    type("angle", "30");
    click("mode-points");

    const before = text("path-data");

    click("convert");

    expect(text("path-data")).toBe(before);

    type("width", "200");

    // Point 3, on the left side of the box, stays at (10.71797, 10).
    expect(text("path-data")).toBe(
      "M109.10279 124.53753 L35.53822 82.06501 L10.71797 10 L85.53822 -4.53753 L159.10279 37.93499 L183.92305 110 Z",
    );

    expect(valueOf("angle")).toBe("30");
  });

  it("step 72: the rounded stretched hexagon, placed by its straight box", () => {
    openOnStep(72);

    expect(text("effective")).toBe("Effective radius: 50 (requested 1000, reduced to fit)");

    type("shape-x", "0");

    expect(text("straight-box")).toBe("Straight box: x 0, y 0, width 124.08793, height 100");
  });
});
