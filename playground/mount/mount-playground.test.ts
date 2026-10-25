// @vitest-environment happy-dom
import { beforeAll, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseRadio,
  clickElement,
  openPage,
  readCheckedRadio,
  readDisplay,
  readText,
  readValue,
  typeAndEnter,
} from "../user-actions";

describe("playground", () => {
  beforeAll(() => {
    // The page as served, without its script tag: the test mounts the playground itself.
    openPage("");
  });

  it("[F02.AC4] opens on the rectangle, the selector on Rectangle, without the number of corners", () => {
    expect(readValue("shape")).toBe("rectangle");
    expect(readDisplay("corners-field") === "none").toBe(true);
  });

  it("[F01.AC1] renders the rectangle as one <path>", () => {
    typeAndEnter("width", "100");
    typeAndEnter("height", "50");
    typeAndEnter("radius", "0");

    expect(document.querySelectorAll("#canvas svg path")).toHaveLength(1);
    expect(readText("#path-data")).toBe("M0 0 L100 0 L100 50 L0 50 Z");
  });

  it("[F01.AC2] shows the effective radius next to the requested one when it was reduced", () => {
    typeAndEnter("height", "25");
    typeAndEnter("radius", "100");

    expect(readText("#effective")).toBe("Effective radius: 12.5 (requested 100, reduced to fit)");
  });

  it("[F01.AC2] says the radius is as requested when it fits", () => {
    typeAndEnter("height", "50");
    typeAndEnter("radius", "10");

    expect(readText("#effective")).toBe("Effective radius: 10 (as requested)");
    expect(readText("#path-data")).toContain("A10 10 0 0 1 100 10");
  });

  it("refuses a radius emptied by a script and clears the effective radius", () => {
    // A field emptied with no focus nor key, the page never reading its text: no radius.
    typeAndEnter("radius", "");

    expect(readText("#status")).toBe(
      "Width, height and radius must be integers ≥ 0, and the angle an integer.",
    );

    expect(readText("#effective")).toBe("");
  });

  it("[F02.AC4] draws a polygon in the same box, keeping width, height and radius", () => {
    typeAndEnter("width", "100");
    typeAndEnter("height", "100");
    typeAndEnter("radius", "0");
    typeAndEnter("corners", "6");
    chooseOption("shape", "polygon");

    // Hexagon (6 corners by default) in 100 × 100: flat top and bottom at (100 − 50√3)/2.
    expect(readDisplay("corners-field") === "none").toBe(false);

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
    typeAndEnter("corners", "3");

    expect(readText("#path-data")).toBe("M100 93.30127 L0 93.30127 L50 6.69873 Z");
  });

  it("[F02.AC2] shows the incircle radius when the radius is too big", () => {
    typeAndEnter("corners", "6");
    typeAndEnter("radius", "1000");

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

  it.each([
    // 13 taken at 12; 2 at 3; 2.5 rounded to 3, a tie up.
    { expected: "12", typed: "13" },
    { expected: "3", typed: "2" },
    { expected: "3", typed: "2.5" },
  ])("[F44.AC4] takes $typed corners as $expected, from 3 to 12", ({ expected, typed }) => {
    typeAndEnter("corners", typed);

    expect([readValue("corners"), readText("#status")]).toEqual([expected, ""]);
  });

  it("[F02.AC1] refuses no corners, emptied by a script", () => {
    typeAndEnter("corners", "");

    expect(readText("#status")).toBe(
      "Width, height and radius must be integers ≥ 0, corners an integer from 3 to 12, and the angle an integer.",
    );

    expect(document.querySelector("#corners")?.getAttribute("aria-invalid")).toBe("true");
    expect(readText("#effective")).toBe("");
  });

  it("[F02.AC4] goes back to the rectangle, the number of corners hidden and ignored", () => {
    typeAndEnter("radius", "0");
    chooseOption("shape", "rectangle");

    expect(readDisplay("corners-field") === "none").toBe(true);
    expect(readText("#status")).toBe("");
    expect(readText("#path-data")).toBe("M0 0 L100 0 L100 100 L0 100 Z");
  });

  it("[F02.AC5] caps a width or a radius above 100 000 at 100 000, in the input too (Q20)", () => {
    chooseOption("shape", "rectangle");
    typeAndEnter("width", "250000");

    expect(readValue("width")).toBe("100000");
    expect(readText("#model")).toContain('"width": 100000');

    typeAndEnter("height", "250000");

    expect(readValue("height")).toBe("100000");
    expect(readText("#model")).toContain('"height": 100000');

    typeAndEnter("radius", "500000");

    expect(readValue("radius")).toBe("100000");
    expect(readText("#model")).toContain('"radius": 100000');
  });

  it(
    "[F02.AC5] [F39.AC1] keeps 100 000 and smaller values exactly",
    // The test before leaves a radius of 100 000: the rectangle 99 999 × 100 000 would then
    // have corners of radius 49 999.5, its band following each by about 39 270 chords of at
    // most 2 screen pixels (Q48), 1 063 ms alone and over Vitest's 5 s under the pre-push's
    // load, unless the band is computed only within the helps' box, where it is empty. With
    // the radius typed 0 first (note 0029 §6), alone: 10 ms, the median of 5 runs of its
    // file; 1 s, 100 times it, fails the 1 063 ms of the band followed.
    { tags: ["timing"], timeout: 1000 },
    () => {
      typeAndEnter("radius", "0");
      typeAndEnter("height", "100000");
      typeAndEnter("width", "99999");

      expect([readValue("height"), readValue("width")]).toEqual(["100000", "99999"]);
      expect(readText("#model")).toContain('"width": 99999');
    },
  );

  it("[F07.AC5] hides the anchor grid for the rectangle, shows it for the polygon on the center", () => {
    chooseOption("shape", "rectangle");

    expect(readDisplay("anchor-field") === "none").toBe(true);
    expect(document.querySelectorAll("#canvas path")).toHaveLength(1);

    typeAndEnter("width", "100");
    typeAndEnter("height", "100");
    typeAndEnter("radius", "0");
    typeAndEnter("corners", "3");
    chooseOption("shape", "polygon");

    expect(readDisplay("anchor-field") === "none").toBe(false);
    expect(readCheckedRadio("anchor")).toBe("mid mid");
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
    chooseRadio("anchor", "mid max");

    // Room 100 − 86.60254 = 13.39746, all of it above the triangle.
    expect(readCheckedRadio("anchor")).toBe("mid max");
    expect(readText("#path-data")).toBe("M100 100 L0 100 L50 13.39746 Z");
    expect(readText("#model")).toContain('"vertical": "max"');
  });

  it("[F07.AC3] keeps the triangle in place from left to right, the anchor still shown", () => {
    chooseRadio("anchor", "min max");

    expect(readText("#path-data")).toBe("M100 100 L0 100 L50 13.39746 Z");
    expect(readText("#model")).toContain('"horizontal": "min"');

    chooseRadio("anchor", "max max");

    expect(readText("#path-data")).toBe("M100 100 L0 100 L50 13.39746 Z");
    expect(readText("#model")).toContain('"horizontal": "max"');
  });

  it("[F07.AC4] lets the radius round the triangle without moving its base", () => {
    typeAndEnter("radius", "10");

    // The base stays on y = 100 between its fillets' tangent points, r / tan(30°) = 17.32051
    // from each base vertex (it would rise if the rounded shape were anchored instead).
    expect(readText("#path-data")).toContain("L17.32051 100 ");
    expect(readText("#path-data")).toContain("82.67949 100");
    expect(readText("#path-data")).not.toBe("M100 100 L0 100 L50 13.39746 Z");
  });

  it("[F07.AC1] puts the anchor back on the center on Reset", () => {
    chooseOption("shape", "polygon");
    chooseRadio("anchor", "min max");
    clickElement("tool-reset");

    expect(readCheckedRadio("anchor")).toBe("mid mid");
  });
});
