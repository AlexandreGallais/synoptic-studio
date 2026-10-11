// @vitest-environment happy-dom
import { beforeAll, describe, expect, it } from "vitest";

import { GUIDED_STEPS } from "./guided-steps";
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

/**
 * Chooses a shape in the selector and fires its `change` event, as a user would.
 *
 * @param value - `rectangle` or `polygon`
 */
function chooseShape(value: string): void {
  const select = document.querySelector<HTMLSelectElement>("#shape");

  if (select === null) {
    return;
  }

  select.value = value;
  select.dispatchEvent(new Event("change"));
}

/**
 * Whether the field of the number of corners is out of sight: its computed display, so that a
 * style rule overriding the `hidden` attribute is caught.
 *
 * @returns `true` when `#corners-field` is not displayed
 */
function isCornersFieldHidden(): boolean {
  const field = document.querySelector("#corners-field");

  return field !== null && getComputedStyle(field).display === "none";
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

/**
 * Clicks a button of the page.
 *
 * @param id - button id
 */
function click(id: string): void {
  document.querySelector<HTMLButtonElement>(`#${id}`)?.click();
}

/**
 * Clicks a button of the page several times.
 *
 * @param id - button id
 * @param count - number of clicks, >= 0
 */
function clickTimes(id: string, count: number): void {
  if (!(count > 0)) {
    return;
  }

  click(id);
  clickTimes(id, count - 1);
}

/**
 * Clicks a cell of the anchor grid, as a user would: it is checked and fires `change`.
 *
 * @param value - value of the cell, horizontal then vertical alignment (`mid max`)
 * @throws {Error} when the page has no cell of this value
 */
function chooseAnchor(value: string): void {
  const buttons = document.querySelectorAll<HTMLInputElement>('input[name="anchor"]');
  const button = [...buttons].find((cell) => cell.value === value);

  if (button === undefined) {
    throw new Error(`No anchor cell ${value}`);
  }

  button.click();
}

/**
 * Value of the checked cell of the anchor grid.
 *
 * @returns its value, empty when none is checked
 */
function checkedAnchor(): string {
  const buttons = document.querySelectorAll<HTMLInputElement>('input[name="anchor"]');

  // By property: happy-dom's `:checked` follows the `checked` attribute, not the state.
  return [...buttons].find((button) => button.checked)?.value ?? "";
}

/**
 * Whether the anchor grid is out of sight, as the browser computes it.
 *
 * @returns `true` when `#anchor-field` is not displayed
 */
function isAnchorFieldHidden(): boolean {
  const field = document.querySelector("#anchor-field");

  return field !== null && getComputedStyle(field).display === "none";
}

describe("playground", () => {
  beforeAll(() => {
    // The page as served, without its script tag: the test mounts the playground itself.
    const parser = new DOMParser();
    const parsed = parser.parseFromString(page, "text/html");

    parsed.querySelector("script")?.remove();
    // The head carries the page's style: visibility is checked as the browser computes it.
    document.head.replaceChildren(...parsed.head.childNodes);
    document.body.replaceChildren(...parsed.body.childNodes);
    mountPlayground(document);
  });

  it("[F02.AC4] opens on the rectangle, the selector on Rectangle, without the number of corners", () => {
    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
    expect(isCornersFieldHidden()).toBe(true);
  });

  it("[F01.AC1] renders the rectangle as one <path>", () => {
    type("width", "100");
    type("height", "50");
    type("radius", "0");

    expect(document.querySelectorAll("#canvas svg path")).toHaveLength(1);
    expect(text("path-data")).toBe("M0 0 L100 0 L100 50 L0 50 Z");
  });

  it("[F01.AC2] shows the effective radius next to the requested one when it was reduced", () => {
    type("height", "25");
    type("radius", "100");

    expect(text("effective")).toBe("Effective radius: 12.5 (requested 100, reduced to fit)");
  });

  it("[F01.AC2] says the radius is as requested when it fits", () => {
    type("height", "50");
    type("radius", "10");

    expect(text("effective")).toBe("Effective radius: 10 (as requested)");
    expect(text("path-data")).toContain("A10 10 0 0 1 100 10");
  });

  it("refuses a negative radius and clears the effective radius", () => {
    type("radius", "-3");

    expect(text("status")).toBe("Width, height and radius must be integers ≥ 0.");
    expect(text("effective")).toBe("");
  });

  it("[F02.AC1] walks the steps of the guided test (F01 and F02), each showing what it tells to look at", () => {
    // "Previous" on the first step stays there and types its values again.
    click("guide-previous");

    // Effective radius text expected at each step, as announced by the step (US-003 card).
    const expected = [
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 12.5 (requested 100, reduced to fit)",
      "Effective radius: 50 (as requested)",
      "Effective radius: 50 (requested 1000, reduced to fit)",
      "Effective radius: 50 (requested 100, reduced to fit)",
      "Effective radius: 100 (as requested)",
      "Effective radius: 0 (as requested)",
      "",
      // F02 (US-007 and US-008 cards).
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 43.30127 (requested 1000, reduced to fit)",
      "Effective radius: 28.86751 (requested 1000, reduced to fit)",
      "Effective radius: 10 (as requested)",
      "",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
    ];

    for (const [index, effective] of expected.entries()) {
      expect(text("guide-number")).toBe(String(index + 1));
      expect(text("effective")).toBe(effective);
      click("guide-next");
    }

    // Past the last step, the guide stays on it: the width typed is capped at 100 000.
    expect(text("guide-number")).toBe("19");
    expect(document.querySelector<HTMLInputElement>("#width")?.value).toBe("100000");
  });

  it("[F02.AC4] stays on the rectangle after the steps of the F01 guided test", () => {
    click("guide-previous");

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
    expect(isCornersFieldHidden()).toBe(true);
  });

  it("[F02.AC4] draws a polygon in the same box, keeping width, height and radius", () => {
    type("width", "100");
    type("height", "100");
    type("radius", "0");
    type("corners", "6");
    chooseShape("polygon");

    // Hexagon (6 corners by default) in 100 × 100: flat top and bottom at (100 − 50√3)/2.
    expect(isCornersFieldHidden()).toBe(false);
    expect([valueOf("width"), valueOf("height"), valueOf("radius")]).toEqual(["100", "100", "0"]);

    expect(text("path-data")).toBe(
      "M25 6.69873 L75 6.69873 L100 50 L75 93.30127 L25 93.30127 L0 50 Z",
    );
  });

  it("[F02.AC3] writes the polygon with at most 5 decimals, clockwise from its top-left vertex", () => {
    // Hexagon of the previous step: 6.69873 is (100 − 50√3)/2 = 6.698729… rounded to 5 decimals.
    const numbers = text("path-data").match(/-?\d+(?:\.\d+)?/gu) ?? [];

    expect(numbers.every((number) => (number.split(".", 2)[1] ?? "").length <= 5)).toBe(true);
    expect(text("path-data").startsWith("M25 6.69873 L75 6.69873")).toBe(true);
  });

  it("[F02.AC4] follows the number of corners: a triangle pointing up", () => {
    type("corners", "3");

    expect(text("path-data")).toBe("M50 6.69873 L100 93.30127 L0 93.30127 Z");
  });

  it("[F02.AC2] shows the incircle radius when the radius is too big", () => {
    type("corners", "6");
    type("radius", "1000");

    // Inradius of the hexagon: 50 cos(π/6) = 43.30127.
    expect(text("effective")).toBe("Effective radius: 43.30127 (requested 1000, reduced to fit)");
    // The circle of the hexagon: six arcs of radius 43.30127 and no line, from the top middle.
    expect(text("path-data")).toMatch(/^M50 6\.69873 (?:A43\.30127 43\.30127 0 0 1 [\d. ]+){6}Z$/u);
  });

  it("[F02.AC1] refuses 13, 2, 2.5 and no corners", () => {
    type("corners", "13");

    expect(text("status")).toBe(
      "Width, height and radius must be integers ≥ 0, and corners an integer from 3 to 12.",
    );

    expect(document.querySelector("#corners")?.getAttribute("aria-invalid")).toBe("true");
    expect(text("effective")).toBe("");

    for (const corners of ["2", "2.5", ""]) {
      type("corners", corners);

      expect(text("status")).toContain("corners an integer from 3 to 12");
      expect(document.querySelector("#corners")?.getAttribute("aria-invalid")).toBe("true");
    }
  });

  it("[F02.AC4] goes back to the rectangle, the number of corners hidden and ignored", () => {
    type("radius", "0");
    chooseShape("rectangle");

    expect(isCornersFieldHidden()).toBe(true);
    expect(text("status")).toBe("");
    expect(text("path-data")).toBe("M0 0 L100 0 L100 100 L0 100 Z");
  });

  it("[F02.AC4] sets the shape and the corners of each step of the guided test", () => {
    // Back to step 1 (Previous stays on the first step), then on to step 10, the first of F02.
    clickTimes("guide-previous", GUIDED_STEPS.length);

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
    expect(isCornersFieldHidden()).toBe(true);

    clickTimes("guide-next", 9);

    expect(text("guide-number")).toBe("10");
    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("polygon");
    expect(isCornersFieldHidden()).toBe(false);
    expect(valueOf("corners")).toBe("6");
  });

  it("[F02.AC5] caps a width or a radius above 100 000 at 100 000, in the input too (Q20)", () => {
    chooseShape("rectangle");
    type("width", "250000");

    expect(valueOf("width")).toBe("100000");
    expect(text("model")).toContain('"width": 100000');

    type("height", "250000");

    expect(valueOf("height")).toBe("100000");
    expect(text("model")).toContain('"height": 100000');

    type("radius", "500000");

    expect(valueOf("radius")).toBe("100000");
    expect(text("model")).toContain('"radius": 100000');
  });

  it("[F02.AC5] keeps 100 000 and smaller values exactly", () => {
    type("height", "100000");
    type("width", "99999");

    expect([valueOf("height"), valueOf("width")]).toEqual(["100000", "99999"]);
    expect(text("model")).toContain('"width": 99999');
  });

  it("[F07.AC5] hides the anchor grid for the rectangle, shows it for the polygon on the center", () => {
    chooseShape("rectangle");

    expect(isAnchorFieldHidden()).toBe(true);
    expect(document.querySelectorAll("#canvas path")).toHaveLength(1);

    type("width", "100");
    type("height", "100");
    type("radius", "0");
    type("corners", "3");
    chooseShape("polygon");

    expect(isAnchorFieldHidden()).toBe(false);
    expect(checkedAnchor()).toBe("mid mid");
    expect(text("path-data")).toBe("M50 6.69873 L100 93.30127 L0 93.30127 Z");
  });

  it("[F07.AC5] offers nine cells in one radio group, for the arrow keys", () => {
    // One name: the browser moves the choice with the arrow keys (REF-WAI-APG-RADIO).
    const cells = document.querySelectorAll<HTMLInputElement>('#anchor-grid input[type="radio"]');

    expect(cells).toHaveLength(9);
    expect([...cells].every((cell) => cell.name === "anchor")).toBe(true);
  });

  it("[F07.AC5] draws the box in grey behind the polygon", () => {
    const paths = document.querySelectorAll("#canvas path");

    expect(paths).toHaveLength(2);
    expect(paths[0]?.classList.contains("box")).toBe(true);
    expect(paths[0]?.getAttribute("d")).toBe("M0 0 L100 0 L100 100 L0 100 Z");
  });

  it("[F07.AC2] puts the triangle's base on the bottom of its box", () => {
    chooseAnchor("mid max");

    // Room 100 − 86.60254 = 13.39746, all of it above the triangle.
    expect(checkedAnchor()).toBe("mid max");
    expect(text("path-data")).toBe("M50 13.39746 L100 100 L0 100 Z");
    expect(text("model")).toContain('"vertical": "max"');
  });

  it("[F07.AC3] keeps the triangle in place from left to right, the anchor still shown", () => {
    chooseAnchor("min max");

    expect(text("path-data")).toBe("M50 13.39746 L100 100 L0 100 Z");
    expect(text("model")).toContain('"horizontal": "min"');

    chooseAnchor("max max");

    expect(text("path-data")).toBe("M50 13.39746 L100 100 L0 100 Z");
    expect(text("model")).toContain('"horizontal": "max"');
  });

  it("[F07.AC4] lets the radius round the triangle without moving its base", () => {
    type("radius", "10");

    // The base stays on y = 100 between its two fillets.
    expect(text("path-data")).toMatch(/L[\d.]+ 100 /u);
  });

  it("[F07.AC1] puts the anchor back on the center at each step of the guided test", () => {
    clickTimes("guide-previous", GUIDED_STEPS.length);
    clickTimes("guide-next", 9);

    expect(checkedAnchor()).toBe("mid mid");

    expect(text("path-data")).toBe(
      "M25 6.69873 L75 6.69873 L100 50 L75 93.30127 L25 93.30127 L0 50 Z",
    );
  });
});
