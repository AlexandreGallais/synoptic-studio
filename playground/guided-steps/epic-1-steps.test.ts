// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  clickElement,
  clickPoint,
  openOnStep,
  readText,
  readValue,
  typeAndEnter,
} from "../user-actions";

describe("playground: the user test of E01 (REV-001)", () => {
  it("[F10.AC6] [F13.AC3] step 67: the rounded body widens, its radius kept", () => {
    openOnStep(67);
    typeAndEnter("width", "200");

    expect(readText("#path-data")).toBe(
      "M12 0 L188 0 A12 12 0 0 1 200 12 L200 88 A12 12 0 0 1 188 100 L12 100 A12 12 0 0 1 0 88 L0 12 A12 12 0 0 1 12 0 Z",
    );

    expect(readText("#effective")).toBe("Effective radius: 12 (as requested)");
  });

  it("step 68: the capsule turned by 90° keeps its width and height", () => {
    openOnStep(68);
    typeAndEnter("angle", "90");

    expect(readText("#effective")).toBe("Effective radius: 20 (requested 1000, reduced to fit)");
    expect([readValue("width"), readValue("height")]).toStrictEqual(["160", "40"]);
    expect(readText("#straight-box")).toBe("Straight box: x 60, y -60, width 40, height 160");
  });

  it("step 69: only the base corners of the anchored triangle are rounded, its base kept", () => {
    openOnStep(69);
    clickElement("mode-points");
    clickPoint(1, false);
    clickPoint(2, true);
    typeAndEnter("point-radius", "15");

    // The base from 25.98 to 94.02 at y 120: a flat face keeps touching its anchored side.
    expect(readText("#path-data")).toBe(
      "M94.01924 120 L25.98076 120 A15 15 0 0 1 12.99038 97.5 L60 16.07695 L107.00962 97.5 A15 15 0 0 1 94.01924 120 Z",
    );

    expect(readText("#effective")).toBe(
      "Effective radius: mixed, shown for each point in Points mode.",
    );
  });

  it("step 70: the stretched hexagon, converted, gets a point under its base", () => {
    openOnStep(70);
    clickElement("mode-points");

    const before = readText("#path-data");

    clickElement("convert");

    expect(readText("#path-data")).toBe(before);

    clickPoint(1, false);
    clickPoint(2, true);
    clickElement("add-midpoint");
    typeAndEnter("point-y", "120");

    expect(readText("#path-data")).toBe(
      "M113.97802 100 L80 120 L46.02198 100 L0 50 L46.02198 0 L113.97802 0 L160 50 Z",
    );

    expect(readValue("height")).toBe("120");
  });

  it("step 71: the turned hexagon, converted then widened, keeps its left side", () => {
    openOnStep(71);
    typeAndEnter("angle", "30");
    clickElement("mode-points");

    const before = readText("#path-data");

    clickElement("convert");

    expect(readText("#path-data")).toBe(before);

    typeAndEnter("width", "200");

    // Point 3, on the left side of the box, stays at (10.71797, 10).
    expect(readText("#path-data")).toBe(
      "M109.10279 124.53753 L35.53822 82.06501 L10.71797 10 L85.53822 -4.53753 L159.10279 37.93499 L183.92305 110 Z",
    );

    expect(readValue("angle")).toBe("30");
  });

  it("step 72: the rounded stretched hexagon, placed by its straight box", () => {
    openOnStep(72);

    expect(readText("#effective")).toBe("Effective radius: 50 (requested 1000, reduced to fit)");

    typeAndEnter("shape-x", "0");

    expect(readText("#straight-box")).toBe("Straight box: x 0, y 0, width 124.08793, height 100");
  });
});
