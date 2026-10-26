// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseShapeKind,
  clickAt,
  clickElement,
  clickWithKeys,
  commitField,
  openPage,
  readDisplay,
  readPressedState,
  readText,
  readValue,
  selectInput,
  typeAndEnter,
} from "../../../../../testing";

describe("toolbar", () => {
  beforeEach(() => {
    // The page as it opens, a rectangle.
    openPage("");
  });

  it("starts with the selector armed, a rectangle edited and the polygon's options hidden", () => {
    expect(readPressedState("tool-selector") === "true").toBe(true);
    expect(readPressedState("tool-rectangle") === "true").toBe(false);
    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readDisplay("polygon-options") === "none").toBe(true);
  });

  it("[F35.AC1] draws a polygon placed by its tool, its options shown, the selector armed again", () => {
    clickElement("tool-polygon");

    expect(readPressedState("tool-polygon") === "true").toBe(true);
    expect(readPressedState("tool-selector") === "true").toBe(false);

    clickAt("#canvas", { clientX: 290, clientY: 180 });

    expect(readPressedState("tool-polygon") === "true").toBe(false);
    expect(readPressedState("tool-selector") === "true").toBe(true);
    expect(readText("#shape-title")).toBe("Polygon");
    expect(readDisplay("polygon-options") === "none").toBe(false);
    // A hexagon, the default number of corners, has six vertices in its contour.
    expect(readText("#contour").match(/"x"/gu)).toHaveLength(6);
  });
});

describe("toolbar after a free shape", () => {
  beforeEach(() => {
    // The page as it opens, radius 10.
    openPage("");
    // A pentagon 120 × 80 converted to a free shape: its width becomes 84.11698.
    chooseShapeKind("polygon");

    typeAndEnter("corners", "5");

    clickWithKeys("#mode-points", {});
    clickWithKeys("#convert", {});
    clickWithKeys('#points [data-vertex="1"]', {});

    // The value is set, then committed alone: no input event (Enter after a paste).
    selectInput("point-x").value = "30";
    commitField("point-x");
  });

  it("[F35.AC1] places a valid rectangle after it, the free shape kept below as it was", () => {
    expect(readValue("width")).toBe("84.11698");

    const free = readText("#path-data");

    clickWithKeys("#tool-rectangle", {});
    clickAt("#canvas", { clientX: 290, clientY: 180 });

    // A rectangle 100 × 100 with the page's radius, 10: no rectangle was placed before it.
    expect([readValue("width"), readValue("height"), readValue("radius")]).toEqual([
      "100",
      "100",
      "10",
    ]);

    expect(readText("#shape-title")).toBe("Rectangle");
    expect(document.querySelectorAll('[aria-invalid="true"]')).toHaveLength(0);

    expect(
      [...document.querySelectorAll("#canvas svg > path:not([class])")].map((path) =>
        path.getAttribute("d"),
      ),
    ).toEqual([free, readText("#path-data")]);
  });

  it("goes back to the shape the page opens with on Reset, in Shape mode, the selector armed", () => {
    clickWithKeys("#tool-polygon", {});
    clickWithKeys("#tool-reset", {});

    expect([readValue("width"), readValue("height"), readValue("radius")]).toEqual([
      "120",
      "80",
      "10",
    ]);

    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readPressedState("tool-selector") === "true").toBe(true);
    expect(readPressedState("mode-shape") === "true").toBe(true);
    expect(document.querySelectorAll('[aria-invalid="true"]')).toHaveLength(0);
  });
});

describe("pipeline column", () => {
  it("is shown, then hidden and shown again by its button", () => {
    openPage("");

    expect(readDisplay("output") === "none").toBe(false);

    clickWithKeys("#tool-output", {});

    expect([readDisplay("output") === "none", readPressedState("tool-output") === "true"]).toEqual([
      true,
      false,
    ]);

    clickWithKeys("#tool-output", {});

    expect([readDisplay("output") === "none", readPressedState("tool-output") === "true"]).toEqual([
      false,
      true,
    ]);
  });
});
