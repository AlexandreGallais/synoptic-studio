// @vitest-environment happy-dom

import { beforeEach, describe, expect, it, vi } from "vitest";

import { add, halfSize, turnAbout, turnDirection } from "../../src";
import { testFaceFile } from "../../tests";
import {
  chooseFiles,
  chooseOption,
  chooseShapeKind,
  clickElement,
  commitField,
  openServedPage,
  readText,
  readValue,
  selectPageElement,
  typeAndEnter,
} from "../user-actions";

import type { Point, Size } from "../../src";
import type { TestFace } from "../../tests";

/** The test font named Test Sans, regular, installable. */
const TEST_SANS: TestFace = { family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 };

// Inter 4.1 Regular 16, kerning included: "Pump P-101" 87.953125 wide, a line 19.359375 tall; a
// letter spacing of 10 adds 10 between each two of its 10 characters, 90: 177.953125.
// cos 23° = 0.92050485, sin 23° = 0.39073113. At (47, 61), turned by 23, its middle is
// (47 + 43.9765625, 61 + 9.6796875) = (90.9765625, 70.6796875), and its turned top-left corner
// P = middle − R(43.9765625, 9.6796875) = (90.97656 − 36.69848, 70.67969 − 26.09321)
// = (54.27808, 44.58648). Its straight box starts at the corner's y, the corner being its highest
// point, and h sin 23° to the left of it, X and Y showing it (AUD-040):
// (54.27808 − 19.359375 · 0.39073113, 44.58648) = (54.27808 − 7.56432, 44.58648) = (46.71377, 44.58648).

/** Precision of the corner kept: within 1e-9. */
const DIGITS = 9;

/** X and Y once turned by 23: the top-left of its straight box, at 5 decimals (AUD-040). */
const START_SHOWN = ["46.71377", "44.58648"];

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
 * Reads the text drawn: its exact position and angle from the model shown, its frame's size —
 * its block from the block shown, or its box.
 *
 * @returns its position, its angle and its frame's size
 */
function readDrawn(): Point & Size & { readonly angle: number } {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page writes the model as JSON
  const model = JSON.parse(readText("#model")) as {
    readonly angle: number;
    readonly boxHeight: number;
    readonly boxWidth: number;
    readonly isInBox: boolean;
    readonly x: number;
    readonly y: number;
  };
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page writes the block as JSON
  const block = JSON.parse(readText("#contour")) as Size;
  const frame = model.isInBox ? { height: model.boxHeight, width: model.boxWidth } : block;

  return { ...frame, angle: model.angle, x: model.x, y: model.y };
}

/**
 * The turned top-left corner of the text drawn, in the SVG: its position turned about the middle
 * of its frame (`DERIV-turned-shape` step 4).
 *
 * @returns where its top-left is drawn
 */
function readCorner(): Point {
  const drawn = readDrawn();
  const middle = add(drawn, halfSize(drawn));

  return turnAbout({ x: drawn.x, y: drawn.y }, middle, turnDirection(drawn.angle));
}

/**
 * Expects the corner drawn to be a point, within 1e-9.
 *
 * @param expected - the corner before the change
 */
function expectCorner(expected: Point): void {
  const corner = readCorner();

  expect(corner.x).toBeCloseTo(expected.x, DIGITS);
  expect(corner.y).toBeCloseTo(expected.y, DIGITS);
}

/**
 * Reads the X and Y fields.
 *
 * @returns their values, X first
 */
function readFields(): readonly string[] {
  return [readValue("shape-x"), readValue("shape-y")];
}

/**
 * Expects the position stored, the top-left of the frame before it turns, within 1e-5.
 *
 * @param expected - the position computed by hand
 */
function expectPosition(expected: Point): void {
  const drawn = readDrawn();

  expect(drawn.x).toBeCloseTo(expected.x, 5);
  expect(drawn.y).toBeCloseTo(expected.y, 5);
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

describe("Text tool: a turned text grows in place (US-038)", () => {
  beforeEach(() => {
    openServedPage("");
    chooseShapeKind("text");
    typeAndEnter("text-size", "16");
    chooseOption("text-weight", "400");
    typeContent("Pump P-101");
    typeAndEnter("shape-x", "47");
    typeAndEnter("shape-y", "61");
    typeAndEnter("angle", "23");
  });

  it("[F05.AC5] keeps the corner (54.27808, 44.58648) when the letter spacing rises to 10", () => {
    const corner = readCorner();

    expect(corner.x).toBeCloseTo(54.27808, 5);
    expect(corner.y).toBeCloseTo(44.58648, 5);

    typeAndEnter("text-letter-spacing", "10");

    // k' = (88.9765625, 9.6796875), R(k') = (78.12120, 43.67611):
    // b' = P − k' + R(k') = (54.27808 − 88.97656 + 78.12120, 44.58648 − 9.67969 + 43.67611);
    // its height kept, X and Y keep showing its straight box, P.x − h sin 23° and P.y.
    expectPosition({ x: 43.42272, y: 78.5829 });
    expect(readFields()).toEqual(START_SHOWN);
    expectCorner(corner);
  });

  it("[F05.AC5] keeps the corner step by step, the spacing raised from 0 to 10", () => {
    const corner = readCorner();

    for (const spacing of ["1", "2", "5", "10"]) {
      typeAndEnter("text-letter-spacing", spacing);
      expectCorner(corner);
    }
  });

  it("[F05.AC5] keeps the corner when a second line makes the block 38.71875 tall", () => {
    const corner = readCorner();

    typeAndEnter("text-letter-spacing", "10");
    typeContent("Pump P-101\nTank");

    // k' = (88.9765625, 19.359375), R(k') = (74.33905, 52.58631):
    // b' = (54.27808 − 88.97656 + 74.33905, 44.58648 − 19.35938 + 52.58631). Twice as tall, its
    // straight box starts at (54.278079 − 38.71875 · 0.39073113, 44.58648) = (54.278079 −
    // 15.128621, 44.58648) = (39.149458, 44.58648), shown 39.14946.
    expectPosition({ x: 39.64056, y: 77.81341 });
    expect(readFields()).toEqual(["39.14946", "44.58648"]);
    expectCorner(corner);
  });

  it("[F05.AC5] keeps the corner when the size goes from 16 to 32, and when the weight changes", () => {
    const corner = readCorner();

    typeAndEnter("text-size", "32");
    expectCorner(corner);

    chooseOption("text-weight", "700");
    expectCorner(corner);

    clickElement("text-italic");
    expectCorner(corner);
  });

  it("[F05.AC5] keeps the corner when the word spacing changes", () => {
    const corner = readCorner();

    typeAndEnter("text-word-spacing", "20");
    expectCorner(corner);
  });

  it("[F05.AC5] keeps the corner when the text goes into a box, then nothing moves on a spacing", () => {
    const corner = readCorner();

    // The box takes the block rounded up, 88 × 20 (US-055), then 120 × 40.
    chooseOption("text-fit", "box");
    expectCorner(corner);
    typeAndEnter("text-box-width", "120");
    typeAndEnter("text-box-height", "40");
    expectCorner(corner);

    const fields = readFields();

    typeAndEnter("text-letter-spacing", "10");
    expect(readFields()).toEqual(fields);
  });

  it("[F05.AC5] keeps the corner of a box 120 × 40 at (47, 61) set to 160 wide", () => {
    // Placed straight, then turned about its middle.
    typeAndEnter("angle", "0");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "120");
    typeAndEnter("text-box-height", "40");
    typeAndEnter("shape-x", "47");
    typeAndEnter("shape-y", "61");
    typeAndEnter("angle", "23");

    // Middle (107, 81); P = middle − R(60, 20) = (107 − 47.41567, 81 − 41.85396) = (59.58433, 39.14604);
    // k' = (80, 20), R(k') = (65.82577, 49.66859): b' = (59.58433 − 80 + 65.82577, 39.14604 − 20 + 49.66859).
    // Its height kept, X and Y show (59.58433 − 40 · 0.39073113, 39.14604) = (43.95509, 39.14604).
    const corner = readCorner();

    expect(corner.x).toBeCloseTo(59.58433, 5);

    typeAndEnter("text-box-width", "160");
    expectPosition({ x: 45.4101, y: 68.81462 });
    expect(readFields()).toEqual(["43.95509", "39.14604"]);
    expectCorner(corner);
  });

  it("[F05.AC5] turns the dashed frame with the text, about the same new middle", () => {
    typeAndEnter("text-letter-spacing", "10");

    // (43.42272 + 88.97656, 78.58290 + 9.67969) = (132.39928, 88.26259); the position, behind
    // the straight box X and Y show (AUD-040).
    expect(readTransforms()).toEqual([
      "rotate(23 132.39928 88.26259)",
      "rotate(23 132.39928 88.26259)",
    ]);
  });

  it("[F05.AC5] keeps the exact position behind the 5 decimals shown, a tool armed then left", () => {
    typeAndEnter("text-letter-spacing", "10");

    const corner = readCorner();
    const { x } = readDrawn();

    // Arming a tool leaves the text edited as it is, until the drawing is clicked (F35).
    clickElement("tool-rectangle");
    clickElement("tool-selector");

    expect(readFields()).toEqual(START_SHOWN);
    expect(readDrawn().x).toBe(x);

    typeAndEnter("text-letter-spacing", "4");
    expectCorner(corner);
  });

  it("[F05.AC5] refuses a fraction typed in X: a position typed is an integer (ADR-0057)", () => {
    const { x } = readDrawn();
    const [shown] = readFields();

    typeAndEnter("shape-x", "47.5");

    // Refused as any value of a field acting as a number field (US-085): X shown again.
    expect(readFields()[0]).toBe(shown);
    expect(readText("#status")).toBe("");
    expect(readDrawn().x).toBe(x);
  });

  it("[F05.AC5] accepts the 5 decimals shown typed back, the exact position behind them", () => {
    typeAndEnter("text-letter-spacing", "10");

    const { x } = readDrawn();

    typeAndEnter("shape-x", "46.71377");

    expect(readText("#status")).toBe("");
    expect(readDrawn().x).toBe(x);
  });

  it("[F05.AC5] keeps the corner when the font changes: Test Sans 400 imported", async () => {
    const corner = readCorner();

    const bytes = new Uint8Array(testFaceFile(TEST_SANS));

    chooseFiles("font-file", [new File([bytes], "400.ttf")]);

    await vi.waitFor(() => {
      expect(readText("#font-message")).toBe("Test Sans 400 imported.");
    });

    // "Pump P-101" in Test Sans 16: 80 wide; P = (54.27808, 44.58648) kept.
    expect(readDrawn().width).toBe(80);
    expectPosition({ x: 47.6911, y: 59.52252 });
    expect(readFields()[1]).toBe("44.58648");
    expectCorner(corner);
  });

  it("[F05.AC5] turns about the middle when the angle changes, the position kept", () => {
    typeAndEnter("text-letter-spacing", "10");

    const { x } = readDrawn();

    typeAndEnter("angle", "40");

    // cos 40° = 0.76604444, sin 40° = 0.64278761; k = (88.97656, 9.67969), middle
    // (43.42272 + 88.97656, 78.58290 + 9.67969) = (132.39928, 88.26259);
    // R(−k) = (−68.16000 + 6.22198, −57.19303 − 7.41507) = (−61.93802, −64.60810): the corner
    // (70.46126, 23.65449), the straight box's top; its left half of w cos 40° + h sin 40° =
    // (136.32000 + 12.44397) / 2 = 74.38198 from the middle's x 132.39928: 58.01730 (AUD-040).
    expect(readFields()).toEqual(["58.0173", "23.65449"]);
    expect(readDrawn().x).toBe(x);
  });

  it("[F05.AC5] draws a text where it is when its angle and its size change together", () => {
    // The angle refused while the spacing changes: the last text drawn is at 23°, 87.953125 wide.
    typeAndEnter("angle", "1.5");
    typeAndEnter("text-letter-spacing", "10");
    typeAndEnter("angle", "40");

    // Middle (47 + 88.97656, 61 + 9.67969) = (135.97656, 70.67969), R(−k) as at 40 above:
    // the corner (74.03854, 6.07158), X and Y showing (74.03854 − 12.44397, 6.07158) =
    // (61.59457, 6.07158) (AUD-040).
    expect(readFields()).toEqual(["61.59458", "6.07158"]);
    expect(readDrawn().x).toBe(47);
  });

  it("[F05.AC5] moves nothing at 0: the block grows to the right and down", () => {
    typeAndEnter("angle", "0");
    typeAndEnter("text-letter-spacing", "10");
    typeContent("Pump P-101\nTank");

    expect(readFields()).toEqual(["47", "61"]);
    expect(readTransforms()[0]).toBeNull();
  });

  it("[F05.AC5] keeps the corner of the last text drawn across a refused size", () => {
    const corner = readCorner();

    typeAndEnter("text-size", "0");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);

    typeAndEnter("text-size", "24");
    commitField("text-size");
    expectCorner(corner);
  });
});
