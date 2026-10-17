// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  openServedPage,
  readText,
  readValue,
  typeValue,
} from "../user-actions";

/**
 * Reads the text drawn, as the model shows it: its stored position and its angle.
 *
 * @returns its position, the top-left of its frame before it turns, and its angle
 */
function readStored(): { readonly angle: number; readonly x: number; readonly y: number } {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page writes the model as JSON
  return JSON.parse(readText("#model")) as {
    readonly angle: number;
    readonly x: number;
    readonly y: number;
  };
}

/**
 * Reads the X and Y fields.
 *
 * @returns their values, X first
 */
function readFields(): readonly string[] {
  return [readValue("shape-x"), readValue("shape-y")];
}

// A box 100 × 20 at (47, 61): its middle (97, 71), half its size k = (50, 10).
// cos 23° = 0.92050485, sin 23° = 0.39073113; R(x, y) = (x cos − y sin, x sin + y cos).
// R(−50, −10) = (−46.02524 + 3.90731, −19.53656 − 9.20505) = (−42.11793, −28.74161):
// the turned top-left corner is (97 − 42.11793, 71 − 28.74161) = (54.88207, 42.25840) (ADR-0057).
describe("Text tool: X and Y of a turned text show its turned top-left corner (US-054)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    chooseOption("text-fit", "box");
    typeValue("text-box-width", "100");
    typeValue("text-box-height", "20");
    typeValue("shape-x", "47");
    typeValue("shape-y", "61");
  });

  it("[F17.AC12] shows the position at 0°, as before", () => {
    expect(readFields()).toEqual(["47", "61"]);
    expect(readStored()).toMatchObject({ x: 47, y: 61 });
  });

  it("[F17.AC12] shows the turned corner (54.88207, 42.2584) when turned by 23, the position kept", () => {
    typeValue("angle", "23");

    expect(readFields()).toEqual(["54.88207", "42.2584"]);
    expect(readStored()).toMatchObject({ angle: 23, x: 47, y: 61 });
  });

  it("[F17.AC12] moves the corner to x 60 when X is typed 60, the angle, the size and Y kept", () => {
    typeValue("angle", "23");
    typeValue("shape-x", "60");

    // b = P − k + R(k), P = (60, 42.25840), R(50, 10) = (46.02524 − 3.90731, 19.53656 + 9.20505)
    // = (42.11793, 28.74161): b = (60 − 50 + 42.11793, 42.25840 − 10 + 28.74161) = (52.11793, 61).
    const stored = readStored();

    expect(readFields()).toEqual(["60", "42.2584"]);
    expect(stored.angle).toBe(23);
    expect(stored.x).toBeCloseTo(52.11793, 5);
    expect(stored.y).toBeCloseTo(61, 9);
    expect(readText("#model")).toContain('"boxWidth": 100');
  });

  it("[F17.AC12] moves the corner to y 50 when Y is typed 50, X kept", () => {
    typeValue("angle", "23");
    typeValue("shape-y", "50");

    // b = (54.88207 − 50 + 42.11793, 50 − 10 + 28.741605) = (47, 68.741605).
    const stored = readStored();

    expect(readFields()).toEqual(["54.88207", "50"]);
    expect(stored.x).toBeCloseTo(47, 9);
    expect(stored.y).toBeCloseTo(68.741605, 6);
  });

  it("[F17.AC12] shows the corner turned about the same middle when +90° is pressed", () => {
    typeValue("text-box-width", "120");
    typeValue("text-box-height", "40");
    typeValue("angle", "23");
    clickElement("turn-right");

    // Box 120 × 40 at (47, 61), middle (107, 81); cos 113° = −0.39073113, sin 113° = 0.92050485.
    // R(−60, −20) = (23.44387 + 18.41010, −55.23029 + 7.81462) = (41.85396, −47.41567):
    // corner (148.85396, 33.58433).
    expect(readFields()).toEqual(["148.85396", "33.58433"]);
    expect(readStored()).toMatchObject({ angle: 113, x: 47, y: 61 });
  });

  it("[F17.AC12] keeps showing the corner when the box grows, the position computed again", () => {
    typeValue("angle", "23");
    typeValue("text-box-width", "120");

    // ADR-0057: k' = (60, 10), R(k') = (55.23029 − 3.90731, 23.44387 + 9.20505) = (51.32298, 32.64892):
    // b' = (54.88207 − 60 + 51.32298, 42.25840 − 10 + 32.64892) = (46.20505, 64.90731).
    const stored = readStored();

    expect(readFields()).toEqual(["54.88207", "42.2584"]);
    expect(stored.x).toBeCloseTo(46.20505, 5);
    expect(stored.y).toBeCloseTo(64.90731, 5);
  });

  it("[F17.AC12] places the text at the integer corner typed at 0°, bit for bit", () => {
    typeValue("shape-x", "12");

    expect(readStored()).toMatchObject({ x: 12, y: 61 });
  });

  it("[F17.AC12] places the corner typed while the angle was refused, at the new angle", () => {
    typeValue("angle", "23");
    typeValue("angle", "1.5");
    typeValue("shape-x", "60");
    typeValue("angle", "40");

    // The corner at (60, 42.25840), Y as shown at 23; cos 40° = 0.76604444, sin 40° = 0.64278761:
    // R(50, 10) = (38.30222 − 6.42788, 32.13938 + 7.66044) = (31.87435, 39.79982);
    // b = (60 − 50 + 31.87435, 42.25840 − 10 + 39.79982) = (41.87435, 72.05822).
    const stored = readStored();

    expect(readFields()).toEqual(["60", "42.2584"]);
    expect(stored.x).toBeCloseTo(41.87435, 5);
    expect(stored.y).toBeCloseTo(72.05822, 5);
  });

  it("[F17.AC12] goes back to the position at 0°", () => {
    typeValue("angle", "23");
    typeValue("angle", "0");

    expect(readFields()).toEqual(["47", "61"]);
  });
});
