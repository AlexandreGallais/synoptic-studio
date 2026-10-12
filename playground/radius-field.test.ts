// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { GUIDED_STEPS } from "./guided-steps";
import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/**
 * The radius input of the page.
 *
 * @returns the input, or a detached one when the page has none
 */
function radiusField(): HTMLInputElement {
  return document.querySelector<HTMLInputElement>("#radius") ?? document.createElement("input");
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
  input.dispatchEvent(new Event("input", { bubbles: true }));
}

/**
 * Gives the vertices different radii, as the Points mode will (US-013), then redraws.
 *
 * @param radii - radius of each vertex, separated by spaces
 */
function keepRadii(radii: string): void {
  radiusField().dataset["radii"] = radii;
  document.querySelector("#width")?.dispatchEvent(new Event("input"));
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

describe("radius field", () => {
  beforeEach(() => {
    // Step 10 of the guided test: a hexagon in 100 × 100, radius 0 (F02).
    globalThis.history.replaceState(undefined, "", "/?step=10");

    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document, GUIDED_STEPS);
  });

  it("[F03.AC2] shows Mixed, empty, when the vertices have different radii", () => {
    keepRadii("10 0 0 0 0 0");

    expect(radiusField().value).toBe("");
    expect(radiusField().placeholder).toBe("Mixed");
    expect(text("effective")).toBe("Effective radius: mixed, shown for each point in Points mode.");
    // One fillet only, on vertex 1.
    expect(text("path-data").match(/A10 10/gu)).toHaveLength(1);
  });

  it("[F03.AC2] gives every vertex the radius typed over Mixed", () => {
    keepRadii("10 0 0 0 0 0");
    type("radius", "7");

    expect(radiusField().dataset["radii"]).toBe("7 7 7 7 7 7");
    expect(radiusField().placeholder).toBe("");
    expect(text("path-data").match(/A7 7/gu)).toHaveLength(6);
  });

  it("[F03.AC2] shows the common radius when the radii kept are all equal", () => {
    keepRadii("4 4 4 4 4 4");

    expect(radiusField().value).toBe("4");
    expect(radiusField().placeholder).toBe("");
  });

  it("[F03.AC3] keeps the radii by vertex number when the number of corners changes", () => {
    keepRadii("10 0 0 0 0 6");
    type("corners", "5");

    expect(radiusField().dataset["radii"]).toBe("10 0 0 0 0");

    type("corners", "8");

    expect(radiusField().dataset["radii"]).toBe("10 0 0 0 0 0 0 0");
  });

  it("[F03.AC3] never gives a removed vertex its radius back, even while the shape is refused", () => {
    keepRadii("10 0 0 0 0 6");
    type("width", "");
    type("corners", "5");
    type("corners", "6");
    type("width", "100");

    expect(radiusField().dataset["radii"]).toBe("10 0 0 0 0 0");
  });

  it("[F03.AC2] types one radius for every vertex at each step of the guided test", () => {
    keepRadii("10 0 0 0 0 6");
    document.querySelector<HTMLButtonElement>("#guide-next")?.click();

    // Step 11 is a triangle with radius 0: three radii, all 0.
    expect(radiusField().dataset["radii"]).toBe("0 0 0");
    expect(radiusField().placeholder).toBe("");
    expect(radiusField().value).toBe("0");
  });

  it("[F03.AC2] stops reading Mixed as soon as the field is typed in", () => {
    keepRadii("10 0 0 0 0 6");
    type("radius", "-");

    expect(radiusField().placeholder).toBe("");
  });
});
