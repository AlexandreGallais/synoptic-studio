// @vitest-environment happy-dom
import { beforeAll, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  openPage,
  readDisabledState,
  readDisplay,
  readText,
  typeAndEnter,
} from "../user-actions";

/**
 * Whether the « Stretched » check box is ticked.
 *
 * @returns its state
 */
function isStretched(): boolean {
  return document.querySelector<HTMLInputElement>("#stretched")?.checked === true;
}

/** Stretched hexagon in 100 × 100: six equal sides of 54.85838 (`DERIV-stretched-polygon`, checks). */
const STRETCHED_HEXAGON = "M77.42919 100 L22.57081 100 L0 50 L22.57081 0 L77.42919 0 L100 50 Z";

describe("playground: stretched polygon (US-021)", () => {
  beforeAll(() => {
    // The page as served, without its script tag: the test mounts the playground itself.
    openPage("");
  });

  it("[F06.AC5] shows the « Stretched » check box for the polygon only, unticked", () => {
    expect(readDisplay("stretched-field") === "none").toBe(true);

    typeAndEnter("width", "100");
    typeAndEnter("height", "100");
    typeAndEnter("radius", "0");
    typeAndEnter("corners", "6");
    chooseOption("shape", "polygon");

    expect(readDisplay("stretched-field") === "none").toBe(false);
    expect(isStretched()).toBe(false);

    // The regular hexagon of F02, centered: flat top and bottom at (100 − 50√3)/2.
    expect(readText("#path-data")).toBe(
      "M75 93.30127 L25 93.30127 L0 50 L25 6.69873 L75 6.69873 L100 50 Z",
    );
  });

  it("[F06.AC1] [F06.AC5] fills the box when ticked, the anchor grid disabled", () => {
    clickElement("stretched");

    expect(isStretched()).toBe(true);
    expect(readText("#path-data")).toBe(STRETCHED_HEXAGON);
    expect(readDisabledState("anchor-field") === "true").toBe(true);
  });

  it("[F06.AC3] stays stretched when the number of corners and the size change", () => {
    typeAndEnter("corners", "5");
    typeAndEnter("width", "300");
    typeAndEnter("height", "50");

    // DERIV-stretched-polygon, checks: the pentagon in 300 × 50, stopped at its corner guard.
    expect(readText("#path-data")).toBe(
      "M228.85967 50 L71.14033 50 L0 12.8595 L150 0 L300 12.8595 Z",
    );
  });

  it("[F06.AC5] says the effective radii differ when its corners are clamped unequally", () => {
    typeAndEnter("radius", "1000");

    expect(readText("#effective")).toBe(
      "Effective radius: mixed, shown for each point in Points mode.",
    );
  });

  it("[F06.AC1] goes back to the regular polygon, placed by its anchor, when unticked", () => {
    typeAndEnter("radius", "0");
    typeAndEnter("corners", "6");
    typeAndEnter("width", "100");
    typeAndEnter("height", "100");
    clickElement("stretched");

    expect(isStretched()).toBe(false);
    expect(readDisabledState("anchor-field") === "true").toBe(false);

    expect(readText("#path-data")).toBe(
      "M75 93.30127 L25 93.30127 L0 50 L25 6.69873 L75 6.69873 L100 50 Z",
    );
  });

  it("[F06.AC4] converts a stretched polygon to a free shape without moving it", () => {
    clickElement("stretched");
    clickElement("mode-points");
    clickElement("convert");

    expect(readText("#path-data")).toBe(STRETCHED_HEXAGON);
  });

  it("ticks and unticks safely on a refused polygon, then draws again once it is valid", () => {
    chooseOption("shape", "polygon");
    // A radius emptied by a script, which the field never read: the polygon refused.
    typeAndEnter("radius", "");
    clickElement("stretched");
    clickElement("stretched");

    expect(readText("#status")).not.toBe("");
    expect(readText("#effective")).toBe("");

    typeAndEnter("radius", "0");

    expect(readText("#status")).toBe("");
    expect(readText("#effective")).not.toBe("");
  });

  it("unticks the check box when a step of the guided test draws a polygon of F02", () => {
    chooseOption("shape", "polygon");
    clickElement(isStretched() ? "shape" : "stretched");

    expect(isStretched()).toBe(true);

    clickElement("guide-next");

    expect(isStretched()).toBe(false);
  });
});
