// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  openServedPage,
  readText,
  selectPageElement,
  typeAndEnter,
  typeValue,
} from "../user-actions";

// Inter 4.1 Regular at size 16, kerning included, as `lineWidth` measures: "Hello" 38.5625,
// "World" 44.49219, "Hell…" 42.79688, "Hello…" 52.39063, "runn…" 48.21875, "running" 57.53125,
// "Pump P-101" 87.95313, "is running" 74.35156, "Pump P-101 is" 104.77344. A line is
// 2478 × 16 / 2048 = 19.359375 tall, its baseline 1984 × 16 / 2048 = 15.5 below its top; the cap
// height is 1490 × 16 / 2048 = 11.640625.

/** A content of four words, wrapped by the boxes of these tests. */
const PUMP = "Pump P-101 is running";

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
 * Reads the characters of each line drawn.
 *
 * @returns one string per `<tspan>`
 */
function readContents(): readonly (string | null | undefined)[] {
  return readLines().map(([content]) => content);
}

/**
 * Reads the baseline of each line drawn.
 *
 * @returns one `y` per `<tspan>`
 */
function readBaselines(): readonly (string | null | undefined)[] {
  return readLines().map((line) => line[2]);
}

/**
 * Reads the dashes of the selected text's frame.
 *
 * @returns the frame's path data
 */
function readFrame(): string | null {
  return selectPageElement("#canvas g.text-frame path.straight-box").getAttribute("d");
}

describe("Text tool: a text in a box (US-033)", () => {
  beforeEach(() => {
    openServedPage("");
    chooseShapeKind("text");
    typeValue("text-size", "16");
    // happy-dom loses the option marked `selected` once the page is moved into its document: the
    // choices the page opens with are chosen again.
    chooseOption("text-weight", "400");
    chooseOption("text-vertical-alignment", "top");
    typeContent("Hello World");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "50");
    typeAndEnter("text-box-height", "100");
  });

  it("[F14.AC1] wraps 'Hello World' in a box 50 wide: one <text>, one positioned <tspan> a line", () => {
    // "Hello World" is 38.5625 + space + 44.49219, wider than 50: "World" goes to a second line,
    // its baseline 15.5 + 19.359375 = 34.859375.
    expect(readLines()).toEqual([
      ["Hello", "0", "15.5"],
      ["World", "0", "34.85938"],
    ]);

    expect(document.querySelectorAll("#canvas text")).toHaveLength(1);
    expect(document.querySelector("#canvas [text-anchor], #canvas [dominant-baseline]")).toBeNull();
  });

  it("[F14.AC1] frames the box, 50 × 100, not the measured block", () => {
    // Dashes 4 long every 7, 1 thick: the top side's last one from 49 to 50, the right side's
    // from 98 to 100 at x 49, the bottom side's first at y 99.
    const dashes = readFrame();

    expect(dashes).toMatch(/^M0 0H4V1H0Z /u);
    expect(dashes).toContain("M49 0H50V1H49Z");
    expect(dashes).toContain("M49 98H50V100H49Z");
    expect(dashes).toContain("M0 99H4V100H0Z");
    // The block measured stays that of the lines: "World", the widest.
    expect(readText("#contour")).toContain('"width": 44.4921875');
  });

  it("[F14.AC2] cuts at Max lines 1 with 'Hell…', the ellipsis written as a character", () => {
    typeValue("text-maximum-lines", "1");

    // "Hello…" (52.39063) is wider than 50; "Hell…" (42.79688) fits.
    expect(readLines()).toEqual([["Hell…", "0", "15.5"]]);
    expect(selectPageElement("#canvas text").textContent).toBe("Hell…");
    expect(readText("#path-data")).toContain(">Hell…</tspan>");
  });

  it("[F17.AC1] keeps a word too long whole, cut with an ellipsis, or drawn whole without it", () => {
    // Inter 16: "running" (57.53) is wider than 50, never broken into "runnin" (47.72) over "g".
    typeContent(PUMP);
    expect(readContents()).toEqual(["Pump", "P-101", "is", "runn…"]);

    clickElement("text-ellipsis");
    expect(readContents()).toEqual(["Pump", "P-101", "is", "running"]);
  });

  it("[F14.AC3] places the block of four lines (77.4375 tall) at the top, middle and bottom", () => {
    typeContent(PUMP);
    chooseOption("text-vertical-alignment", "middle");
    // Room (100 − 4 × 19.359375) / 2 = 11.28125 above: 15.5 + 11.28125 = 26.78125.
    expect(readBaselines()).toEqual(["26.78125", "46.14063", "65.5", "84.85938"]);

    chooseOption("text-vertical-alignment", "bottom");
    // 15.5 + 22.5625 = 38.0625; the last baseline 96.140625, its descent reaching 100.
    expect(readBaselines()).toEqual(["38.0625", "57.42188", "76.78125", "96.14063"]);
  });

  it("[F14.AC1] frames the box at its position and turns both about the box's middle", () => {
    typeAndEnter("shape-x", "10");
    typeAndEnter("shape-y", "20");
    // The box 50 × 100 at (10, 20): its first dash from (10, 20), its middle (35, 70).
    expect(readFrame()).toMatch(/^M10 20H14V21H10Z /u);

    typeValue("angle", "90");

    expect(
      ["#canvas g:has(> text)", "#canvas g.text-frame"].map((selector) =>
        selectPageElement(selector).getAttribute("transform"),
      ),
    ).toEqual(["rotate(90 35 70)", "rotate(90 35 70)"]);
  });

  it("[F14.AC3] aligns the lines across the box's width", () => {
    chooseOption("text-alignment", "center");

    // (50 − 38.5625) / 2 = 5.71875; (50 − 44.4921875) / 2 = 2.75390625.
    expect(readLines().map(([, x]) => x)).toEqual(["5.71875", "2.75391"]);
  });

  it("[F14.AC3] spreads the lines 24 apart, then back to the font's line height", () => {
    typeContent(PUMP);
    typeValue("text-line-height", "24");
    // Half-leading (24 − 19.359375) / 2 = 2.3203125: the first baseline at 17.8203125.
    expect(readBaselines()).toEqual(["17.82031", "41.82031", "65.82031", "89.82031"]);

    typeValue("text-line-height", "0");
    expect(readBaselines()).toEqual(["15.5", "34.85938", "54.21875", "73.57813"]);
  });

  it("[F14.AC5] trims the block to the first line's cap height", () => {
    typeContent(PUMP);
    clickElement("text-trim");

    // The first baseline one cap height, 11.640625, below the box's top.
    expect(readBaselines()).toEqual(["11.64063", "31", "50.35938", "69.71875"]);
  });

  it("[F17.AC1] wraps 'Pump P-101 is running' automatically in a box 110 wide", () => {
    typeContent(PUMP);
    typeAndEnter("text-box-width", "110");
    // Inter 16: "Pump P-101 is" (104.77) fits; with " running" (57.53) it would not.
    expect(readContents()).toEqual(["Pump P-101 is", "running"]);
  });

  it("turns the text and its box by 90 about the box's middle", () => {
    typeContent(PUMP);
    typeAndEnter("text-box-width", "110");
    typeValue("angle", "90");

    // The box 110 × 100 at (0, 0): its middle (55, 50).
    expect(
      ["#canvas g:has(> text)", "#canvas g.text-frame"].map((selector) =>
        selectPageElement(selector).getAttribute("transform"),
      ),
    ).toEqual(["rotate(90 55 50)", "rotate(90 55 50)"]);
  });

  it("draws no frame around a box 0 wide, its lines still shown", () => {
    typeAndEnter("text-box-width", "0");

    expect(document.querySelector("#canvas g.text-frame")).toBeNull();
    // Each word on its own line, kept whole and cut to its first character and the ellipsis,
    // drawn past the box's edge.
    expect(readContents()).toEqual(["H…", "W…"]);
  });

  it("[F14.AC6] draws the text as in F05 once Fit content is chosen again", () => {
    typeContent(PUMP);
    typeValue("text-maximum-lines", "1");
    chooseOption("text-fit", "content");

    // One line, the block 166.80469 wide: the frame's right side, 1 thick, ends at it.
    expect(readLines()).toEqual([[PUMP, "0", "15.5"]]);
    expect(readFrame()).toContain("M165.80469 0H166.80469V4H165.80469Z");
  });
});
