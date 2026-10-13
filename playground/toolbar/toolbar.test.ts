// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { CURRENT_GUIDED_STEPS, GUIDED_STEPS } from "../guided-steps";
import {
  clickElement,
  clickWithKeys,
  commitField,
  openServedPage,
  readDisplay,
  readPressedState,
  readText,
  readValue,
  selectInput,
  typeValue,
} from "../user-actions";

describe("toolbar", () => {
  beforeEach(() => {
    openServedPage("");
  });

  it("starts on the rectangle, its button pressed and the polygon's options hidden", () => {
    expect(readPressedState("tool-rectangle") === "true").toBe(true);
    expect(readPressedState("tool-polygon") === "true").toBe(false);
    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readDisplay("polygon-options") === "none").toBe(true);
  });

  it("draws a polygon from its button, its options shown", () => {
    clickElement("tool-polygon");

    expect(readPressedState("tool-polygon") === "true").toBe(true);
    expect(readPressedState("tool-rectangle") === "true").toBe(false);
    expect(readText("#shape-title")).toBe("Polygon");
    expect(readDisplay("polygon-options") === "none").toBe(false);
    // A hexagon, the default number of corners, has six vertices in its contour.
    expect(readText("#contour").match(/"x"/gu)).toHaveLength(6);
  });
});

describe("toolbar after a free shape", () => {
  beforeEach(() => {
    openServedPage("");
    // A pentagon 120 × 80 converted to a free shape: its width becomes 84.11698.
    clickWithKeys("#tool-polygon", {});

    typeValue("corners", "5");

    clickWithKeys("#mode-points", {});
    clickWithKeys("#convert", {});
    clickWithKeys('#points [data-vertex="1"]', {});

    // The value is set, then committed alone: no input event (Enter after a paste).
    selectInput("point-x").value = "30";
    commitField("point-x");
  });

  it("draws a valid rectangle, its width rounded and one radius for every corner", () => {
    expect(readValue("width")).toBe("84.11698");

    clickWithKeys("#tool-rectangle", {});

    expect(readValue("width")).toBe("84");
    // The radius field was empty (Mixed): the first radius kept, 10, goes to every corner.
    expect(readValue("radius")).toBe("10");
    expect(document.querySelectorAll('[aria-invalid="true"]')).toHaveLength(0);
  });

  it("goes back to the shape the page opens with on Reset, in Shape mode", () => {
    clickWithKeys("#tool-reset", {});

    expect([readValue("width"), readValue("height"), readValue("radius")]).toEqual([
      "120",
      "80",
      "10",
    ]);

    expect(readPressedState("tool-rectangle") === "true").toBe(true);
    expect(readPressedState("mode-shape") === "true").toBe(true);
    expect(document.querySelectorAll('[aria-invalid="true"]')).toHaveLength(0);
  });
});

describe("pipeline column", () => {
  it("is shown, then hidden and shown again by its button", () => {
    openServedPage("");

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

describe("guided test of the page", () => {
  it("is hidden when the feature being validated has no steps", () => {
    openServedPage("");

    expect(CURRENT_GUIDED_STEPS).toHaveLength(0);
    expect(readDisplay("guide") === "none").toBe(true);
  });

  it("shows every step of the epic at ?guide=epic", () => {
    openServedPage("?guide=epic");

    expect(readDisplay("guide") === "none").toBe(false);
    expect(readText("#guide-count")).toBe(String(GUIDED_STEPS.length));
  });
});
