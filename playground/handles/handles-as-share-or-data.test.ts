// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openOnStep,
  readText,
  typeAndEnter,
  typeValue,
} from "../user-actions";

/**
 * Pastes a text on the side after the point selected.
 *
 * @param text - what is pasted
 */
function pasteCurve(text: string): void {
  typeValue("handle-paste", text);
  clickElement("paste-curve");
}

/**
 * Whether the line saying why a paste was refused is hidden.
 *
 * @returns its `hidden` state
 */
function isMessageHidden(): boolean {
  return document.querySelector<HTMLElement>("#paste-message")?.hidden !== false;
}

describe("handles typed as shares of the side, or pasted (US-025)", () => {
  beforeEach(() => {
    // The rectangle 100 × 80: points 1 (0, 0), 2 (100, 0); Points mode, point 1, handles on.
    openOnStep(1);
    typeAndEnter("width", "100");
    clickElement("mode-points");
    clickPoint(1, false);
    clickElement("handles-switch");
  });

  it("[F12.AC4] takes 25 % along and 30 % across the side, above it", () => {
    // Side (0, 0) → (100, 0): 25 % along, 30 % across: the end at (25, −30).
    chooseOption("handle-method", "share");
    typeAndEnter("handle-after-x", "25");
    typeAndEnter("handle-after-y", "30");

    expect(readText("#path-data")).toContain("C25 -30 ");
  });

  it("[F12.AC4] pastes a C command on the side after the point, both ends' handles set", () => {
    pasteCurve("C 30 -15 70 -15 100 0");

    expect(readText("#path-data")).toContain("C30 -15 70 -15 100 0");
    // Point 1 mirrors its handle (mode angle and length): the left side bends too.
    expect(readText("#path-data")).toContain("C0 80 -30 15 0 0");
    // Point 2 mirrors its handle too: (30, 15) after, the right side bends.
    expect(readText("#path-data")).toContain("C130 15 100 80 100 80");
  });

  it("[F12.AC4] lays a cubic-bezier() on the side", () => {
    // ease-in-out on (0, 0) → (100, 0): (21, 21) and (79, −21).
    pasteCurve("cubic-bezier(0.42, 0, 0.58, 1)");

    expect(readText("#path-data")).toContain("C21 21 79 -21 100 0");
  });

  it("refuses another text with a line saying why, the drawing unchanged", () => {
    const before = readText("#path-data");

    pasteCurve("C 30 -15 70 -15 90 0");

    expect(readText("#paste-message")).toBe("The C must end on the side's last point.");
    expect(isMessageHidden()).toBe(false);
    expect(readText("#path-data")).toBe(before);

    pasteCurve("C 30 -15 70 -15 100 0");

    expect(isMessageHidden()).toBe(true);
  });

  it("[F12.AC4] leaves the next point's handles alone in mode none", () => {
    clickPoint(2, false);
    clickElement("handles-switch");
    chooseOption("handle-mode", "none");
    clickPoint(1, false);
    pasteCurve("C 30 -15 70 -15 100 0");

    expect(readText("#path-data")).toContain("C30 -15 70 -15 100 0");
    expect(readText("#path-data")).not.toContain("C130 15");
  });

  it("[F12.AC4] pastes on the side after the last point, back to point 1", () => {
    clickPoint(4, false);
    clickElement("handles-switch");
    pasteCurve("C -15 50 -15 30 0 0");

    expect(readText("#path-data")).toContain("C-15 50 -15 30 0 0");
  });

  it("[F12.AC4] pastes control points as drawn on a turned shape", () => {
    // Turned 90° about (50, 40): point 1 at (90, −10), point 2 at (90, 90).
    typeAndEnter("angle", "90");
    pasteCurve("C 105 15 105 65 90 90");

    expect(readText("#path-data")).toContain("C105 15 105 65 90 90");
  });
});
