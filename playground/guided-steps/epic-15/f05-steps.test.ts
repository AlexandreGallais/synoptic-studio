// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  openOnStep,
  readDisplay,
  readPressedState,
  readText,
  readValue,
  selectChoice,
  selectInput,
  selectPageElement,
  typeAndEnter,
  typeValue,
} from "../../user-actions";
import { GUIDED_STEPS } from "../lists";

/**
 * Opens the playground's epic list on a step of F05, found by its title.
 *
 * @param title - the step's title, without "F05 · "
 */
function openStep(title: string): void {
  const index = GUIDED_STEPS.findIndex((step) => step.title === `F05 · ${title}`);

  expect(index).toBeGreaterThanOrEqual(0);
  openOnStep(index + 1);
}

/**
 * Types a content in the Text tool's text area, as the designer would.
 *
 * @param content - the lines, a line feed between two
 */
function typeContent(content: string): void {
  const area = selectPageElement("#text-content");

  if (area instanceof HTMLTextAreaElement) {
    area.value = content;
  }

  area.dispatchEvent(new Event("input", { bubbles: true }));
}

/**
 * Reads each line drawn: its characters, `x` and `y`.
 *
 * @returns one row per `<tspan>`
 */
function readLines(): readonly (readonly (string | null)[])[] {
  return [...document.querySelectorAll("#canvas tspan")].map((span) => [
    span.textContent,
    span.getAttribute("x"),
    span.getAttribute("y"),
  ]);
}

/**
 * Reads the measured block shown in the pipeline column.
 *
 * @returns its width and height
 */
function readBlock(): { readonly height: number; readonly width: number } {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by showText from a TextBlock
  return JSON.parse(readText("#contour")) as { readonly height: number; readonly width: number };
}

/**
 * Reads an attribute of the drawn `<text>`.
 *
 * @param name - `fill`, `direction`, `font-family` or another attribute of the text
 * @returns its value, null when absent
 */
function readTextAttribute(name: string): string | null | undefined {
  return document.querySelector("#canvas text")?.getAttribute(name);
}

/**
 * Reads the `transform` of the text's group and of its frame.
 *
 * @returns the two values, the text's first
 */
function readTransforms(): readonly (string | null | undefined)[] {
  return [
    document.querySelector("#canvas g:has(> text)")?.getAttribute("transform"),
    document.querySelector("#canvas g.text-frame")?.getAttribute("transform"),
  ];
}

/**
 * Reads the underline and strike-through paths drawn with the text.
 *
 * @returns their path data
 */
function readStrokes(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas g:has(> text) path")].map((path) =>
    path.getAttribute("d"),
  );
}

/**
 * Whether the dashed frame of the text's block is drawn.
 *
 * @returns true when it is
 */
function hasFrame(): boolean {
  return document.querySelector("#canvas g.text-frame") !== null;
}

describe("the guided test of F05 (VAL-014): the Text tool's options", () => {
  it("is in the epic's list, and Next from F10's last step types F05's first", () => {
    openStep("Inter installed");
    clickElement("guide-previous");
    clickElement("guide-next");

    expect(readPressedState("tool-text")).toBe("true");

    expect(readLines()).toEqual([
      ["Pump P-101", "0", "31"],
      ["Running", "0", "69.71875"],
    ]);

    // "Pump P-101" in Inter Regular 32: (11351 − 93) × 32 / 2048 = 175.90625; 2 × 38.71875 tall.
    expect(readBlock()).toEqual({ height: 77.4375, width: 175.90625 });
    expect(hasFrame()).toBe(true);
  });

  it("content: three lines, then an empty last line, each 2478 × 32 / 2048 = 38.71875 tall", () => {
    openStep("Content and lines");
    typeContent("Pump P-101\nRunning\nTank T-1");
    expect(readBlock().height).toBe(116.15625);
    typeContent("Pump P-101\nRunning\n");
    expect(readBlock().height).toBe(116.15625);
  });

  it("an empty text draws nothing, no frame, no message", () => {
    openStep("Empty text");
    typeContent("");
    expect([readLines(), hasFrame(), readText("#status")]).toEqual([[], false, ""]);
  });

  it("size 48: the first baseline at 1984 × 48 / 2048 = 46.5", () => {
    openStep("Size");
    typeValue("text-size", "48");
    expect(readLines()[0]?.[2]).toBe("46.5");
    // 175.90625 × 48 / 32 and 3 × 38.71875.
    expect(readBlock()).toEqual({ height: 116.15625, width: 263.859375 });
  });

  it("weight 100, then 900, Bold released at both", () => {
    openStep("Weight");
    chooseOption("text-weight", "100");

    expect([readTextAttribute("font-weight"), readPressedState("text-bold")]).toEqual([
      "100",
      "false",
    ]);

    chooseOption("text-weight", "900");

    expect([readTextAttribute("font-weight"), readPressedState("text-bold")]).toEqual([
      "900",
      "false",
    ]);
  });

  it("italic, then upright", () => {
    openStep("Italic");
    clickElement("text-italic");
    expect(readTextAttribute("font-style")).toBe("italic");
    clickElement("text-italic");
    expect(readTextAttribute("font-style")).toBe("normal");
  });

  it("color, opacity 40, then 0: the frame stays", () => {
    openStep("Color and opacity");
    typeValue("text-color", "#dc2626");
    typeValue("text-opacity", "40");

    expect([readTextAttribute("fill"), readTextAttribute("fill-opacity")]).toEqual([
      "#dc2626",
      "0.4",
    ]);

    typeValue("text-opacity", "0");
    expect([readTextAttribute("fill-opacity"), hasFrame()]).toEqual(["0", true]);
  });

  it("letter spacing 10: 9 gaps in “Pump P-101”, none after its end", () => {
    openStep("Letter spacing");
    typeValue("text-letter-spacing", "10");
    // 175.90625 + 9 × 10.
    expect(readBlock().width).toBe(265.90625);
  });

  it("word spacing 20: the one space of “Pump P-101” widens", () => {
    openStep("Word spacing");
    typeValue("text-word-spacing", "20");
    expect(readBlock().width).toBe(195.90625);
    expect(readText("#pieces")).toContain('"width": 123.609375');
  });

  it("underline, then strike-through, one stroke per line", () => {
    openStep("Underline and strike-through");
    clickElement("text-underline");
    // Inter Regular: 31 + 348 × 32 / 2048 = 36.4375, 140 × 32 / 2048 = 2.1875 thick.
    expect(readStrokes()).toHaveLength(2);
    expect(readStrokes()[0]).toBe("M0 36.4375 L175.90625 36.4375 L175.90625 38.625 L0 38.625 Z");
    clickElement("text-underline");
    clickElement("text-strike");
    expect(readStrokes()).toHaveLength(2);
    expect(readStrokes()[0]).toMatch(/^M0 20\.51563 /u);
  });

  it.each([
    ["upper", "PUMP P-101 IS RUNNING"],
    ["lower", "pump p-101 is running"],
    ["title", "Pump P-101 Is Running"],
    ["camel", "pumpP101IsRunning"],
    ["pascal", "PumpP101IsRunning"],
    ["kebab", "pump-p-101-is-running"],
    ["snake", "pump_p_101_is_running"],
    ["constant", "PUMP_P_101_IS_RUNNING"],
  ])("case %s writes %s (Q36)", (textCase, written) => {
    openStep("Case (decision Q36)");
    chooseOption("text-case", textCase);
    expect(readLines()[0]?.[0]).toBe(written);
  });

  it("alignment center, then right: “Running” moves, “Pump P-101” stays", () => {
    openStep("Alignment");
    chooseOption("text-alignment", "center");
    // (175.90625 − 123.609375) / 2 = 26.1484375.
    expect(readLines().map(([, x]) => x)).toEqual(["0", "26.14844"]);
    chooseOption("text-alignment", "right");
    expect(readLines().map(([, x]) => x)).toEqual(["0", "52.29688"]);
  });

  it("[F05.AC4] right to left, aligned left: each line written at its right end", () => {
    openStep("Right to left: left is the left side (decision)");
    expect(readTextAttribute("direction")).toBe("rtl");
    expect(readLines().map(([, x]) => x)).toEqual(["175.90625", "123.60938"]);
  });

  it("Hebrew: each letter a missing-glyph box, 1344 × 32 / 2048 = 21 wide", () => {
    openStep("Hebrew");
    // שלום: 4 × 21; עולם טוב: 7 × 21 and a space of 9.
    expect(readLines().map(([, x]) => x)).toEqual(["84", "156"]);
  });
});

describe("the guided test of F05 (VAL-014): placing and turning", () => {
  it("X 100, Y 50 moves the text", () => {
    openStep("Position");
    typeValue("shape-x", "100");
    typeValue("shape-y", "50");
    expect(readLines()[0]).toEqual(["Pump P-101", "100", "81"]);
  });

  it("[F05.AC5] turned by 90 about the middle of its block", () => {
    openStep("Turned");
    typeValue("angle", "90");

    // (100 + 175.90625 / 2, 50 + 77.4375 / 2).
    expect(readTransforms()).toEqual([
      "rotate(90 187.95313 88.71875)",
      "rotate(90 187.95313 88.71875)",
    ]);
  });

  it("[F05.AC5] an angle kept from 0 to 359, and the quarter-turn buttons", () => {
    openStep("Angle kept from 0 to 359");

    const kept = ["370", "-90", "360"].map((typed) => {
      typeAndEnter("angle", typed);

      return readValue("angle");
    });

    expect(kept).toEqual(["10", "270", "0"]);
    expect(readTransforms()[0]).toBeNull();

    const turned = ["turn-right", "turn-left", "turn-left"].map((id) => {
      clickElement(id);

      return readValue("angle");
    });

    expect(turned).toEqual(["90", "0", "270"]);
  });

  it("[F05.AC5] the dashed frame turns with the letters", () => {
    openStep("The dashed frame");
    expect(readTransforms()[1]).toBe("rotate(30 87.95313 38.71875)");
    expect(document.querySelectorAll("#canvas g.text-frame path.straight-box")).toHaveLength(1);
  });

  it("the turn on the group holding the text and its four strokes", () => {
    openStep("Turned as a group (decision)");
    expect(readTransforms()[0]).toBe("rotate(90 87.95313 38.71875)");
    expect(readStrokes()).toHaveLength(4);
  });

  it.each([
    ["text-size", "0", "16"],
    ["text-opacity", "101", "100"],
    ["text-color", "red", "#1e293b"],
  ])("refuses %s %s, and draws again at %s", (id, refused, valid) => {
    openStep("Refused values");
    typeValue(id, refused);

    expect([readLines(), selectPageElement(`#${id}`).getAttribute("aria-invalid")]).toEqual([
      [],
      "true",
    ]);

    expect(readText("#status")).not.toBe("");
    typeValue(id, valid);
    expect([readLines().length, readText("#status")]).toEqual([2, ""]);
  });

  it("[F05.AC5] each tool keeps its placement", () => {
    openStep("Each tool keeps its placement");
    clickElement("tool-rectangle");
    expect(["shape-x", "shape-y", "angle"].map((id) => readValue(id))).toEqual(["0", "0", "0"]);
    typeValue("angle", "30");
    clickElement("tool-text");
    expect(["shape-x", "shape-y", "angle"].map((id) => readValue(id))).toEqual(["100", "50", "90"]);
    expect(readTransforms()[0]).toBe("rotate(90 187.95313 88.71875)");
  });
});

describe("the guided test of F05 (VAL-014): the decisions to confirm", () => {
  it("a font without metrics cannot be chosen: the list holds Inter alone", () => {
    openStep("A font without metrics (decided, US-036)");
    expect([...selectChoice("text-font").options].map((option) => option.value)).toEqual(["Inter"]);
    expect(readTextAttribute("font-family")).toBe('"Inter", sans-serif');
  });

  it("a font chosen in the list is written as the list holds it", () => {
    openStep("Font name as listed (decided, US-036)");
    chooseOption("text-font", "Inter");
    expect(readTextAttribute("font-family")).toBe('"Inter", sans-serif');
  });

  it("starts the next step with the drawing's options off, as the page opens (US-036)", () => {
    openStep("Font name as listed (decided, US-036)");

    for (const id of ["embed-fonts", "html-layer"]) {
      const box = selectInput(id);

      box.checked = true;
      box.dispatchEvent(new Event("change", { bubbles: true }));
    }

    expect(document.querySelector("#canvas switch")).not.toBeNull();
    clickElement("guide-next");

    expect([selectInput("embed-fonts").checked, selectInput("html-layer").checked]).toEqual([
      false,
      false,
    ]);

    expect(document.querySelector("#canvas switch")).toBeNull();
  });

  it("[F05.AC2] bold from 100 gives 700, then 100; from 800, 700 then 800", () => {
    openStep("Bold button (decided, US-037)");

    for (const weight of ["100", "800"]) {
      chooseOption("text-weight", weight);
      clickElement("text-bold");
      expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["700", "true"]);
      clickElement("text-bold");
      expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual([weight, "false"]);
    }
  });

  it("forgets the weight held when another step is opened", () => {
    openStep("Bold button (decided, US-037)");
    chooseOption("text-weight", "100");
    clickElement("text-bold");
    clickElement("guide-next");
    clickElement("guide-previous");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["400", "false"]);
    clickElement("text-bold");
    expect(readValue("text-weight")).toBe("700");
  });

  it("letter spacing 100 001 refused, −100 000 accepted without a frame", () => {
    openStep("Spacing bounds ±100 000 (decision)");
    typeValue("text-letter-spacing", "100001");
    expect(readText("#status")).not.toBe("");
    typeValue("text-letter-spacing", "-100000");
    expect([readText("#status"), hasFrame(), readBlock().width]).toEqual(["", false, 0]);
  });

  it("a line narrower than nothing, centered, starts right of the block's middle", () => {
    openStep("A line narrower than nothing (decision)");
    // "Pump P-101": 175.90625 − 9 × 20 = −4.09375; "Running": 123.609375 − 6 × 20 = 3.609375;
    // centered: (3.609375 + 4.09375) / 2 = 3.8515625.
    expect(readBlock().width).toBe(3.609375);
    expect(readLines().map(([, x]) => x)).toEqual(["3.85156", "0"]);
  });

  it("no stroke under a line 0 wide or less", () => {
    openStep("No stroke under an empty line (decision)");
    clickElement("text-underline");

    expect(readStrokes()).toEqual([
      "M0 75.15625 L3.60938 75.15625 L3.60938 77.34375 L0 77.34375 Z",
    ]);
  });

  it("a line separator does not break the line: a missing-glyph box", () => {
    openStep("Line breaks (decision and question)");
    // "Pump P-101" with a 21 box for its space of 9: 175.90625 − 9 + 21.
    expect([readLines().length, readBlock().width]).toEqual([1, 187.90625]);
  });

  it("a character outside the scripts measured is a missing-glyph box", () => {
    openStep("Characters measured (decision)");
    typeContent("中");
    expect(readBlock().width).toBe(21);
  });

  it("Inter has no line gap: two lines 77.4375 tall", () => {
    openStep("Where the line gap sits (question)");
    expect(readBlock().height).toBe(77.4375);
  });

  it("a font of line height 0 is a question only: the opening text is drawn", () => {
    openStep("A font of line height 0 (question)");
    expect(readLines()).toHaveLength(2);
  });

  it("“To” kerned 160 units closer than “T o” without its space", () => {
    openStep("Kerning across scripts (question)");
    // "T o" 48.84375 less a space of 9 is 39.84375; "To" 37.34375: 2.5 closer, 160 × 32 / 2048.
    expect(readText("#pieces")).toContain('"width": 37.34375');
    expect(readText("#pieces")).toContain('"width": 48.84375');
  });
});

describe("the guided test of F05 (VAL-014): the browser checks and the end", () => {
  it("letter spacing between the letters only: “HH” 23.78125 + 10", () => {
    openStep("Browser: letter spacing after the last letter");
    expect(readBlock().width).toBe(33.78125);
  });

  it("our underline at 200: its top 193.75 + 348 × 200 / 2048, 140 × 200 / 2048 thick", () => {
    openStep("Browser: our underline against the native one");

    // 227.734375 and 13.671875: half the thickness is 6.8359375.
    expect(readStrokes()).toEqual([
      "M0 227.73438 L297.26563 227.73438 L297.26563 241.40625 L0 241.40625 Z",
    ]);
  });

  it("[F05.AC6] back to the rectangle, drawn as before", () => {
    openStep("Back to the rectangle");
    clickElement("tool-rectangle");
    expect(readDisplay("text-options")).toBe("none");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(readText("#path-data")).toBe("M0 0 L120 0 L120 80 L0 80 Z");
  });
});
