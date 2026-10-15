// @vitest-environment happy-dom

import { describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  commitField,
  openServedPage,
  pressKey,
  pressPointer,
  readDisplay,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";

/**
 * Types a value and leaves the field, as the designer pressing Tab.
 *
 * @param id - the field
 * @param value - what is typed or pasted
 */
function typeAndLeave(id: string, value: string): void {
  typeValue(id, value);
  commitField(id);
}

/**
 * Reads the picker's three values.
 *
 * @returns the hue, the saturation, the lightness or brightness
 */
function readPickerValues(): readonly string[] {
  return ["color-hue-value", "color-saturation-value", "color-level-value"].map((id) =>
    readValue(id),
  );
}

/**
 * Reads the fill and the fill opacity of the text and of its background band.
 *
 * @returns the text's fill and opacity, then the band's
 */
function readFills(): readonly (string | null | undefined)[] {
  const text = selectPageElement("#canvas text");
  const band = text.parentElement?.querySelector("path");

  return [
    text.getAttribute("fill"),
    text.getAttribute("fill-opacity"),
    band?.getAttribute("fill"),
    band?.getAttribute("fill-opacity"),
  ];
}

describe("US-035 Product Owner test card", () => {
  it("follows the card from step 1 to step 9, one step after the other", () => {
    openServedPage("");
    clickElement("tool-text");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("color-model", "hsb");

    // 1. F80 shown #ff8800, the text orange.
    typeAndLeave("text-color", "F80");
    expect([readValue("text-color"), readFills()[0]]).toEqual(["#ff8800", "#ff8800"]);

    // 2. #0000ffcc: #0000ff at 80, the band a little transparent.
    clickElement("text-background");
    typeAndLeave("text-background-color", "#0000ffcc");

    expect([readValue("text-background-color"), readValue("text-background-opacity")]).toEqual([
      "#0000ff",
      "80",
    ]);

    expect(readFills().slice(2)).toEqual(["#0000ff", "0.8"]);

    // 3. red refused: #ff8800 back, a message.
    typeAndLeave("text-color", "red");
    expect(readValue("text-color")).toBe("#ff8800");
    expect(readText("#text-color-message")).not.toBe("");

    // 4. #ff8000 and its swatch: hue 30.1, HSB 30.1, 100, 100.
    typeAndLeave("text-color", "#ff8000");
    clickElement("text-color-swatch");
    expect(readDisplay("color-picker") === "none").toBe(false);
    expect(selectPageElement("#color-hue").getAttribute("aria-valuenow")).toBe("30.1");
    expect(readPickerValues()).toEqual(["30.1", "100", "100"]);
    expect(selectPageElement("#color-plane").dataset["model"]).toBe("hsb");

    // 5. HSL: 30.1, 100, 50, the plane in HSL.
    chooseOption("color-model", "hsl");
    expect(readPickerValues()).toEqual(["30.1", "100", "50"]);
    expect(selectPageElement("#color-plane").dataset["model"]).toBe("hsl");

    // 6. HSB, hue 210, the middle of the plane's right edge: #004080.
    chooseOption("color-model", "hsb");
    typeAndLeave("color-hue-value", "210");
    setElementBox("#color-plane", { height: 100, left: 0, top: 0, width: 200 });
    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 200, clientY: 50 });
    expect(readValue("text-color")).toBe("#004080");

    // 7. S 66.7, B 60: #336699.
    typeAndLeave("color-saturation-value", "66.7");
    typeAndLeave("color-level-value", "60");
    expect(readValue("text-color")).toBe("#336699");

    // 8. Right, Page Up, Home, End on the hue: 211, 221, 0, 359.9.
    const hues = ["ArrowRight", "PageUp", "Home", "End"].map((key) => {
      pressKey("#color-hue", key);

      return readValue("color-hue-value");
    });

    expect(hues).toEqual(["211", "221", "0", "359.9"]);

    // 9. Escape: closed, the color kept. HSB(359.9°, 66.7 %, 60 %): red 153 (99), green 51 (33),
    // blue 0.6 − 0.6 · 0.667 · 0.99833 = 0.20047, 51.1 (33).
    pressKey("body", "Escape");
    expect(readDisplay("color-picker")).toBe("none");
    expect(readValue("text-color")).toBe("#993333");
  });
});
