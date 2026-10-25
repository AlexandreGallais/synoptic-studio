// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  commitField,
  openPage,
  pressKey,
  pressPointer,
  readValue,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";

/**
 * Types a value and leaves the field, as the designer pressing Tab.
 *
 * @param id - the field
 * @param value - what is typed
 */
function typeAndLeave(id: string, value: string): void {
  typeValue(id, value);
  commitField(id);
}

/**
 * Reads the picker's three values and the color they give.
 *
 * @returns the hue, the saturation, the lightness or brightness, then the hexadecimal
 */
function readPicked(): readonly string[] {
  return ["color-hue-value", "color-saturation-value", "color-level-value", "text-color"].map(
    (id) => readValue(id),
  );
}

/**
 * Presses the plane at a point of the window, the plane laid out 200 × 100 from (10, 20).
 *
 * @param type - `pointerdown` or `pointermove`
 * @param clientX - the pointer's x in the window
 * @param clientY - the pointer's y in the window
 */
function pressPlane(type: string, clientX: number, clientY: number): void {
  pressPointer("#color-plane", type, { buttons: 1, clientX, clientY });
}

describe("Color picker: the plane", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("color-model", "hsb");
    typeAndLeave("text-color", "#ff8000");
    clickElement("text-color-swatch");
    setElementBox("#color-plane", { height: 100, left: 10, top: 20, width: 200 });
  });

  it("[F16.AC2] paints the plane in HSB, then in HSL with the lightness up, for the hue held", () => {
    const plane = selectPageElement("#color-plane");
    const mark = selectPageElement("#color-plane-mark");

    expect(plane.dataset["model"]).toBe("hsb");
    expect(plane.getAttribute("aria-label")).toBe("Saturation and brightness");
    // The hue at its purest, HSB(30°, 100 %, 100 %).
    expect(plane.style.getPropertyValue("--hue-color")).toBe("#ff8000");

    chooseOption("color-model", "hsl");

    expect(plane.dataset["model"]).toBe("hsl");
    expect(plane.getAttribute("aria-label")).toBe("Saturation and lightness");
    // HSL 30°, 100 %, 50 %: the mark at the right end, halfway up.
    expect([mark.style.left, mark.style.top]).toEqual(["100%", "50%"]);
  });

  it("[F16.AC2] picks 100 %, 50 % at the middle of the right edge: #004080 at 210°", () => {
    typeAndLeave("color-hue-value", "210");
    // (210 − 10, 70 − 20) = (200, 50): S = 100 · 200 / 200, B = 100 · (100 − 50) / 100.
    pressPlane("pointerdown", 210, 70);

    expect(readPicked()).toEqual(["210", "100", "50", "#004080"]);
    expect(selectPageElement("#canvas text").getAttribute("fill")).toBe("#004080");
    expect(selectPageElement("#color-plane-mark").style.left).toBe("100%");
    expect(selectPageElement("#color-plane-mark").style.top).toBe("50%");

    // #336699 shows 210, 67, 60; 67 typed again is the value shown, so the exact 102 / 153 stays.
    typeAndLeave("text-color", "#336699");
    typeAndLeave("color-saturation-value", "67");
    expect(readPicked()).toEqual(["210", "67", "60", "#336699"]);

    // 61 typed: 210°, 102 / 153, 61 %: red 51.85 → 52, green 103.7 → 104, blue 155.55 → 156;
    // S still 67 (ADR-0058).
    typeAndLeave("color-level-value", "61");
    expect(readPicked()).toEqual(["210", "67", "61", "#34689c"]);
  });

  it("[F16.AC2] puts the mark at the top left on Home: 0 %, 100 % up", () => {
    pressKey("#color-plane", "Home");

    // HSB 30°, 0 %, 100 %: x = 100 · 0 / 100 = 0; y = 100 − 100 · 100 / 100 = 0.
    const mark = selectPageElement("#color-plane-mark");

    expect([mark.style.left, mark.style.top]).toEqual(["0%", "0%"]);

    pressKey("#color-plane", "ArrowDown");
    // Brightness 99: y = 100 − 99 = 1.
    expect(mark.style.top).toBe("1%");
  });

  it("[F16.AC2] follows a drag beyond the plane's edges, the button held", () => {
    pressPlane("pointerdown", 110, 70);
    // (−90 − 10, 170 − 20): beyond the left and bottom edges, 0 % and 0 %.
    pressPlane("pointermove", -90, 170);
    expect(readPicked()).toEqual(["30", "0", "0", "#000000"]);

    pressPointer("#color-plane", "pointermove", { buttons: 0, clientX: 210, clientY: 20 });
    expect(readPicked()).toEqual(["30", "0", "0", "#000000"]);
  });

  it.each([
    // 99 % with the exact hue 30.11765° and brightness 1: green 1 − 0.99 · 0.49804 = 0.50694,
    // 129.27 → 81; blue 0.01, 2.55 → 03; shown hue 60 · 126 / 252 = 30, S 252 / 255 → 99.
    { expected: ["30", "99", "100", "#ff8103"], key: "ArrowLeft" },
    // 99 % brightness, S 1 exact: red 252.45 → fc; green 0.99 · 0.50196 = 0.49694, 126.7 → 7f.
    { expected: ["30", "100", "99", "#fc7f00"], key: "ArrowDown" },
    // PageDown: 90 % gives #ff8d1a (hue 60 · 115 / 229 = 30.131°); again 80 %: green
    // 1 − 0.8 · 0.49782 = 0.60175, 153.4 → 99; blue 0.2, 51 → 33.
    { expected: ["30", "80", "100", "#ff9933"], key: "PageDown" },
    // White, a grey: the hue held, 30.
    { expected: ["30", "0", "100", "#ffffff"], key: "Home" },
  ])("[F16.AC2] answers $key on the plane", ({ expected, key }) => {
    pressKey("#color-plane", key);

    if (key === "PageDown") {
      pressKey("#color-plane", key);
    }

    expect(readPicked()).toEqual(expected);

    expect(selectPageElement("#color-plane").getAttribute("aria-valuetext")).toBe(
      `Saturation ${expected[1] ?? ""} %, brightness ${expected[2] ?? ""} %`,
    );
  });

  it("does not answer a key the plane does not answer", () => {
    pressKey("#color-plane", "Enter");

    expect(readPicked()).toEqual(["30", "100", "100", "#ff8000"]);
  });

  it("[F16.AC2] keeps the slider's hue for a grey while the picker is open", () => {
    typeAndLeave("text-color", "#808080");
    // V = 128 / 255 = 50.2 %, shown 50.
    expect(readPicked()).toEqual(["30", "0", "50", "#808080"]);

    typeAndLeave("color-hue-value", "200");
    expect(readPicked()).toEqual(["200", "0", "50", "#808080"]);

    // (60 − 10, 20 − 20) = (50, 0): S 25 %, B 100 %. HSB(200°, 25 %, 100 %): red 0.75, 191.25 → bf;
    // green 1 − 0.25 · 0.33333 = 0.91667, 233.75 → ea; blue ff; shown hue 199.7 → 200, S 25.1 → 25.
    pressPlane("pointerdown", 60, 20);
    expect(readPicked()).toEqual(["200", "25", "100", "#bfeaff"]);
  });

  it("[F16.AC2] opens a grey at the hue 0, and keeps its hue in the other model", () => {
    typeAndLeave("text-color", "#808080");
    clickElement("text-color-swatch");
    clickElement("text-color-swatch");
    expect(readPicked()).toEqual(["0", "0", "50", "#808080"]);

    typeAndLeave("color-hue-value", "120");
    chooseOption("color-model", "hsl");
    expect(readPicked()).toEqual(["120", "0", "50", "#808080"]);
  });
});
