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
  typeValue,
} from "../user-actions";

describe("handles on a point of a regular polygon (US-028)", () => {
  it("[F12.AC8] converts the hexagon first, unmoved, then switches its point to handles", () => {
    // A hexagon 100 × 100, radius 10; point 1 selected, then Handles.
    openOnStep(1);
    typeValue("width", "100");
    typeValue("height", "100");
    typeValue("corners", "6");
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
    // Point 1 sharp now: only its corner changes, the others keep their arcs.
    expect(readText("#path-data").match(/A10 10 /gu)).toHaveLength(5);
    expect(drawn.match(/A10 10 /gu)).toHaveLength(6);

    typeAndEnter("handle-after-x", "90");

    expect(readText("#path-data")).toContain("C");
  });
});
