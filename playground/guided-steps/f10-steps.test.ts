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

import { GUIDED_STEPS_F12 } from "./guided-steps-f12";
import { GUIDED_STEPS_OF_EPIC_1 } from "./guided-steps-of-epic-1";

/**
 * Opens the playground on a step of F10.
 *
 * @param step - the step of F10, from 1
 */
function openStep(step: number): void {
  openOnStep(GUIDED_STEPS_OF_EPIC_1.length + GUIDED_STEPS_F12.length + step);
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

describe("the guided test of F10 (VAL-012)", () => {
  it("step 1 draws a circle, step 2 an ellipse touching its box", () => {
    openStep(1);

    expect(readText("#path-data")).toMatch(/^M50 0 C77\.61424 0 100 22\.38576 100 50 /u);
    expect(readValue("shape")).toBe("round");

    openStep(2);

    expect(readText("#straight-box")).toBe("Straight box: x 0, y 0, width 120, height 80");
  });

  it("step 3 turns it about the centre of its box", () => {
    openStep(3);
    typeAndEnter("angle", "30");

    expect(readText("#path-data")).toMatch(/^M80 5\.35898 C/u);
  });

  it("step 4 lists its 4 points, top first, without a handles switch", () => {
    openStep(4);
    clickElement("mode-points");
    clickPoint(1, false);

    expect(readPointRows()[0]).toBe("1 · x 60 · y 0 · radius 0");
    expect(document.querySelector<HTMLElement>("#handles-switch")?.hidden).toBe(true);
  });

  it("step 5 converts it to its points, unmoved, point 1 in handles mode", () => {
    openStep(5);
    clickElement("mode-points");

    const drawn = readText("#path-data");

    clickElement("convert");
    clickPoint(1, false);

    expect(readText("#path-data")).toBe(drawn);
    expect(readPressedState("handles-switch")).toBe("true");
  });

  it("step 6 converts it to a capsule of radius 40", () => {
    openStep(6);
    clickElement("mode-points");
    clickElement("convert-rounded");

    expect(readPointRows()[0]).toBe("1 · x 0 · y 0 · radius 40");
  });

  it("step 7 greys both conversions at width 0", () => {
    openStep(7);
    typeAndEnter("width", "0");
    clickElement("mode-points");

    expect([isDisabled("convert"), isDisabled("convert-rounded")]).toEqual([true, true]);
  });
});
