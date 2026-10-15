// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  commitField,
  openOnStep,
  openServedPage,
  pressKey,
  pressPointer,
  readDisplay,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../../user-actions";
import { GUIDED_STEPS } from "../lists";

/**
 * Opens the playground's epic list on a step of F16, found by its title, the plane laid out
 * 200 × 100 and each slider 200 wide at the window's top left.
 *
 * @param title - the step's title, without "F16 · "
 */
function openStep(title: string): void {
  const index = GUIDED_STEPS.findIndex((step) => step.title === `F16 · ${title}`);

  expect(index).toBeGreaterThanOrEqual(0);
  openOnStep(index + 1);
  setElementBox("#color-plane", { height: 100, left: 0, top: 0, width: 200 });
  setElementBox("#color-hue", { height: 10, left: 0, top: 0, width: 200 });
  setElementBox("#color-opacity", { height: 10, left: 0, top: 0, width: 200 });
}

/**
 * Types a value into a field it gives the focus to, then leaves it, as the designer pressing Tab.
 *
 * @param id - the field
 * @param value - what is typed or pasted
 */
function typeAndLeave(id: string, value: string): void {
  selectPageElement(`#${id}`).dispatchEvent(new FocusEvent("focus"));
  typeValue(id, value);
  commitField(id);
}

/**
 * Presses the main button on the plane, the plane 200 × 100 at the window's top left.
 *
 * @param clientX - the pointer's x
 * @param clientY - the pointer's y
 */
function clickPlane(clientX: number, clientY: number): void {
  pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX, clientY });
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

/**
 * Whether the picker is shown.
 *
 * @returns true when open
 */
function isPickerOpen(): boolean {
  return readDisplay("color-picker") !== "none";
}

/**
 * Reads the hexadecimal and the opacity of the text's color fields.
 *
 * @returns the color, then the opacity
 */
function readTextColor(): readonly string[] {
  return [readValue("text-color"), readValue("text-opacity")];
}

describe("the guided test of F16 (VAL-016): the ways to type and pick a color", () => {
  it("is the current guided test, and Next from F14's last step types F16's first", () => {
    openServedPage("");
    expect(readText("#guide-title")).toBe("F16 · Six digits");

    openStep("Six digits");
    clickElement("guide-previous");
    expect(readText("#guide-title")).toBe("F14 · Back to the rectangle");
    clickElement("guide-next");
    expect([readValue("text-color"), readFills()[0]]).toEqual(["#1e293b", "#1e293b"]);
  });

  it.each([
    { title: "Six digits", typed: "FF8000", want: ["#ff8000", "100"] },
    { title: "With # and spaces", typed: " #FF8000 ", want: ["#ff8000", "100"] },
    { title: "Three digits", typed: "f80", want: ["#ff8800", "100"] },
    // C doubled is CC, 204; 100 · 204 / 255 = 80.
    { title: "Four digits", typed: "#F80C", want: ["#ff8800", "80"] },
    { title: "Eight digits", typed: "#0000ffcc", want: ["#0000ff", "80"] },
    { title: "Pasted", typed: "#22C55E", want: ["#22c55e", "100"] },
  ])("[F16.AC1] $title: $typed gives $want", ({ title, typed, want }) => {
    openStep(title);
    typeAndLeave("text-color", typed);
    expect(readTextColor()).toEqual(want);
    expect(readFills()[0]).toBe(want[0]);
  });

  it("[F16.AC1] refused twice, the opening color back with a line; then a color, the line gone", () => {
    openStep("Refused, the color back");

    for (const typed of ["red", "#ff800"]) {
      typeAndLeave("text-color", typed);
      expect(readValue("text-color")).toBe("#1e293b");
      expect(readText("#text-color-message")).toContain(typed);
    }

    typeAndLeave("text-color", "FF8000");
    expect([readValue("text-color"), readText("#text-color-message")]).toEqual(["#ff8000", ""]);
  });

  it("[F16.AC1] opacity 40: fill-opacity 0.4, the model's opacity 40", () => {
    openStep("Opacity");
    typeValue("text-opacity", "40");
    expect(readFills().slice(0, 2)).toEqual(["#ff8000", "0.4"]);
    expect(readText("#model")).toContain('"opacity": 40');
  });

  it("[F16.AC2] the swatch opens the picker under the color's fields: HSB 30.1, 100, 100", () => {
    openStep("The swatch opens the picker");
    clickElement("text-color-swatch");
    expect(isPickerOpen()).toBe(true);
    expect(selectPageElement("#text-color-message").nextElementSibling?.id).toBe("color-picker");

    expect([readValue("color-model"), ...readPickerValues()]).toEqual([
      "hsb",
      "30.1",
      "100",
      "100",
    ]);
  });

  it("[F16.AC2] closed by the swatch, by Escape, the focus back on the swatch, by a click outside", () => {
    openStep("The picker closed three ways");
    clickElement("text-color-swatch");
    clickElement("text-color-swatch");
    expect(isPickerOpen()).toBe(false);

    clickElement("text-color-swatch");
    pressKey("body", "Escape");
    expect(isPickerOpen()).toBe(false);
    expect(document.activeElement).toBe(selectPageElement("#text-color-swatch"));

    clickElement("text-color-swatch");
    pressPointer("#canvas", "pointerdown", { buttons: 1, clientX: 0, clientY: 0 });
    expect(isPickerOpen()).toBe(false);
  });

  it("[F16.AC2] the hue bar dragged: 100 of 200 is 180°; past the end 360°, shown 0, red", () => {
    openStep("Hue bar by mouse");
    clickElement("text-color-swatch");
    pressPointer("#color-hue", "pointerdown", { buttons: 1, clientX: 100, clientY: 5 });
    // HSB(180°, 100 %, 100 %) is cyan.
    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["180", "#00ffff"]);
    pressPointer("#color-hue", "pointermove", { buttons: 1, clientX: 400, clientY: 5 });
    expect([readValue("color-hue-value"), readValue("text-color")]).toEqual(["0", "#ff0000"]);
  });

  it("[F16.AC2] the hue bar's keys: 210, then 211, 221, 0, 359.9", () => {
    openStep("Hue bar by keys");
    clickElement("text-color-swatch");
    expect(readValue("color-hue-value")).toBe("210");

    const hues = ["ArrowRight", "PageUp", "Home", "End"].map((key) => {
      pressKey("#color-hue", key);

      return readValue("color-hue-value");
    });

    expect(hues).toEqual(["211", "221", "0", "359.9"]);
  });

  it("[F16.AC2] the opacity bar: 50 at its middle, then 49, 39, 100", () => {
    openStep("Opacity bar");
    clickElement("text-color-swatch");
    pressPointer("#color-opacity", "pointerdown", { buttons: 1, clientX: 100, clientY: 5 });
    expect([readValue("text-opacity"), readFills()[1]]).toEqual(["50", "0.5"]);

    const opacities = ["ArrowLeft", "PageDown", "End"].map((key) => {
      pressKey("#color-opacity", key);

      return readValue("text-opacity");
    });

    expect(opacities).toEqual(["49", "39", "100"]);
  });

  it("[F16.AC2] the plane in HSB: white at the top left, black at the bottom, #804000", () => {
    openStep("Plane in HSB");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");

    // HSB(30.1°, 100 %, 50 %): red 127.5, 128 (80); green 0.5 · 30.1 / 60 = 0.25083, 63.96 (40).
    const colors = [
      [0, 0],
      [100, 100],
      [200, 50],
    ].map(([x = 0, y = 0]) => {
      clickPlane(x, y);

      return readValue("text-color");
    });

    expect(colors).toEqual(["#ffffff", "#000000", "#804000"]);
  });

  it("[F16.AC2] the plane in HSL: L third, white at the top, #ff8000 at the right edge's middle", () => {
    openStep("Plane in HSL");
    clickElement("text-color-swatch");
    chooseOption("color-model", "hsl");
    expect(readText("#color-level-name")).toBe("L");
    clickPlane(100, 0);
    expect(readValue("text-color")).toBe("#ffffff");
    clickPlane(200, 50);
    expect(readValue("text-color")).toBe("#ff8000");
  });

  it("[F16.AC2] H 210, S 66.7, B 60 typed: #336699", () => {
    openStep("Values typed");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    typeAndLeave("color-hue-value", "210");
    typeAndLeave("color-saturation-value", "66.7");
    typeAndLeave("color-level-value", "60");
    expect(readValue("text-color")).toBe("#336699");
  });

  it("[F16.AC2] a grey opens at 0°; 200° kept, the top right #00aaff", () => {
    openStep("A grey keeps its hue");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    expect(readValue("color-hue-value")).toBe("0");
    typeAndLeave("color-hue-value", "200");
    clickPlane(200, 0);
    // HSB(200°, 100 %, 100 %): green 1 − 20 / 60 = 0.66667, 170 (aa); blue ff.
    expect(readValue("text-color")).toBe("#00aaff");
  });

  it("[F16.AC4] the background's swatch: the picker under its fields at 60°; the band green", () => {
    openStep("The background's color");
    clickElement("text-background-color-swatch");

    expect(selectPageElement("#text-background-color-message").nextElementSibling?.id).toBe(
      "color-picker",
    );

    expect(readValue("color-hue-value")).toBe("60");
    typeAndLeave("color-hue-value", "120");
    expect(readFills()).toEqual(["#1e293b", "1", "#00ff00", "1"]);
  });
});

describe("the guided test of F16 (VAL-016): its decisions, each with its example", () => {
  it("decision 1: the model holds the hexadecimal and the opacity, never HSL", () => {
    openStep("Hexadecimal stored (decision 1)");
    clickElement("text-color-swatch");
    chooseOption("color-model", "hsl");
    expect(readPickerValues()).toEqual(["30.1", "100", "50"]);
    expect(readText("#model")).toContain('"color": "#ff8000"');
    expect(readText("#model")).toContain('"opacity": 100');
    expect(readText("#model")).not.toContain("30.1");
  });

  it("decision 2: #FF8000 read in lower case; spaces around ignored", () => {
    openStep("Forms typed (decision 2)");
    expect([readValue("text-color"), readFills()[0]]).toEqual(["#FF8000", "#ff8000"]);
    typeAndLeave("text-color", " #ff8000 ");
    expect(readValue("text-color")).toBe("#ff8000");
  });

  it("decision 3: f80, #0000ffcc, and the alpha ec: 100 · 236 / 255 = 92.55, 93", () => {
    openStep("Three digits, and an alpha (decision 3)");

    const read = ["f80", "#0000ffcc", "#ff8000ec"].map((typed) => {
      typeAndLeave("text-color", typed);

      return readTextColor();
    });

    expect(read).toEqual([
      ["#ff8800", "100"],
      ["#0000ff", "80"],
      ["#ff8000", "93"],
    ]);
  });

  it("decision 4: red and #ff800 refused; the color pasted elsewhere is six digits", () => {
    openStep("Anything else refused; a copy (decision 4)");

    for (const typed of ["red", "#ff800"]) {
      typeAndLeave("text-color", typed);
      expect(readValue("text-color")).toBe("#ff8000");
    }

    typeAndLeave("text-background-color", readValue("text-color"));

    expect([readValue("text-background-color"), readValue("text-background-opacity")]).toEqual([
      "#ff8000",
      "100",
    ]);
  });

  it("decision 5: open, closed, open and closed by Escape, open and closed by a click outside", () => {
    openStep("Opened and closed (decision 5)");

    const states = ["swatch", "Escape", "outside"].flatMap((way) => {
      clickElement("text-color-swatch");

      const isOpen = isPickerOpen();

      if (way === "swatch") {
        clickElement("text-color-swatch");
      } else if (way === "Escape") {
        pressKey("body", "Escape");
      } else {
        pressPointer("#canvas", "pointerdown", { buttons: 1, clientX: 0, clientY: 0 });
      }

      return [isOpen, isPickerOpen()];
    });

    expect(states).toEqual([true, false, true, false, true, false]);
  });

  it("decision 6: HSB as the page opens, B third; HSL one choice away, L third", () => {
    openStep("HSB first (decision 6)");
    clickElement("text-color-swatch");
    expect([readValue("color-model"), readText("#color-level-name")]).toEqual(["hsb", "B"]);
    chooseOption("color-model", "hsl");
    expect(readText("#color-level-name")).toBe("L");
  });

  it("decision 7: HSB at 210°, the right edge's middle: #004080", () => {
    openStep("The plane's axes (decision 7)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    typeAndLeave("color-hue-value", "210");
    clickPlane(200, 50);
    expect(readValue("text-color")).toBe("#004080");
  });

  it("decision 8: #336699 is 210, 66.7, 60; S 67 gives #326699", () => {
    openStep("One decimal (decision 8)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    expect(readPickerValues()).toEqual(["210", "66.7", "60"]);
    typeAndLeave("color-saturation-value", "67");
    // Red 0.6 · (1 − 0.67) = 0.198, 50.49, 50 (32).
    expect(readValue("text-color")).toBe("#326699");
  });

  it("decision 9: HSL 30°, 100 %, 50 %: the green 127.5 rounded up, #ff8000", () => {
    openStep("Rounded, a tie up (decision 9)");
    chooseOption("color-model", "hsl");
    clickElement("text-color-swatch");
    typeAndLeave("color-hue-value", "30");
    typeAndLeave("color-saturation-value", "100");
    typeAndLeave("color-level-value", "50");
    expect(readValue("text-color")).toBe("#ff8000");
  });

  it("decision 10: #808080 opens at 0°; 200° kept as the saturation grows", () => {
    openStep("A grey's hue (decision 10)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    expect(readValue("color-hue-value")).toBe("0");
    typeAndLeave("color-hue-value", "200");
    // (100, 50) of 200 × 100: S 50 %, B 50 %.
    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 100, clientY: 50 });
    expect(readPickerValues()).toEqual(["200", "50", "50"]);
  });

  it("decision 11: hue 211, 221; on the plane S 67.7, then B 61", () => {
    openStep("Keys (decision 11)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    pressKey("#color-hue", "ArrowRight");
    expect(readValue("color-hue-value")).toBe("211");
    pressKey("#color-hue", "PageUp");
    pressKey("#color-plane", "ArrowRight");
    pressKey("#color-plane", "ArrowUp");
    expect(readPickerValues()).toEqual(["221", "67.7", "61"]);

    const hue = selectPageElement("#color-hue");

    expect([hue.getAttribute("aria-label"), hue.getAttribute("aria-valuenow")]).toEqual([
      "Hue",
      "221",
    ]);
  });

  it("decision 12: 30.1°, Right 31.1°, End 359.9°, 360 shown 0", () => {
    openStep("Hue to one decimal (decision 12)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");

    const hues = ["ArrowRight", "End"].map((key) => {
      pressKey("#color-hue", key);

      return readValue("color-hue-value");
    });

    typeAndLeave("color-hue-value", "360");
    expect([...hues, readValue("color-hue-value")]).toEqual(["31.1", "359.9", "0"]);
  });

  it("decision 13: the background's color has its swatch and picker", () => {
    openStep("The background's color too (decision 13)");
    clickElement("text-background-color-swatch");
    expect(isPickerOpen()).toBe(true);
    typeAndLeave("text-background-color", "#22c55e");
    expect(readFills()[2]).toBe("#22c55e");
  });

  it("decision 14: red then Tab brings #ff8800 back, a line under it", () => {
    openStep("Refused, the color back (decision 14)");
    typeAndLeave("text-color", "red");
    expect(readValue("text-color")).toBe("#ff8800");
    expect(readText("#text-color-message")).toContain("#ff8800 kept");
  });

  it("decision 15: empty shown again; hue 400 and −10 held at the ends; the opacity a percent", () => {
    openStep("Beyond the ends (decision 15)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");

    const hues = ["", "400", "-10"].map((typed) => {
      typeAndLeave("color-hue-value", typed);

      return readValue("color-hue-value");
    });

    // 100 · 67 / 200 = 33.5, to the nearest percent: 34.
    pressPointer("#color-opacity", "pointerdown", { buttons: 1, clientX: 67, clientY: 5 });
    expect([...hues, readValue("text-opacity")]).toEqual(["30.1", "0", "0", "34"]);
  });

  it("decision 16: the opacity typed while open moves the bar; Escape gives the swatch the focus", () => {
    openStep("Under its fields; Escape (decision 16)");
    clickElement("text-color-swatch");
    typeValue("text-opacity", "50");
    expect(selectPageElement("#color-opacity").getAttribute("aria-valuenow")).toBe("50");
    selectPageElement("#color-hue").focus();
    pressKey("body", "Escape");
    expect(isPickerOpen()).toBe(false);
    expect(document.activeElement).toBe(selectPageElement("#text-color-swatch"));
  });

  it("decision 17: #523a39 picked at HSB 0.7°, 30 %, 32.2 %, shown HSL 2.4°, 18 %, 27.3 %", () => {
    openStep("Another model, its own hue (decision 17)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    typeAndLeave("color-hue-value", "0.7");
    typeAndLeave("color-saturation-value", "30");
    typeAndLeave("color-level-value", "32.2");
    expect(readValue("text-color")).toBe("#523a39");
    chooseOption("color-model", "hsl");
    expect(readPickerValues()).toEqual(["2.4", "18", "27.3"]);
  });

  it("decision 18: the track is the slider, named Hue at 30.1; the thumb only drawn", () => {
    openStep("Read by a screen reader (decision 18)");
    clickElement("text-color-swatch");

    const hue = selectPageElement("#color-hue");
    const thumb = selectPageElement("#color-hue-thumb");

    expect([hue.getAttribute("role"), hue.getAttribute("aria-label")]).toEqual(["slider", "Hue"]);
    expect([hue.getAttribute("aria-valuenow"), hue.tabIndex]).toEqual(["30.1", 0]);
    expect([thumb.hasAttribute("role"), thumb.hasAttribute("tabindex")]).toEqual([false, false]);
  });

  it("decision 19: #002330's hue 196.25 shown 196.2; #030330's saturation 93.75 shown 93.7", () => {
    openStep("A half-tenth (decision 19)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    expect(readValue("color-hue-value")).toBe("196.2");
    typeAndLeave("text-color", "#030330");
    expect(readValue("color-saturation-value")).toBe("93.7");
  });

  it("decision 20: on the plane, Page Up 76.7 %, End 100 %, B kept at 60", () => {
    openStep("Page keys on the plane (decision 20)");
    chooseOption("color-model", "hsb");
    clickElement("text-color-swatch");
    pressKey("#color-plane", "PageUp");
    expect(readPickerValues()).toEqual(["210", "76.7", "60"]);
    pressKey("#color-plane", "End");
    expect(readPickerValues()).toEqual(["210", "100", "60"]);
  });

  it("[F16.AC4] back to F14: the box, the yellow band, the letters #1e293b", () => {
    openStep("Back to F14 and F05");
    expect(readFills()).toEqual(["#1e293b", "1", "#ffff00", "1"]);
    expect(document.querySelectorAll("#canvas tspan")).toHaveLength(1);
  });

  it("[F16.AC4] back to the rectangle, drawn as before", () => {
    openStep("Back to the rectangle");
    clickElement("tool-rectangle");
    expect([readDisplay("text-options"), isPickerOpen()]).toEqual(["none", false]);
    expect(readText("#path-data")).toBe("M0 0 L120 0 L120 80 L0 80 Z");
  });
});
