// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  clickElement,
  clickPoint,
  openOnStep,
  readPointRows,
  readPressedState,
  readText,
  readValue,
  typeAndEnter,
} from "../user-actions";

/**
 * Opens the page on a round shape 120 × 80, in Points mode.
 *
 * @param width - width of its box
 */
function openRoundShape(width: string): void {
  openOnStep(1);
  clickElement("tool-round");
  typeAndEnter("width", width);
  typeAndEnter("height", "80");
  clickElement("mode-points");
}

/**
 * Whether a button of the page is greyed.
 *
 * @param id - its id
 * @returns whether it has the `disabled` attribute
 */
function isDisabled(id: string): boolean {
  return document.querySelector(`#${id}`)?.hasAttribute("disabled") === true;
}

describe("converting a round shape (US-030)", () => {
  it("[F10.AC4] gives its 4 points with their handles, the drawing unchanged", () => {
    openRoundShape("120");

    const drawn = readText("#path-data");

    clickElement("convert");

    expect(readValue("shape")).toBe("free");
    expect(readText("#path-data")).toBe(drawn);
    expect(readPointRows()).toHaveLength(4);

    clickPoint(1, false);

    expect(readPressedState("handles-switch")).toBe("true");
    expect(readValue("handle-mode")).toBe("angle-and-length");
  });

  it("[F10.AC5] gives the 4 corners of its box rounded by 40: a capsule", () => {
    // Half the smaller side, 80 / 2 = 40 (DERIV-round-shape step 2).
    openRoundShape("120");
    clickElement("convert-rounded");

    expect(readValue("shape")).toBe("free");

    expect(readPointRows()).toEqual([
      "1 · x 0 · y 0 · radius 40",
      "2 · x 120 · y 0 · radius 40",
      "3 · x 120 · y 80 · radius 40",
      "4 · x 0 · y 80 · radius 40",
    ]);

    expect(readText("#path-data")).toBe(
      "M40 0 L80 0 A40 40 0 0 1 120 40 A40 40 0 0 1 80 80 L40 80 A40 40 0 0 1 0 40 A40 40 0 0 1 40 0 Z",
    );
  });

  it("greys both buttons on a box of width 0, and converts nothing", () => {
    openRoundShape("0");

    expect([isDisabled("convert"), isDisabled("convert-rounded")]).toEqual([true, true]);

    clickElement("convert");
    clickElement("convert-rounded");

    expect(readValue("shape")).toBe("round");
  });

  it("shows the rounded conversion for a round shape only", () => {
    openOnStep(1);
    clickElement("tool-polygon");
    clickElement("mode-points");

    expect(document.querySelector<HTMLElement>("#convert-rounded")?.hidden).toBe(true);
    expect(document.querySelector<HTMLElement>("#convert")?.hidden).toBe(false);
  });
});
