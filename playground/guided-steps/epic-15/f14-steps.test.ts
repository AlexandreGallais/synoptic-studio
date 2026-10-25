// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  openOnStep,
  readDisplay,
  readText,
  readValue,
  selectPageElement,
  typeAndEnter,
  typeInField,
  typeValue,
} from "../../user-actions";
import { GUIDED_STEPS } from "../lists";

// Inter 4.1 Regular at size 16, kerning included, as `lineWidth` measures: "Hello" 38.5625,
// "World" 44.49219, "Hell…" 42.79688, "Hello…" 52.39063, "runn…" 48.21875, "running" 57.53125,
// "Pump P-101" 87.95313, "Running" 61.80469, "Pump P-101 is running" 166.80469. A line is
// (1984 + 494) × 16 / 2048 = 19.359375 tall, its baseline 1984 × 16 / 2048 = 15.5 below its top,
// 494 × 16 / 2048 = 3.859375 above its bottom; the cap height is 1490 × 16 / 2048 = 11.640625.

/**
 * Opens the playground's epic list on a step of F14, found by its title.
 *
 * @param title - the step's title, without "F14 · "
 */
function openStep(title: string): void {
  const index = GUIDED_STEPS.findIndex((step) => step.title === `F14 · ${title}`);

  expect(index).toBeGreaterThanOrEqual(0);
  openOnStep(index + 1);
}

/**
 * Reads the characters of each line drawn.
 *
 * @returns one string per `<tspan>`
 */
function readContents(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas tspan")].map((span) => span.textContent);
}

/**
 * Reads the baseline of each line drawn.
 *
 * @returns one `y` per `<tspan>`
 */
function readBaselines(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas tspan")].map((span) => span.getAttribute("y"));
}

/**
 * Reads the start of each line drawn.
 *
 * @returns one `x` per `<tspan>`
 */
function readLineStarts(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas tspan")].map((span) => span.getAttribute("x"));
}

/**
 * Reads the measured block shown in the pipeline column.
 *
 * @returns its width and height
 */
function readBlock(): { readonly height: number; readonly width: number } {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by showText from a TextBlock
  return JSON.parse(readText("#contour")) as { readonly height: number; readonly width: number };
}

/**
 * Reads the dashes of the selected text's frame.
 *
 * @returns the frame's path data
 */
function readFrame(): string | null {
  return selectPageElement("#canvas g.text-frame path.straight-box").getAttribute("d");
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
 * Reads the background's `<path>`: the one drawn before the `<text>` in its group.
 *
 * @returns its path data, fill and fill opacity, undefined without a path
 */
function readBackground(): readonly (string | null | undefined)[] {
  const path = selectPageElement("#canvas text").parentElement?.querySelector("path");

  return ["d", "fill", "fill-opacity"].map((name) => path?.getAttribute(name));
}

/**
 * Whether the page refuses the text: nothing drawn, a message, the fields outlined.
 *
 * @param id - the field typed, without `#`
 * @returns true when refused
 */
function isRefused(id: string): boolean {
  return (
    readContents().length === 0 &&
    readText("#status") !== "" &&
    selectPageElement(`#${id}`).getAttribute("aria-invalid") === "true"
  );
}

describe("the guided test of F14 (VAL-015): the box's options", () => {
  it("Next from F05's last step types F14's first", () => {
    openStep("Fit content or Box");
    clickElement("guide-previous");
    clickElement("guide-next");
    expect([readValue("text-fit"), readContents()]).toEqual(["content", ["Hello World"]]);
  });

  it("[F17.AC15] a step is a new text: Box measures it again, the box typed before forgotten", () => {
    openStep("Fit content or Box");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "50");
    clickElement("guide-previous");
    clickElement("guide-next");
    chooseOption("text-fit", "box");

    expect([readValue("text-box-width"), readValue("text-box-height")]).toEqual(["88", "20"]);
  });

  it("[F14.AC1] Box: “Hello” over “World”, the frame the box 50 × 100", () => {
    openStep("Fit content or Box");
    chooseOption("text-fit", "box");

    // The box takes the text's block rounded up, nothing moving.
    expect([readValue("text-box-width"), readValue("text-box-height"), readContents()]).toEqual([
      "88",
      "20",
      ["Hello World"],
    ]);

    typeAndEnter("text-box-width", "50");
    typeAndEnter("text-box-height", "100");
    expect([readDisplay("text-box-fields"), readContents()]).toEqual(["block", ["Hello", "World"]]);
    // Dashes 4 long every 7, 1 thick: the right side's last from 98 to 100 at x 49.
    expect(readFrame()).toContain("M49 98H50V100H49Z");
  });

  it("width 200: one line; height 10: the line still drawn", () => {
    openStep("Width and height");
    typeAndEnter("text-box-width", "200");
    expect(readContents()).toEqual(["Hello World"]);
    typeAndEnter("text-box-height", "10");
    expect([readContents(), readBaselines()]).toEqual([["Hello World"], ["15.5"]]);
  });

  it("[F17.AC1] a long word kept whole, cut with an ellipsis; no option for it", () => {
    openStep("Long word kept whole");

    // No field and no label for a word too long: the option is removed.
    expect([
      document.querySelector("#text-long-word") === null,
      document.body.textContent.includes("Long words"),
    ]).toEqual([true, false]);

    expect([readContents(), readBlock().width]).toEqual([
      ["Pump", "P-101", "is", "runn…"],
      48.21875,
    ]);
  });

  it("[F14.AC1] ellipsis off: “running” whole, 57.53125 past 50", () => {
    openStep("Ellipsis");
    clickElement("text-ellipsis");
    expect([readContents()[3], readBlock().width]).toEqual(["running", 57.53125]);
    clickElement("text-ellipsis");
    expect(readContents()[3]).toBe("runn…");
  });

  it("[F14.AC2] max lines 1: “Hell…”; 0: two lines again", () => {
    openStep("Max lines");
    typeAndEnter("text-maximum-lines", "1");
    expect(readContents()).toEqual(["Hell…"]);
    typeAndEnter("text-maximum-lines", "0");
    expect(readContents()).toEqual(["Hello", "World"]);
  });

  it("[F14.AC3] middle, then bottom: down (100 − 77.4375) / 2 = 11.28125, then 22.5625", () => {
    openStep("Vertical alignment");
    chooseOption("text-vertical-alignment", "middle");
    expect(readBaselines()[0]).toBe("26.78125");
    chooseOption("text-vertical-alignment", "bottom");
    expect(readBaselines()[0]).toBe("38.0625");
  });

  it("[F14.AC3] center, then right: across the box 200, (200 − 87.55469) / 2 = 56.22266", () => {
    openStep("Alignment across the box");
    chooseOption("text-alignment", "center");
    expect(readLineStarts()).toEqual(["56.22266"]);
    chooseOption("text-alignment", "right");
    expect(readLineStarts()).toEqual(["112.44531"]);
  });

  it("[F14.AC3] line height 24: (24 − 19.359375) / 2 + 15.5 = 17.82031, then 24 apart", () => {
    openStep("Line height");
    typeAndEnter("text-line-height", "24");
    expect(readBaselines()).toEqual(["17.82031", "41.82031", "65.82031", "89.82031"]);
    typeAndEnter("text-line-height", "0");
    expect(readBaselines().slice(0, 2)).toEqual(["15.5", "34.85938"]);
  });

  it("[F14.AC5] trim: the first baseline at the cap height, 11.64063", () => {
    openStep("Trim");
    clickElement("text-trim");
    expect(readBaselines()[0]).toBe("11.64063");
  });

  it("[F14.AC4] a yellow band behind each line, 19.359375 tall, the two touching", () => {
    openStep("Background");
    clickElement("text-background");

    expect(readBackground()).toEqual([
      "M0 0 L87.95313 0 L87.95313 19.35938 L0 19.35938 Z M0 19.35938 L61.80469 19.35938 L61.80469 38.71875 L0 38.71875 Z",
      "#ffff00",
      "1",
    ]);
  });

  it("[F14.AC4] the background's color and opacity", () => {
    openStep("Background color and opacity");
    typeValue("text-background-color", "#22c55e");
    typeAndEnter("text-background-opacity", "40");
    expect(readBackground().slice(1)).toEqual(["#22c55e", "0.4"]);
  });

  it("turned by 90 about the box's middle, (55, 50)", () => {
    openStep("Turned about the box");
    typeAndEnter("angle", "90");
    expect(readTransforms()).toEqual(["rotate(90 55 50)", "rotate(90 55 50)"]);
  });

  it.each([
    ["100001", "100000"],
    ["", "110"],
  ])("caps or refuses the box's width %j, the field then showing %s (US-085)", (typed, shown) => {
    openStep("Refused values");
    typeAndEnter("text-box-width", typed);
    expect([readValue("text-box-width"), readText("#status")]).toEqual([shown, ""]);
    typeAndEnter("text-box-width", "110");
    expect([readContents().length, readText("#status")]).toEqual([2, ""]);
  });

  it("refuses the background's opacity 101, 100 shown again, the text drawn (US-087)", () => {
    openStep("Refused values");
    typeInField("text-background-opacity", "101");
    expect(isRefused("text-background-opacity")).toBe(false);

    expect([
      readValue("text-background-opacity"),
      readContents().length,
      readText("#status"),
    ]).toEqual(["100", 2, ""]);
  });

  it("[F14.AC6] back to F05: one line 166.80469 wide, the box's fields hidden", () => {
    openStep("Back to F05");
    chooseOption("text-fit", "content");

    expect([readDisplay("text-box-fields"), readContents()]).toEqual([
      "none",
      ["Pump P-101 is running"],
    ]);

    expect(readBlock().width).toBe(166.8046875);
  });
});

describe("the guided test of F14 (VAL-015): the decisions to confirm", () => {
  it("decision 1: a width of 100 001 refused, 60 drawn", () => {
    openStep("Box from 0 to 100 000 (decision 1)");
    // Capped at 100 000 as a shape's width, the same rule.
    typeAndEnter("text-box-width", "100001");
    expect(readValue("text-box-width")).toBe("100000");
    typeAndEnter("text-box-width", "60");
    expect(readContents()).toEqual(["Hello", "World"]);
  });

  it("decision 2: the opening space stays with “Hello”", () => {
    openStep("Opening spaces (decision 2)");
    expect(readContents()).toEqual([" Hello", "World"]);
  });

  it("[F17.AC1] decisions 3 and 4: a word kept whole, “runn…”, then whole without the ellipsis", () => {
    openStep("Long word kept whole (decisions 3 and 4)");
    expect(readContents()).toEqual(["runn…"]);
    clickElement("text-ellipsis");
    expect(readContents()).toEqual(["running"]);
  });

  it("decision 5: no break at a no-break space, the line whole past the box", () => {
    openStep("No-break space (decision 5)");
    expect(readContents()).toEqual(["Hello\u{A0}World"]);
  });

  it("decision 6: one line at least in a box 5 tall; max lines 100 001 taken at 100 000", () => {
    openStep("One line at least (decision 6)");
    expect(readContents()).toEqual(["Hell…"]);
    typeInField("text-maximum-lines", "100001");
    // Taken at the field's most, still one line in a box 5 tall.
    expect([readValue("text-maximum-lines"), readContents()]).toEqual(["100000", ["Hell…"]]);
  });

  it("decision 7: the ellipsis on, then off", () => {
    openStep("Ellipsis on or off (decision 7)");
    expect(readContents()).toEqual(["Hello…"]);
    clickElement("text-ellipsis");
    expect(readContents()).toEqual(["Hello"]);
  });

  it("decision 8: the ellipsis removes characters", () => {
    openStep("Ellipsis removes characters (decision 8)");
    expect([readContents(), readBlock().width]).toEqual([["Hello…"], 52.390625]);
    typeAndEnter("text-box-width", "50");
    expect([readContents(), readBlock().width]).toEqual([["Hell…"], 42.796875]);
  });

  it("decision 9: one character a line, none with an ellipsis", () => {
    openStep("One character too wide (decision 9)");
    expect(readContents()).toEqual(["W", "a"]);
  });

  it("decision 10: an empty line left out adds no ellipsis", () => {
    openStep("Empty lines left out (decision 10)");
    expect(readContents()).toEqual(["Hello"]);
  });

  it("decision 11: “H…” drawn past a box 10 wide", () => {
    openStep("Even “H…” too wide (decision 11)");
    expect([readContents(), readBlock().width]).toEqual([["H…"], 25.71875]);
  });

  it("decision 12: the leading spread half above, half below", () => {
    openStep("Line height spread (decision 12)");
    typeAndEnter("text-line-height", "30");
    // (30 − 19.359375) / 2 + 15.5 = 20.8203125.
    expect(readBaselines()).toEqual(["20.82031"]);
    typeAndEnter("text-line-height", "0");
    expect(readBaselines()).toEqual(["15.5"]);
  });

  it("[F17.AC1] decisions 13 to 15: the automatic wrap only, no Wrap field", () => {
    openStep("Automatic wrap only (decisions 13 to 15)");
    expect(readContents()).toEqual(["Pump P-101 is", "running"]);

    // No field and no label for a wrap style: the balanced wrap is removed.
    expect([
      document.querySelector("#text-wrap") === null,
      document.body.textContent.includes("Balance"),
    ]).toEqual([true, false]);
  });

  it("decision 16: a line height without a box, the box's fields hidden", () => {
    openStep("Options of every text (decision 16)");
    typeAndEnter("text-line-height", "30");

    expect([readBaselines(), readDisplay("text-box-fields")]).toEqual([
      ["20.82031", "50.82031"],
      "none",
    ]);
  });

  it("decision 17: a yellow opaque background; the box's defaults", () => {
    openStep("Defaults (decision 17)");
    clickElement("text-background");
    expect(readBackground().slice(1)).toEqual(["#ffff00", "1"]);
    chooseOption("text-fit", "box");

    expect(
      ["text-maximum-lines", "text-vertical-alignment", "text-box-width", "text-box-height"].map(
        (id) => readValue(id),
      ),
    ).toEqual(["0", "top", "88", "20"]);

    expect(selectPageElement("#text-ellipsis")).toHaveProperty("checked", true);
  });

  it("decision 18: a band from the ascender to the descender", () => {
    openStep("Band from ascender to descender (decision 18)");
    expect(readBackground()[0]).toBe("M0 0 L38.5625 0 L38.5625 19.35938 L0 19.35938 Z");
  });

  it("decision 19: bands 4.640625 apart; trimmed, the first above the block's top", () => {
    openStep("Bands under a line height (decision 19)");
    // The first baseline 17.8203125 less the ascent 15.5; the second 24 lower.
    expect(readBackground()[0]).toMatch(/^M0 2\.32031 .* M0 26\.32031 /u);
    clickElement("text-trim");
    // The first baseline at the cap height 11.640625, less 15.5.
    expect(readBackground()[0]).toMatch(/^M0 -3\.85938 /u);
  });

  it("decision 20: overlapping bands in one path, filled once", () => {
    openStep("Overlapping bands (decision 20)");

    // (12 − 19.359375) / 2 + 15.5 − 15.5 = −3.6796875; the second 12 lower, 8.3203125.
    expect(readBackground()).toEqual([
      "M0 -3.67969 L38.5625 -3.67969 L38.5625 15.67969 L0 15.67969 Z M0 8.32031 L44.49219 8.32031 L44.49219 27.67969 L0 27.67969 Z",
      "#ffff00",
      "0.4",
    ]);
  });

  it("decision 21: turned about the box's middle, (30, 20)", () => {
    openStep("Turned about the box's middle (decision 21)");
    typeAndEnter("angle", "90");
    expect(readTransforms()).toEqual(["rotate(90 30 20)", "rotate(90 30 20)"]);
  });

  it("decision 22: an ellipsis may come after U+202F, never right after U+00A0", () => {
    openStep("Other no-break characters (decision 22)");
    expect(readContents()).toEqual(["100\u{202F}…", "100…"]);
  });

  it("[F05.AC5] a turned box: a spacing moves nothing, 160 wide keeps its corner", () => {
    openStep("A turned box resized in place (US-038)");
    typeAndEnter("text-letter-spacing", "10");
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["47", "61"]);

    // X and Y show its straight box; a width typed keeps its turned top-left corner
    // (ADR-0057), the highest point at 23°, so that, its height kept, X and Y stay.
    typeAndEnter("text-box-width", "160");
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["47", "61"]);
  });

  it("[F14.AC7] a box 60 × 40 turned by 90: its straight box at (10, −10), 40 × 60", () => {
    openStep("The straight box of a turned text (US-039)");
    typeAndEnter("angle", "90");

    // R(90°) about (30, 20): (0, 0) ↦ (50, −10), (60, 40) ↦ (10, 50).
    expect(readText("#straight-box")).toBe("Straight box: x 10, y -10, width 40, height 60");
    expect(readDisplay("straight-box") === "none").toBe(false);
  });

  it("[F14.AC6] back to the rectangle, drawn as before", () => {
    openStep("Back to the rectangle");
    chooseShapeKind("rectangle");
    expect(readDisplay("text-options")).toBe("none");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(readText("#path-data")).toBe("M0 0 L120 0 L120 80 L0 80 Z");
  });
});
