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

    // 4. #ff8000 and its swatch: hue 30, HSB 30, 100, 100, no decimal (60 · 128 / 255 = 30.1 → 30).
    typeAndLeave("text-color", "#ff8000");
    clickElement("text-color-swatch");
    expect(readDisplay("color-picker") === "none").toBe(false);
    expect(selectPageElement("#color-hue").getAttribute("aria-valuenow")).toBe("30");
    expect(readPickerValues()).toEqual(["30", "100", "100"]);
    expect(selectPageElement("#color-plane").dataset["model"]).toBe("hsb");

    // 5. HSL: 30, 100, 50, the plane in HSL.
    chooseOption("color-model", "hsl");
    expect(readPickerValues()).toEqual(["30", "100", "50"]);
    expect(selectPageElement("#color-plane").dataset["model"]).toBe("hsl");

    // 6. HSB, hue 210, the middle of the plane's right edge: #004080.
    chooseOption("color-model", "hsb");
    typeAndLeave("color-hue-value", "210");
    setElementBox("#color-plane", { height: 100, left: 0, top: 0, width: 200 });
    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 200, clientY: 50 });
    expect(readValue("text-color")).toBe("#004080");

    // 7. #336699 typed, kept; S left at 67: unchanged; B 61: 210°, the exact 102 / 153, 61 %, so
    // red 51.85 → 52, green 103.7 → 104, blue 155.55 → 156: #34689c, S still 67.
    typeAndLeave("text-color", "#336699");
    expect(readPickerValues()).toEqual(["210", "67", "60"]);
    typeAndLeave("color-saturation-value", "67");
    expect(readValue("text-color")).toBe("#336699");
    typeAndLeave("color-level-value", "61");
    expect([readValue("text-color"), readPickerValues()[1]]).toEqual(["#34689c", "67"]);

    // 8. Right, Page Up, Home, End on the hue: 211, 221, 0, 359.
    const hues = ["ArrowRight", "PageUp", "Home", "End"].map((key) => {
      pressKey("#color-hue", key);

      return readValue("color-hue-value");
    });

    expect(hues).toEqual(["211", "221", "0", "359"]);

    // 9. Escape: closed, the color kept. Each hue with S the exact 102 / 153 of #336699 and B 61
    // held: 359° gives red 0.61 · 255 = 155.55 → 156 (9c), green 0.61 / 3 · 255 = 51.85 → 52 (34),
    // blue 0.61 · (1 − 2/3 · 59/60) · 255 = 53.58 → 54 (36).
    pressKey("body", "Escape");
    expect(readDisplay("color-picker")).toBe("none");
    expect(readValue("text-color")).toBe("#9c3436");
  });
});
