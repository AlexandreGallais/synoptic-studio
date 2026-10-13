// @vitest-environment happy-dom
import { beforeAll, describe, expect, it } from "vitest";

import { GUIDED_STEPS } from "./guided-steps";
import {
  chooseOption,
  clickElement,
  openPage,
  readText,
  readValue,
  typeValue,
} from "./user-actions";

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
 * Clicks a button of the page several times.
 *
 * @param id - button id
 * @param count - number of clicks, >= 0
 */
function clickTimes(id: string, count: number): void {
  if (!(count > 0)) {
    return;
  }

  clickElement(id);
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
    openPage("");
  });

  it("[F02.AC4] opens on the rectangle, the selector on Rectangle, without the number of corners", () => {
    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
    expect(isCornersFieldHidden()).toBe(true);
  });

  it("[F01.AC1] renders the rectangle as one <path>", () => {
    typeValue("width", "100");
    typeValue("height", "50");
    typeValue("radius", "0");

    expect(document.querySelectorAll("#canvas svg path")).toHaveLength(1);
    expect(readText("#path-data")).toBe("M0 0 L100 0 L100 50 L0 50 Z");
  });

  it("[F01.AC2] shows the effective radius next to the requested one when it was reduced", () => {
    typeValue("height", "25");
    typeValue("radius", "100");

    expect(readText("#effective")).toBe("Effective radius: 12.5 (requested 100, reduced to fit)");
  });

  it("[F01.AC2] says the radius is as requested when it fits", () => {
    typeValue("height", "50");
    typeValue("radius", "10");

    expect(readText("#effective")).toBe("Effective radius: 10 (as requested)");
    expect(readText("#path-data")).toContain("A10 10 0 0 1 100 10");
  });

  it("refuses a negative radius and clears the effective radius", () => {
    typeValue("radius", "-3");

    expect(readText("#status")).toBe(
      "Width, height and radius must be integers ≥ 0, and the angle an integer.",
    );

    expect(readText("#effective")).toBe("");
  });

  it("[F02.AC1] walks the steps of the guided test (F01, F02, F07, F03, F09, F08, F04, F06, then E01), each showing what it tells to look at", () => {
    // "Previous" on the first step stays there and types its values again.
    clickElement("guide-previous");

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
      // F07 (US-010 card).
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      // F03 (US-012 and US-013 cards): each step starts afresh, without rounding.
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      // F09 (US-017 card): at width or height 0 no corner can be rounded, so the radius stays
      // shown as requested (Q17).
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      // F08 (US-014, US-015 and US-016 cards): the sharp hexagon, then the rectangle rounded by 10;
      // each step types its shape afresh, the clicks being the Product Owner's.
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 10 (as requested)",
      "Effective radius: 0 (as requested)",
      // F04 (US-018 and US-019 cards): each step starts straight, sharp; the angles are typed by
      // the Product Owner.
      ...Array.from({ length: 11 }, () => "Effective radius: 0 (as requested)"),
      // F06 (US-021 card): sharp polygons, but the pentagon rounded by 1000, its corners clamped
      // differently.
      ...Array.from({ length: 3 }, () => "Effective radius: 0 (as requested)"),
      "Effective radius: mixed, shown for each point in Points mode.",
      ...Array.from({ length: 5 }, () => "Effective radius: 0 (as requested)"),
      // The user test of E01 (REV-001), before the clicks and values the Product Owner adds.
      "Effective radius: 12 (as requested)",
      "Effective radius: 20 (requested 1000, reduced to fit)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 0 (as requested)",
      "Effective radius: 50 (requested 1000, reduced to fit)",
    ];

    for (const [index, effective] of expected.entries()) {
      expect(readText("#guide-number")).toBe(String(index + 1));
      expect(readText("#effective")).toBe(effective);
      clickElement("guide-next");
    }

    // Past the last step, the guide stays on it: the stretched hexagon of E01, rounded, straight.
    expect(readText("#guide-number")).toBe("72");

    expect(readText("#path-data")).toBe(
      "M92.04396 100 L67.95604 100 A50 50 0 0 1 31.16755 83.86159 A50 50 0 0 1 31.16755 16.13841 A50 50 0 0 1 67.95604 0 L92.04396 0 A50 50 0 0 1 128.83245 16.13841 A50 50 0 0 1 128.83245 83.86159 A50 50 0 0 1 92.04396 100 Z",
    );

    expect(document.querySelector<HTMLInputElement>("#angle")?.value).toBe("0");

    // The last step of F06: the stretched hexagon, straight.
    clickTimes("guide-previous", 6);

    expect(readText("#guide-number")).toBe("66");

    expect(readText("#path-data")).toBe(
      "M77.42919 100 L22.57081 100 L0 50 L22.57081 0 L77.42919 0 L100 50 Z",
    );

    // The last step of F09: the 120 × 80 rectangle, a free shape.
    clickTimes("guide-previous", 28);

    expect(readText("#guide-number")).toBe("38");

    expect(readText("#path-data")).toBe(
      "M10 0 L110 0 A10 10 0 0 1 120 10 L120 70 A10 10 0 0 1 110 80 L10 80 A10 10 0 0 1 0 70 L0 10 A10 10 0 0 1 10 0 Z",
    );

    // The last step of F03: a rectangle 0 wide, not drawn.
    clickTimes("guide-previous", 5);

    expect(readText("#guide-number")).toBe("33");
    expect(readText("#path-data")).toBe("M0 0 L0 50 Z");

    // The last step of F07: back to the rectangle, the anchor grid hidden.
    clickTimes("guide-previous", 5);

    expect(readText("#guide-number")).toBe("28");

    expect(readText("#path-data")).toBe(
      "M10 0 L90 0 A10 10 0 0 1 100 10 L100 40 A10 10 0 0 1 90 50 L10 50 A10 10 0 0 1 0 40 L0 10 A10 10 0 0 1 10 0 Z",
    );

    expect(document.querySelector("#anchor-field")?.hasAttribute("hidden")).toBe(true);

    // The next tests start from the last step of F02: a rectangle, the anchor on the center.
    clickTimes("guide-previous", 9);

    expect(readText("#guide-number")).toBe("19");
    expect(checkedAnchor()).toBe("mid mid");
  });

  it("[F02.AC4] stays on the rectangle after the steps of the F01 guided test", () => {
    clickElement("guide-previous");

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
    expect(isCornersFieldHidden()).toBe(true);
  });

  it("[F02.AC4] draws a polygon in the same box, keeping width, height and radius", () => {
    typeValue("width", "100");
    typeValue("height", "100");
    typeValue("radius", "0");
    typeValue("corners", "6");
    chooseOption("shape", "polygon");

    // Hexagon (6 corners by default) in 100 × 100: flat top and bottom at (100 − 50√3)/2.
    expect(isCornersFieldHidden()).toBe(false);

    expect([readValue("width"), readValue("height"), readValue("radius")]).toEqual([
      "100",
      "100",
      "0",
    ]);

    expect(readText("#path-data")).toBe(
      "M75 93.30127 L25 93.30127 L0 50 L25 6.69873 L75 6.69873 L100 50 Z",
    );
  });

  it("[F02.AC3] writes the polygon with at most 5 decimals, clockwise from the right end of its base", () => {
    // Hexagon of the previous step: 6.69873 is (100 − 50√3)/2 = 6.698729… rounded to 5 decimals.
    const numbers = readText("#path-data").match(/-?\d+(?:\.\d+)?/gu) ?? [];

    expect(numbers.every((number) => (number.split(".", 2)[1] ?? "").length <= 5)).toBe(true);
    expect(readText("#path-data").startsWith("M75 93.30127 L25 93.30127")).toBe(true);
  });

  it("[F02.AC4] follows the number of corners: a triangle pointing up", () => {
    typeValue("corners", "3");

    expect(readText("#path-data")).toBe("M100 93.30127 L0 93.30127 L50 6.69873 Z");
  });

  it("[F02.AC2] shows the incircle radius when the radius is too big", () => {
    typeValue("corners", "6");
    typeValue("radius", "1000");

    // Inradius of the hexagon: 50 cos(π/6) = 43.30127.
    expect(readText("#effective")).toBe(
      "Effective radius: 43.30127 (requested 1000, reduced to fit)",
    );

    // The circle of the hexagon: six arcs of radius 43.30127 and no line, from the middle of the
    // base, where the first fillet ends.
    expect(readText("#path-data")).toMatch(
      /^M50 93\.30127 (?:A43\.30127 43\.30127 0 0 1 [\d. ]+){6}Z$/u,
    );
  });

  it("[F02.AC1] refuses 13, 2, 2.5 and no corners", () => {
    typeValue("corners", "13");

    expect(readText("#status")).toBe(
      "Width, height and radius must be integers ≥ 0, corners an integer from 3 to 12, and the angle an integer.",
    );

    expect(document.querySelector("#corners")?.getAttribute("aria-invalid")).toBe("true");
    expect(readText("#effective")).toBe("");

    for (const corners of ["2", "2.5", ""]) {
      typeValue("corners", corners);

      expect(readText("#status")).toContain("corners an integer from 3 to 12");
      expect(document.querySelector("#corners")?.getAttribute("aria-invalid")).toBe("true");
    }
  });

  it("[F02.AC4] goes back to the rectangle, the number of corners hidden and ignored", () => {
    typeValue("radius", "0");
    chooseOption("shape", "rectangle");

    expect(isCornersFieldHidden()).toBe(true);
    expect(readText("#status")).toBe("");
    expect(readText("#path-data")).toBe("M0 0 L100 0 L100 100 L0 100 Z");
  });

  it("[F02.AC4] sets the shape and the corners of each step of the guided test", () => {
    // Back to step 1 (Previous stays on the first step), then on to step 10, the first of F02.
    clickTimes("guide-previous", GUIDED_STEPS.length);

    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("rectangle");
    expect(isCornersFieldHidden()).toBe(true);

    clickTimes("guide-next", 9);

    expect(readText("#guide-number")).toBe("10");
    expect(document.querySelector<HTMLSelectElement>("#shape")?.value).toBe("polygon");
    expect(isCornersFieldHidden()).toBe(false);
    expect(readValue("corners")).toBe("6");
  });

  it("[F02.AC5] caps a width or a radius above 100 000 at 100 000, in the input too (Q20)", () => {
    chooseOption("shape", "rectangle");
    typeValue("width", "250000");

    expect(readValue("width")).toBe("100000");
    expect(readText("#model")).toContain('"width": 100000');

    typeValue("height", "250000");

    expect(readValue("height")).toBe("100000");
    expect(readText("#model")).toContain('"height": 100000');

    typeValue("radius", "500000");

    expect(readValue("radius")).toBe("100000");
    expect(readText("#model")).toContain('"radius": 100000');
  });

  it("[F02.AC5] keeps 100 000 and smaller values exactly", () => {
    typeValue("height", "100000");
    typeValue("width", "99999");

    expect([readValue("height"), readValue("width")]).toEqual(["100000", "99999"]);
    expect(readText("#model")).toContain('"width": 99999');
  });

  it("[F07.AC5] hides the anchor grid for the rectangle, shows it for the polygon on the center", () => {
    chooseOption("shape", "rectangle");

    expect(isAnchorFieldHidden()).toBe(true);
    expect(document.querySelectorAll("#canvas path")).toHaveLength(1);

    typeValue("width", "100");
    typeValue("height", "100");
    typeValue("radius", "0");
    typeValue("corners", "3");
    chooseOption("shape", "polygon");

    expect(isAnchorFieldHidden()).toBe(false);
    expect(checkedAnchor()).toBe("mid mid");
    expect(readText("#path-data")).toBe("M100 93.30127 L0 93.30127 L50 6.69873 Z");
  });

  it("[F07.AC5] offers nine cells in one radio group, for the arrow keys", () => {
    // One name: the browser moves the choice with the arrow keys (REF-WAI-APG-RADIO).
    const cells = document.querySelectorAll<HTMLInputElement>('#anchor-grid input[type="radio"]');

    expect(cells).toHaveLength(9);
    expect([...cells].every((cell) => cell.name === "anchor")).toBe(true);

    // Reading order, rows top to bottom: the arrow keys follow it.
    expect([...cells].map((cell) => cell.value)).toEqual([
      "min min",
      "mid min",
      "max min",
      "min mid",
      "mid mid",
      "max mid",
      "min max",
      "mid max",
      "max max",
    ]);

    expect(cells[0]?.getAttribute("aria-label")).toBe("Top left");
    expect(cells[8]?.getAttribute("aria-label")).toBe("Bottom right");
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
    expect(readText("#path-data")).toBe("M100 100 L0 100 L50 13.39746 Z");
    expect(readText("#model")).toContain('"vertical": "max"');
  });

  it("[F07.AC3] keeps the triangle in place from left to right, the anchor still shown", () => {
    chooseAnchor("min max");

    expect(readText("#path-data")).toBe("M100 100 L0 100 L50 13.39746 Z");
    expect(readText("#model")).toContain('"horizontal": "min"');

    chooseAnchor("max max");

    expect(readText("#path-data")).toBe("M100 100 L0 100 L50 13.39746 Z");
    expect(readText("#model")).toContain('"horizontal": "max"');
  });

  it("[F07.AC4] lets the radius round the triangle without moving its base", () => {
    typeValue("radius", "10");

    // The base stays on y = 100 between its fillets' tangent points, r / tan(30°) = 17.32051
    // from each base vertex (it would rise if the rounded shape were anchored instead).
    expect(readText("#path-data")).toContain("L17.32051 100 ");
    expect(readText("#path-data")).toContain("82.67949 100");
    expect(readText("#path-data")).not.toBe("M100 100 L0 100 L50 13.39746 Z");
  });

  it("[F07.AC1] puts the anchor back on the center at each step of the guided test", () => {
    clickTimes("guide-previous", GUIDED_STEPS.length);
    clickTimes("guide-next", 9);

    expect(checkedAnchor()).toBe("mid mid");

    expect(readText("#path-data")).toBe(
      "M75 93.30127 L25 93.30127 L0 50 L25 6.69873 L75 6.69873 L100 50 Z",
    );
  });

  it("[F07.AC2] places the polygon of each F07 step of the guided test where the step says", () => {
    // Steps 20 to 28 are F07's; each path is the one its "Look at" announces.
    clickTimes("guide-previous", GUIDED_STEPS.length);
    clickTimes("guide-next", 20);

    expect(readText("#path-data")).toBe("M100 100 L0 100 L50 13.39746 Z");

    clickTimes("guide-next", 2);

    expect(readText("#path-data")).toBe("M100 86.60254 L0 86.60254 L50 0 Z");

    // The square of 50 in a 100 × 50 box: right (50 to 100), then left (0 to 50) by a corner button;
    // its path starts where the fillet of point 1, the right end of the base, ends on the base.
    clickTimes("guide-next", 2);

    expect(checkedAnchor()).toBe("max mid");
    expect(readText("#path-data")).toContain("M90 50 L60 50 ");

    clickElement("guide-next");

    expect(checkedAnchor()).toBe("min max");
    expect(readText("#path-data")).toContain("M40 50 L10 50 ");
  });
});
