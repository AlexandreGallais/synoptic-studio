// @vitest-environment happy-dom

import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  chooseOption,
  clickElement,
  commitField,
  openServedPage,
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

describe("Color picker: focus, keys, rounding and what it writes (US-035, audit)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
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

  it("brings a value typed beyond 100 to 100, and rounds 66.75 to 66.8", () => {
    typeAndLeave("color-saturation-value", "150");
    expect(readValue("color-saturation-value")).toBe("100");

    typeAndLeave("color-saturation-value", "66.75");
    expect(readValue("color-saturation-value")).toBe("66.8");
  });

  it("rounds the plane's level to one decimal: 200 of 300 high is 66.7", () => {
    setElementBox("#color-plane", { height: 300, left: 0, top: 0, width: 300 });
    // 100 · (300 − 100) / 300 = 66.667, shown 66.7; x 300 of 300 is 100 %.
    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 300, clientY: 100 });

    expect(readValue("color-level-value")).toBe("66.7");
    expect(selectPageElement("#color-plane").getAttribute("aria-valuenow")).toBe("100");
  });

  it("takes a hue of 1.1 to 0.1 on Left, the binary64 error rounded away", () => {
    typeAndLeave("color-hue-value", "1.1");
    pressKey("#color-hue", "ArrowLeft");

    expect(readValue("color-hue-value")).toBe("0.1");
  });

  it("paints the plane with the pure hue, and the opacity slider with the color", () => {
    typeAndLeave("color-hue-value", "210");

    // HSB(210°, 100 %, 100 %): red 0, green 0.5, 127.5 gives 128 (80), blue ff.
    expect(selectPageElement("#color-plane").style.getPropertyValue("--hue-color")).toBe("#0080ff");

    expect(selectPageElement("#color-opacity").style.getPropertyValue("--picked-color")).toBe(
      readValue("text-color"),
    );
  });

  it("places the hue's thumb at 100 · 30.1 / 360 = 8.36111 %", () => {
    expect(selectPageElement("#color-hue-thumb").style.left).toMatch(/^8\.36111/u);
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

  it("keeps its hue when another model is chosen and it gives the color back: #7e7f80 at 200°", () => {
    typeAndLeave("color-hue-value", "200");
    typeAndLeave("color-saturation-value", "1");
    typeAndLeave("color-level-value", "50");
    chooseOption("color-model", "hsl");

    // #7e7f80 alone reads as hue 210°; HSL 200°, 0.8 %, 49.8 % gives #7e7f80 back: 200° kept.
    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["200", "#7e7f80"]);
  });

  it("[F16.AC3] shows the color's own hue when the one held would not give it back: #523a39", () => {
    typeAndLeave("color-hue-value", "0.7");
    typeAndLeave("color-saturation-value", "30");
    typeAndLeave("color-level-value", "32.2");
    expect(readValue("text-color")).toBe("#523a39");

    chooseOption("color-model", "hsl");

    // Own hue 60 · (0x3a − 0x39) / (0x52 − 0x39) = 60 / 25 = 2.4°; HSL 0.7°, 18 %, 27.3 % would
    // give #523939, so the hue shown is 2.4°, which gives #523a39 back.
    expect([
      readValue("color-hue-value"),
      readValue("color-saturation-value"),
      readValue("color-level-value"),
      readValue("text-color"),
    ]).toEqual(["2.4", "18", "27.3", "#523a39"]);

    typeAndLeave("color-level-value", "27.3");
    expect(readValue("text-color")).toBe("#523a39");
  });
});
