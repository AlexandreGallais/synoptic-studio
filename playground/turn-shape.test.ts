// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { chooseOption, clickElement, openPage, readText, typeValue } from "./user-actions";

/** Fires the `change` event of the angle field, as when Enter is pressed or the field is left. */
function commitAngle(): void {
  document.querySelector("#angle")?.dispatchEvent(new Event("change"));
}

/**
 * Angle shown in the angle field.
 *
 * @returns its value, empty when absent
 */
function angleShown(): string {
  return document.querySelector<HTMLInputElement>("#angle")?.value ?? "";
}

/**
 * Number of paths of a class drawn on the canvas.
 *
 * @param name - class of the paths
 * @returns how many are drawn
 */
function drawn(name: string): number {
  return document.querySelectorAll(`#canvas path.${name}`).length;
}

/** Draws a square polygon in the 120 × 80 box and converts it to a free shape (F08). */
function convertSquare(): void {
  chooseOption("shape", "polygon");

  typeValue("corners", "4");
  clickElement("mode-points");
  clickElement("convert");
}

describe("turning and moving a shape (US-018)", () => {
  beforeEach(() => {
    // Step 1 of the guided test: a 120 × 80 rectangle, radius 10 (F01).
    openPage("?step=1");
    typeValue("radius", "0");
  });

  it("[F04.AC5] shows a straight shape at 0°, its straight box being its box", () => {
    expect(angleShown()).toBe("0");
    expect(readText("#straight-box")).toBe("Straight box: x 0, y 0, width 120, height 80");
    expect(drawn("turned-box")).toBe(0);
    expect(drawn("straight-box")).toBe(0);
  });

  it("[F04.AC1] turns the rectangle clockwise by 90° about the center of its box", () => {
    // 120 × 80, center (60, 40): (0, 0) ↦ (60 + 40, 40 − 60) = (100, −20); the straight box runs
    // from (20, −20), 80 wide and 120 high.
    typeValue("angle", "90");

    expect(readText("#straight-box")).toBe("Straight box: x 20, y -20, width 80, height 120");
    expect(readText("#path-data")).toBe("M100 -20 L100 100 L20 100 L20 -20 Z");
  });

  it("[F04.AC5] draws the turned box as the selection and the straight box dashed", () => {
    typeValue("angle", "30");

    expect(drawn("turned-box")).toBe(1);
    expect(drawn("straight-box")).toBe(1);
  });

  it("[F04.AC1] shows the angle kept from 0 to 359 once typed", () => {
    typeValue("angle", "370");
    commitAngle();

    expect(angleShown()).toBe("10");

    typeValue("angle", "-90");
    commitAngle();

    expect(angleShown()).toBe("270");
  });

  it("[F04.AC5] turns by −90° and +90° with the buttons, wrapping at 0", () => {
    clickElement("turn-right");

    expect(angleShown()).toBe("90");

    clickElement("turn-left");
    clickElement("turn-left");

    expect(angleShown()).toBe("270");

    clickElement("turn-right");

    expect(angleShown()).toBe("0");
  });

  it("[F04.AC3] moves the shape so that its straight box starts at the x and y typed", () => {
    typeValue("angle", "90");
    typeValue("shape-x", "0");
    typeValue("shape-y", "0");

    // The straight box went from (20, −20) to (0, 0): the shape moved by (−20, 20).
    expect(readText("#straight-box")).toBe("Straight box: x 0, y 0, width 80, height 120");
    expect(readText("#path-data")).toBe("M80 0 L80 120 L0 120 L0 0 Z");
  });

  it("[F04.AC3] keeps the center of the box where it is when the shape turns", () => {
    typeValue("shape-x", "100");
    typeValue("angle", "90");

    // At (100, 0) the center is (160, 40); turned, the straight box runs from 160 − 40 = 120.
    expect(readText("#straight-box")).toBe("Straight box: x 120, y -20, width 80, height 120");
  });

  it("[F04.AC2] turns a polygon's box with it: a triangle's base at the bottom goes to the left", () => {
    chooseOption("shape", "polygon");

    typeValue("corners", "3");
    typeValue("width", "100");
    typeValue("height", "100");

    const bottom = [...document.querySelectorAll<HTMLInputElement>('input[name="anchor"]')].find(
      (button) => button.value === "mid max",
    );

    if (bottom !== undefined) {
      bottom.checked = true;
      bottom.dispatchEvent(new Event("change"));
    }

    typeValue("angle", "90");

    // The base, at y = 100 from x = 0 to 100, turned about (50, 50), lies on x = 0: point 1, the
    // right end of the base (100, 100), is drawn at (0, 100).
    expect(readText("#path-data")).toMatch(/^M0 100 /u);
    expect(drawn("box")).toBe(1);
  });

  it("[F04.AC1] refuses an angle that is not an integer", () => {
    typeValue("angle", "2.5");

    expect(readText("#status")).not.toBe("");
  });

  it("[F04.AC3] moves nothing for a position that is not an integer", () => {
    typeValue("shape-x", "2.5");

    expect(readText("#straight-box")).toBe("Straight box: x 0, y 0, width 120, height 80");
  });

  it("[F04.AC5] draws the turned box as a frame: the box and the box inset by 1", () => {
    // 120 × 80 at 90°: outer box from (20, −20) to (100, 100); inner one from (21, −19) to (99, 99).
    typeValue("angle", "90");

    expect(document.querySelector("#canvas path.turned-box")?.getAttribute("d")).toBe(
      "M100 -20 L100 100 L20 100 L20 -20 Z M99 -19 L99 99 L21 99 L21 -19 Z",
    );
  });

  it("[F04.AC3] leaves the X field alone while it is typed in", () => {
    const field = document.querySelector<HTMLInputElement>("#shape-x");

    field?.focus();
    typeValue("angle", "90");

    // The straight box starts at 20 now, but the field being typed keeps what it holds.
    expect(field?.value).toBe("0");
  });

  it("[F04.AC3] moves a converted free shape without undoing the width typed", () => {
    convertSquare();
    typeValue("width", "200");
    typeValue("shape-x", "10");

    expect(readText("#straight-box")).toMatch(/^Straight box: x 10, y [^,]+, width 200,/u);
  });

  it("[F04.AC1] turns a converted free shape from the angle field", () => {
    convertSquare();
    // The square fitted in the 120 × 80 box is 80 × 80; at 45° its straight box is 80√2 wide.
    typeValue("angle", "45");

    expect(readText("#straight-box")).toMatch(/width 113\.13708, height 113\.13708$/u);
  });

  it("[F04.AC5] turns from 0 with a button when the angle field is empty", () => {
    typeValue("angle", "");
    clickElement("turn-right");

    expect(angleShown()).toBe("90");
  });

  it("[F04.AC1] keeps the view still while the shape turns, so that it turns about its center on screen", () => {
    // 120 × 80 turned by 90° goes up to y −20: the view does not follow it (VAL-007).
    typeValue("angle", "90");

    expect(document.querySelector("#canvas svg")?.getAttribute("viewBox")).toMatch(/^-40 -40 /u);
  });

  it("[F04.AC5] opens the guided test of F04 at step 47, each step starting straight", () => {
    openPage("?step=47");

    expect(readText("#guide-title")).toBe("F04 · Turn by 90°");

    typeValue("angle", "90");

    expect(readText("#straight-box")).toBe("Straight box: x 20, y -20, width 80, height 120");

    clickElement("guide-next");

    expect(angleShown()).toBe("0");
    expect(readText("#guide-title")).toBe("F04 · Tilt by 45°");
  });
});
