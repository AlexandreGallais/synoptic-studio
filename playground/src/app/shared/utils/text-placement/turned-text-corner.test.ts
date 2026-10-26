// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  openPage,
  readText,
  readValue,
  typeAndEnter,
} from "../../../../../testing";

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

// A box 100 × 20 at (47, 61): its middle (97, 71). cos 23° = 0.92050485, sin 23° = 0.39073113.
// Its straight box spans, about the middle, half of w |cos| + h |sin| across and of w |sin| +
// h |cos| down: (92.05049 + 7.81462) / 2 = 49.93255 and (39.07311 + 18.41010) / 2 = 28.74161,
// so X and Y show (97 − 49.93255, 71 − 28.74161) = (47.06745, 42.25840); its turned
// top-left corner, which a resize keeps, is (54.88207, 42.25840) (ADR-0057).
describe("Text tool: X and Y of a turned text show its straight box", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "100");
    typeAndEnter("text-box-height", "20");
    typeAndEnter("shape-x", "47");
    typeAndEnter("shape-y", "61");
  });

  it("[F17.AC12] shows the position at 0°, as before", () => {
    expect(readFields()).toEqual(["47", "61"]);
    expect(readStored()).toMatchObject({ x: 47, y: 61 });
  });

  it("[F17.AC12] shows the straight box (47.06745, 42.2584) when turned by 23, the position kept", () => {
    typeAndEnter("angle", "23");

    expect(readFields()).toEqual(["47.06745", "42.2584"]);
    expect(readStored()).toMatchObject({ angle: 23, x: 47, y: 61 });
  });

  it("[F17.AC12] moves the straight box to x 60 when X is typed 60, the angle, the size and Y kept", () => {
    typeAndEnter("angle", "23");
    typeAndEnter("shape-x", "60");

    // The shift 60 − 47.06745 = 12.93255: x = 47 + 12.93255 = 59.93255, y kept (ADR-0035).
    const stored = readStored();

    expect(readFields()).toEqual(["60", "42.2584"]);
    expect(stored.angle).toBe(23);
    expect(stored.x).toBeCloseTo(59.93255, 5);
    expect(stored.y).toBeCloseTo(61, 9);
    expect(readText("#model")).toContain('"boxWidth": 100');
  });

  it("[F17.AC12] moves the straight box to y 50 when Y is typed 50, X kept", () => {
    typeAndEnter("angle", "23");
    typeAndEnter("shape-y", "50");

    // The shift 50 − 42.258395 = 7.741605: y = 61 + 7.741605 = 68.741605, x kept.
    const stored = readStored();

    expect(readFields()).toEqual(["47.06745", "50"]);
    expect(stored.x).toBeCloseTo(47, 9);
    expect(stored.y).toBeCloseTo(68.741605, 6);
  });

  it("[F17.AC12] shows the straight box turned about the same middle when +90° is pressed", () => {
    typeAndEnter("text-box-width", "120");
    typeAndEnter("text-box-height", "40");
    typeAndEnter("angle", "23");
    clickElement("turn-right");

    // Box 120 × 40 at (47, 61), middle (107, 81); |cos 113°| = 0.39073113, sin 113° = 0.92050485.
    // Half spans (46.88774 + 36.82019) / 2 = 41.85396 and (110.46058 + 15.62925) / 2 = 63.04491:
    // (107 − 41.85396, 81 − 63.04491) = (65.14604, 17.95509).
    expect(readFields()).toEqual(["65.14604", "17.95509"]);
    expect(readStored()).toMatchObject({ angle: 113, x: 47, y: 61 });
  });

  it("[F17.AC12] keeps its turned corner when the box grows, X and Y showing its straight box", () => {
    typeAndEnter("angle", "23");
    typeAndEnter("text-box-width", "120");

    // ADR-0057: k' = (60, 10), R(k') = (55.23029 − 3.90731, 23.44387 + 9.20505) = (51.32298, 32.64892):
    // b' = (54.88207 − 60 + 51.32298, 42.25840 − 10 + 32.64892) = (46.20505, 64.90731). Its
    // height kept, its straight box still starts at (54.88207 − 20 sin 23°, 42.25840), the corner
    // being its highest point: (47.06745, 42.2584).
    const stored = readStored();

    expect(readFields()).toEqual(["47.06745", "42.2584"]);
    expect(stored.x).toBeCloseTo(46.20505, 5);
    expect(stored.y).toBeCloseTo(64.90731, 5);
  });

  it("[F17.AC12] places the text at the integer typed at 0°, bit for bit", () => {
    typeAndEnter("shape-x", "12");

    expect(readStored()).toMatchObject({ x: 12, y: 61 });
  });

  it("[F17.AC12] places the straight box at the x typed while the angle was refused, at the new angle", () => {
    typeAndEnter("angle", "23");
    // An angle emptied by a script, which the field never read: the text refused.
    typeAndEnter("angle", "");
    typeAndEnter("shape-x", "60");
    typeAndEnter("angle", "40");

    // Turned about its middle (97, 71), its position kept, then moved along x: cos 40° =
    // 0.76604444, sin 40° = 0.64278761; half spans (76.60444 + 12.85575) / 2 = 44.73010 and
    // (64.27876 + 15.32089) / 2 = 39.79982; it starts at (52.26990, 31.20018), and 60 typed in X
    // moves it by 60 − 52.26990 = 7.73010: x = 54.73010, y 61 kept, Y left as shown.
    const stored = readStored();

    expect(readFields()).toEqual(["60", "31.20018"]);
    expect(stored.x).toBeCloseTo(54.7301, 5);
    expect(stored.y).toBe(61);
  });

  it("[F17.AC12] goes back to the position at 0°", () => {
    typeAndEnter("angle", "23");
    typeAndEnter("angle", "0");

    expect(readFields()).toEqual(["47", "61"]);
  });
});
