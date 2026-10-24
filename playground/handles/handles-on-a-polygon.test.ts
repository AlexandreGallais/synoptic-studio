// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openOnStep,
  readPointRows,
  readPressedState,
  readText,
  readValue,
  typeAndEnter,
} from "../user-actions";

describe("handles on a point of a regular polygon (US-028)", () => {
  it("[F12.AC8] converts the hexagon first, unmoved, then switches its point to handles", () => {
    // A hexagon 100 × 100, radius 10; point 1 selected, then Handles.
    openOnStep(1);
    typeAndEnter("width", "100");
    typeAndEnter("height", "100");
    typeAndEnter("corners", "6");
    chooseOption("shape", "polygon");
    typeAndEnter("radius", "10");
    clickElement("mode-points");
    clickPoint(1, false);

    const drawn = readText("#path-data");

    clickElement("handles-switch");

    expect(readValue("shape")).toBe("free");
    expect(readPressedState("handles-switch")).toBe("true");
    expect(readPointRows()).toHaveLength(6);
    expect(readPointRows()[0]).toMatch(/radius 10 \(effective 0\)$/u);
    expect(readValue("handle-mode")).toBe("angle-and-length");

    // Handles (0, 0), shown on the page at the point itself.
    expect(
      ["before-x", "before-y", "after-x", "after-y"].map((field) => readValue(`handle-${field}`)),
    ).toEqual(["75", "93.30127", "75", "93.30127"]);

    // Unmoved, unrenumbered: the path before, only the arc at point 1 gone.
    expect(drawn).toMatch(/^M69\.2265 93\.30127 /u);

    expect(readText("#path-data")).toBe(
      "M75 93.30127 L30.7735 93.30127 A10 10 0 0 1 22.11325 88.30127 L2.88675 55 A10 10 0 0 1 2.88675 45 L22.11325 11.69873 A10 10 0 0 1 30.7735 6.69873 L69.2265 6.69873 A10 10 0 0 1 77.88675 11.69873 L97.11325 45 A10 10 0 0 1 97.11325 55 Z",
    );

    typeAndEnter("handle-after-x", "90");

    expect(readText("#path-data")).toContain("C");
  });

  it("hides the switch on a polygon that cannot be converted, of width 0", () => {
    openOnStep(1);
    typeAndEnter("width", "0");
    typeAndEnter("corners", "6");
    chooseOption("shape", "polygon");
    clickElement("mode-points");
    clickPoint(1, false);

    expect(document.querySelector<HTMLElement>("#handles-switch")?.hidden).toBe(true);
  });
});
