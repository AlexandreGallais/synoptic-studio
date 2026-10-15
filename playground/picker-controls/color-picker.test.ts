// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  commitField,
  openServedPage,
  pressKey,
  pressPointer,
  readDisplay,
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
 * Reads the picker's three values: the hue, the saturation, the lightness or brightness.
 *
 * @returns them as the fields show them
 */
function readPickerValues(): readonly string[] {
  return ["color-hue-value", "color-saturation-value", "color-level-value"].map((id) =>
    readValue(id),
  );
}

/**
 * Reads an attribute of an element of the page.
 *
 * @param selector - CSS selector of the element
 * @param name - the attribute's name, `aria-valuenow`
 * @returns its value
 */
function readAttribute(selector: string, name: string): string | null {
  return selectPageElement(selector).getAttribute(name);
}

describe("Color picker: opening, sliders and values (US-035)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("color-model", "hsb");
    typeAndLeave("text-color", "#ff8000");
    clickElement("text-color-swatch");
  });

  it("[F16.AC2] opens on the swatch: hue 30.1°, HSB first, 30.1°, 100 %, 100 %", () => {
    expect(readDisplay("color-picker") === "none").toBe(false);
    expect(readAttribute("#text-color-swatch", "aria-expanded")).toBe("true");
    expect(readAttribute("#color-hue", "aria-valuenow")).toBe("30.1");
    expect(readAttribute("#color-hue", "aria-valuetext")).toBe("30.1°");
    expect(readPickerValues()).toEqual(["30.1", "100", "100"]);
    expect(selectPageElement("#color-level-name").textContent).toBe("B");
    expect([readValue("text-color"), readValue("text-opacity")]).toEqual(["#ff8000", "100"]);
  });

  it("[F16.AC2] shows HSL when chosen: 30.1°, 100 %, 50 %, the color kept", () => {
    chooseOption("color-model", "hsl");

    expect(readPickerValues()).toEqual(["30.1", "100", "50"]);
    expect(selectPageElement("#color-level-name").textContent).toBe("L");
    expect(readValue("text-color")).toBe("#ff8000");
  });

  it.each([
    // HSB(31.1°, 100 %, 100 %): green 1 − (1 − 31.1 / 60) = 0.51833, 132.2 → 84.
    { color: "#ff8400", hue: "31.1", key: "ArrowRight" },
    // 41.1°: green 0.685, 174.7 → af.
    { color: "#ffaf00", hue: "41.1", key: "PageUp" },
    { color: "#ff0000", hue: "0", key: "Home" },
    // 359.9°: blue 1 − 5.99833 mod 6 = 0.00167, 0.4 → 00.
    { color: "#ff0000", hue: "359.9", key: "End" },
  ])("[F16.AC2] takes the hue to $hue° on $key, the color following", ({ color, hue, key }) => {
    const order = ["ArrowRight", "PageUp", "Home", "End"];

    const pressed = order.slice(0, order.indexOf(key) + 1);

    for (const each of pressed) {
      pressKey("#color-hue", each);
    }

    expect([readAttribute("#color-hue", "aria-valuenow"), readValue("color-hue-value")]).toEqual([
      hue,
      hue,
    ]);

    expect([
      readValue("text-color"),
      selectPageElement("#canvas text").getAttribute("fill"),
    ]).toEqual([color, color]);
  });

  it("[F16.AC2] sets the hue where the slider is clicked, 360° at the right end shown 0", () => {
    setElementBox("#color-hue", { height: 10, left: 20, top: 0, width: 360 });
    // 360 · (200 − 20) / 360 = 180°: HSB(180°, 100 %, 100 %) is cyan.
    pressPointer("#color-hue", "pointerdown", { buttons: 1, clientX: 200, clientY: 5 });

    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["180", "#00ffff"]);

    pressPointer("#color-hue", "pointermove", { buttons: 1, clientX: 380, clientY: 5 });

    expect(readValue("color-hue-value")).toBe("0");
  });

  it("[F16.AC2] sets the opacity on its slider, by its keys and a click", () => {
    pressKey("#color-opacity", "PageDown");

    expect([readAttribute("#color-opacity", "aria-valuenow"), readValue("text-opacity")]).toEqual([
      "90",
      "90",
    ]);

    pressKey("#color-opacity", "Home");
    expect(selectPageElement("#canvas text").getAttribute("fill-opacity")).toBe("0");

    setElementBox("#color-opacity", { height: 10, left: 0, top: 0, width: 200 });
    // 100 · 67 / 200 = 33.5, to the nearest percent: 34.
    pressPointer("#color-opacity", "pointerdown", { buttons: 1, clientX: 67, clientY: 5 });
    expect(readValue("text-opacity")).toBe("34");
  });

  it("follows the opacity and the hexadecimal typed while it is open", () => {
    typeValue("text-opacity", "40");
    expect(readAttribute("#color-opacity", "aria-valuenow")).toBe("40");

    typeAndLeave("text-color", "#336699");
    expect(readPickerValues()).toEqual(["210", "66.7", "60"]);
  });

  it("[F16.AC2] sets a value typed, refused when left empty", () => {
    typeAndLeave("color-hue-value", "400");
    expect(readValue("color-hue-value")).toBe("0");

    typeAndLeave("color-saturation-value", "");
    expect(readValue("color-saturation-value")).toBe("100");
  });

  it("does not answer a key no slider answers", () => {
    pressKey("#color-hue", "a");
    pressKey("#color-opacity", "a");

    expect([readValue("color-hue-value"), readValue("text-opacity")]).toEqual(["30.1", "100"]);
  });

  it.each([
    {
      close: (): void => {
        pressKey("body", "Escape");
      },
      way: "Escape",
    },
    {
      close: (): void => {
        clickElement("text-color-swatch");
      },
      way: "a second click on the swatch",
    },
    {
      close: (): void => {
        pressPointer("#canvas", "pointerdown", { buttons: 1, clientX: 0, clientY: 0 });
      },
      way: "a click outside",
    },
  ])("[F16.AC2] closes on $way, the color kept", ({ close }) => {
    close();

    expect(readDisplay("color-picker")).toBe("none");
    expect(readAttribute("#text-color-swatch", "aria-expanded")).toBe("false");
    expect(readValue("text-color")).toBe("#ff8000");
  });

  it("stays open on a click in it or on its color's fields; moves under the background's", () => {
    pressPointer("#color-model", "pointerdown", { buttons: 1, clientX: 0, clientY: 0 });
    pressPointer("#text-opacity", "pointerdown", { buttons: 1, clientX: 0, clientY: 0 });
    expect(readDisplay("color-picker") === "none").toBe(false);

    clickElement("text-background-color-swatch");

    expect(selectPageElement("#text-background-color-message").nextElementSibling?.id).toBe(
      "color-picker",
    );

    expect(readPickerValues()).toEqual(["60", "100", "100"]);
  });
});
