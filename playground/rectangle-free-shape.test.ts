// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

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
 * Text shown in an element of the page.
 *
 * @param id - element id, without `#`
 * @returns its text content, empty when absent
 */
function text(id: string): string {
  return document.querySelector(`#${id}`)?.textContent ?? "";
}

describe("rectangle as a free shape", () => {
  beforeEach(() => {
    // Step 1 of the guided test: a 120 × 80 rectangle, radius 10 (F01).
    globalThis.history.replaceState(undefined, "", "/?step=1");

    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document);
  });

  it("[F09.AC4] shows the rectangle as a free shape: the corners of the unit square with their radii, its size", () => {
    const model: unknown = JSON.parse(text("model"));

    expect(model).toEqual({
      corners: [
        { point: { x: 0, y: 0 }, radius: 10 },
        { point: { x: 1, y: 0 }, radius: 10 },
        { point: { x: 1, y: 1 }, radius: 10 },
        { point: { x: 0, y: 1 }, radius: 10 },
      ],
      height: 80,
      width: 120,
    });
  });

  it("[F09.AC3] keeps the stored points at width 0, and comes back the same", () => {
    const before = text("path-data");
    const model = text("model");

    type("width", "0");

    // Nothing drawn across, the stored unit square untouched.
    expect(text("path-data")).toBe("M0 0 L0 80 Z");

    expect(JSON.parse(text("model"))).toMatchObject({
      corners: [
        { point: { x: 0, y: 0 } },
        { point: { x: 1, y: 0 } },
        { point: { x: 1, y: 1 } },
        { point: { x: 0, y: 1 } },
      ],
      width: 0,
    });

    type("width", "120");

    expect(text("model")).toBe(model);
    expect(text("path-data")).toBe(before);

    expect(before).toBe(
      "M10 0 L110 0 A10 10 0 0 1 120 10 L120 70 A10 10 0 0 1 110 80 L10 80 A10 10 0 0 1 0 70 L0 10 A10 10 0 0 1 10 0 Z",
    );
  });

  it("[F09.AC4] lets Points mode round one corner of the rectangle, as in F03", () => {
    document.querySelector<HTMLButtonElement>("#mode-points")?.click();

    document
      .querySelector('#points [data-vertex="3"]')
      ?.dispatchEvent(new MouseEvent("click", { bubbles: true }));

    type("radius", "30");

    expect(document.querySelector<HTMLInputElement>("#radius")?.dataset["radii"]).toBe(
      "10 10 30 10",
    );

    expect(text("path-data")).toContain("A30 30");
  });

  it("[F09.AC4] lists the drawn points of the rectangle, not the stored ones", () => {
    document.querySelector<HTMLButtonElement>("#mode-points")?.click();

    expect(document.querySelector('#points [data-vertex="1"]')?.textContent).toBe(
      "1 · x 0 · y 0 · radius 10",
    );

    expect(document.querySelector('#points [data-vertex="3"]')?.textContent).toBe(
      "3 · x 120 · y 80 · radius 10",
    );
  });

  it("[F09.AC4] still refuses a fractional width typed for the rectangle (Q15)", () => {
    type("width", "2.5");

    expect(text("status")).toBe("Width, height and radius must be integers ≥ 0.");
  });
});
