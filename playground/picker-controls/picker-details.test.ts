// @vitest-environment happy-dom

import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  commitField,
  openPage,
  pressKey,
  pressPointer,
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
 * @param value - what is typed
 */
function typeAndLeave(id: string, value: string): void {
  typeValue(id, value);
  commitField(id);
}

/**
 * Presses a key on an element with a cancellable event, as the browser fires it.
 *
 * @param selector - CSS selector of the element
 * @param key - the key pressed
 * @returns whether the page prevented the key's default action
 */
function pressCancellableKey(selector: string, key: string): boolean {
  const event = new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key });

  selectPageElement(selector).dispatchEvent(event);

  return event.defaultPrevented;
}

describe("Color picker: focus, keys, rounding and what it writes (audit)", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("color-model", "hsb");
    typeAndLeave("text-color", "#ff8000");
    clickElement("text-color-swatch");
  });

  it("gives the focus back to the swatch on Escape", () => {
    selectPageElement("#color-hue").focus();
    pressKey("#color-hue", "Escape");

    expect(document.activeElement).toBe(selectPageElement("#text-color-swatch"));
  });

  it("takes the focus and the pointer on a press, so that a drag goes on beyond the edges", () => {
    const plane = selectPageElement("#color-plane");
    const capture = vi.spyOn(plane, "setPointerCapture");

    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 0, clientY: 0 });

    expect(document.activeElement).toBe(plane);
    expect(capture).toHaveBeenCalledWith(1);
  });

  it.each(["#color-hue", "#color-opacity", "#color-plane"])(
    "prevents the page's scrolling on a key %s answers, not on another",
    (selector) => {
      expect(pressCancellableKey(selector, "PageDown")).toBe(true);
      expect(pressCancellableKey(selector, "a")).toBe(false);
    },
  );

  it("brings a value typed beyond 100 to 100, and rounds 66.75 to 67", () => {
    typeAndLeave("color-saturation-value", "150");
    expect(readValue("color-saturation-value")).toBe("100");

    // 67 % with the exact hue and brightness: #ffaa54, its saturation 171 / 255 = 67.06 → 67.
    typeAndLeave("color-saturation-value", "66.75");
    expect(readValue("color-saturation-value")).toBe("67");
  });

  it("rounds the plane's level to a whole number: 200 of 300 high is 67", () => {
    setElementBox("#color-plane", { height: 300, left: 0, top: 0, width: 300 });
    // 100 · (300 − 100) / 300 = 66.667, 67; x 300 of 300 is 100 %, the value shown, so exact.
    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 300, clientY: 100 });

    expect(readValue("color-level-value")).toBe("67");
    // Red 0.67 · 255 = 170.85 → ab; green 0.67 · (1 − 0.49804) = 0.33631, 85.76 → 56.
    expect(readValue("text-color")).toBe("#ab5600");
    expect(selectPageElement("#color-plane").getAttribute("aria-valuenow")).toBe("100");
  });

  it("rounds a typed hue of 1.4 to 1, then takes it to 0 on Left", () => {
    // HSB 1°, 1, 1: green 1 − 59 / 60 = 0.01667, 4.25 → 04; #ff0400 shows 60 · 4 / 255 = 0.94 → 1.
    typeAndLeave("color-hue-value", "1.4");
    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["1", "#ff0400"]);

    pressKey("#color-hue", "ArrowLeft");
    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["0", "#ff0000"]);
  });

  it("paints the plane with the pure hue, and the opacity slider with the color", () => {
    typeAndLeave("color-hue-value", "210");

    // HSB(210°, 100 %, 100 %): red 0, green 0.5, 127.5 gives 128 (80), blue ff.
    expect(selectPageElement("#color-plane").style.getPropertyValue("--hue-color")).toBe("#0080ff");

    expect(selectPageElement("#color-opacity").style.getPropertyValue("--picked-color")).toBe(
      readValue("text-color"),
    );
  });

  it("places the hue's thumb at 100 · 30 / 360 = 8.33333 %", () => {
    expect(selectPageElement("#color-hue-thumb").style.left).toMatch(/^8\.33333/u);
  });

  it("keeps the color a plane click wrote, as the one a refused hexadecimal brings back", () => {
    setElementBox("#color-plane", { height: 100, left: 0, top: 0, width: 200 });
    typeAndLeave("color-hue-value", "210");
    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 200, clientY: 50 });
    typeAndLeave("text-color", "red");

    expect(readValue("text-color")).toBe("#004080");
  });

  it("clears the line saying why once the picker writes a color", () => {
    typeAndLeave("text-color", "red");
    expect(readText("#text-color-message")).not.toBe("");

    pressKey("#color-hue", "ArrowRight");
    expect(readText("#text-color-message")).toBe("");
  });

  it("closes the first picker when another swatch opens its own", () => {
    clickElement("text-background-color-swatch");

    expect(selectPageElement("#text-color-swatch").getAttribute("aria-expanded")).toBe("false");

    expect(selectPageElement("#text-background-color-swatch").getAttribute("aria-expanded")).toBe(
      "true",
    );
  });

  it("keeps a color set without any event, once the field gets the focus", () => {
    const field = selectPageElement("#text-color");

    if (field instanceof HTMLInputElement) {
      field.value = "#123456";
    }

    field.dispatchEvent(new FocusEvent("focus"));
    typeAndLeave("text-color", "red");

    expect(readValue("text-color")).toBe("#123456");
  });

  it("[F16.AC2] gives #336699 back when S goes to 50 then to 67, the value read (ADR-0059)", () => {
    typeAndLeave("text-color", "#336699");

    expect(
      ["color-hue-value", "color-saturation-value", "color-level-value"].map((id) => readValue(id)),
    ).toEqual(["210", "67", "60"]);

    // S 50 with the exact 210° and 153 / 255: red 0.6 · 0.5 · 255 = 76.5 → 77 (4d); green
    // 0.6 · (1 − 0.5 · 0.5) · 255 = 114.75 → 115 (73); blue 153 (99).
    typeAndLeave("color-saturation-value", "50");
    expect(readValue("text-color")).toBe("#4d7399");

    // 67 is the value read from #336699: its exact 102 / 153 again. Rebuilt from #4d7399's exact
    // values (ADR-0058's wording), 0.6 · 0.33 · 255 = 50.49 would give #326699.
    typeAndLeave("color-saturation-value", "67");
    expect(readValue("text-color")).toBe("#336699");
  });

  it("holds the values typed, then shows the color's own hue in the other model: #7e7f80 at 210°", () => {
    // Against #ff8000 read: 200 then 1 typed give #fcfeff; 50 typed, the three values held as
    // typed, HSB 200°, 1 %, 50 %: red 0.5 · 0.99, 126.2 → 126 (7e); green 0.5 · (1 − 0.01 / 3),
    // 127.1 → 127 (7f); blue 127.5 → 128 (80). The picker holds the values typed, 200 among them.
    typeAndLeave("color-hue-value", "200");
    typeAndLeave("color-saturation-value", "1");
    typeAndLeave("color-level-value", "50");
    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["200", "#7e7f80"]);

    // Another model reads the color again: its own hue 60 · ((126 − 127) / 2 + 4) = 210°, the
    // color kept (ADR-0058).
    chooseOption("color-model", "hsl");
    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["210", "#7e7f80"]);
  });
});
