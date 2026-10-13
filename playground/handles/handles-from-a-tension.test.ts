// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openOnStep,
  readText,
  typeAndEnter,
} from "../user-actions";

/**
 * Whether the tension field is hidden.
 *
 * @returns its `hidden` state
 */
function isTensionHidden(): boolean {
  return document.querySelector<HTMLElement>("#handle-tension-field")?.hidden !== false;
}

describe("handles from a tension (US-026)", () => {
  beforeEach(() => {
    // The rectangle 100 × 80: point 2 (100, 0) between (0, 0) and (100, 80); Points mode, point 2,
    // handles on, smoothed by a tension.
    openOnStep(1);
    typeAndEnter("width", "100");
    clickElement("mode-points");
    clickPoint(2, false);
    clickElement("handles-switch");
    chooseOption("handle-mode", "auto");
  });

  it("[F12.AC4] computes the handles from the neighbours, Catmull–Rom at tension 0", () => {
    // (100, 80) − (0, 0) over 6 = (16.66667, 13.33333): the side to point 3 leaves through (116.66667, 13.33333).
    expect(isTensionHidden()).toBe(false);
    expect(readText("#path-data")).toContain("C116.66667 13.33333 ");
  });

  it("[F12.AC4] straightens the sides at tension 100", () => {
    expect(document.querySelector<HTMLInputElement>("#handle-tension")?.value).toBe("0");

    typeAndEnter("handle-tension", "100");

    expect(document.querySelector<HTMLInputElement>("#handle-tension")?.value).toBe("100");

    expect(readText("#path-data")).not.toContain("C");
  });

  it("refuses a tension outside 0 to 100, or not whole", () => {
    typeAndEnter("handle-tension", "150");

    expect(readText("#path-data")).toContain("C116.66667 13.33333 ");

    typeAndEnter("handle-tension", "12.5");

    expect(readText("#path-data")).toContain("C116.66667 13.33333 ");
  });

  it("[F12.AC4] follows a neighbour that moves", () => {
    // Point 3 moved to (150, 80): the handle becomes (150, 80) / 6 = (25, 13.33333).
    clickPoint(3, false);
    typeAndEnter("point-x", "150");

    expect(readText("#path-data")).toContain("C125 13.33333 ");
  });

  it("leaves the tension once a handle is typed, for mode angle", () => {
    typeAndEnter("handle-after-x", "130");

    expect(document.querySelector<HTMLSelectElement>("#handle-mode")?.value).toBe("angle");
    expect(isTensionHidden()).toBe(true);
  });
});
