// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import { clickElement, openServedPage, readText, typeValue } from "../user-actions";

import { scheduleRedraw } from "./schedule-redraw";

/** The animation frames asked and not run yet, as the browser holds them until its next repaint. */
const FRAMES: FrameRequestCallback[] = [];

/**
 * Runs the frames asked, as the browser does before its next repaint.
 */
function runFrames(): void {
  const asked = FRAMES.splice(0);

  for (const frame of asked) {
    frame(0);
  }
}

/**
 * Reads the height of the measured block shown in the pipeline column.
 *
 * @returns its height
 */
function readBlockHeight(): number {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by showText from a TextBlock
  return (JSON.parse(readText("#contour")) as { readonly height: number }).height;
}

describe("A redraw per animation frame (US-048, REF-MDN-RAF)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");

    // Two lines typed, as the page opens with them in a browser, which drops the line break
    // right after `<textarea>`; happy-dom keeps it.
    const area = document.querySelector("#text-content");

    if (area instanceof HTMLTextAreaElement) {
      area.value = "Pump P-101\nRunning";
    }

    typeValue("text-size", "16");

    // From here, the frames wait for the test to run them, as in a browser.
    Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
      configurable: true,
      value: (callback: FrameRequestCallback): number => {
        FRAMES.push(callback);

        return FRAMES.length;
      },
    });
  });

  it("[F17.AC5] gathers 30 sizes typed before a frame into one redraw, the last size drawn", () => {
    const before = readBlockHeight();

    const sizes = Array.from({ length: 30 }, (unused, index) => 17 + index);

    for (const size of sizes) {
      typeValue("text-size", String(size));
    }

    // One frame asked for the 30 inputs, none drawn yet: no redraw waits behind another.
    expect(FRAMES).toHaveLength(1);
    expect(readBlockHeight()).toBe(before);

    runFrames();

    // Two lines of Inter, each 2478 × size / 2048 tall: 38.71875 at 16, 111.31640625 at 46.
    expect(before).toBe(38.71875);
    expect(readBlockHeight()).toBe(111.31640625);
    expect(FRAMES).toHaveLength(0);
  });

  it("[F17.AC5] asks a new frame for a value typed after the frame ran", () => {
    typeValue("text-size", "20");
    runFrames();
    typeValue("text-size", "32");

    expect(FRAMES).toHaveLength(1);
    runFrames();
    // Two lines of 2478 × 32 / 2048 = 38.71875: 77.4375.
    expect(readBlockHeight()).toBe(77.4375);
  });

  it("redraws at once a document without a window", () => {
    const calls: string[] = [];
    const detached = document.implementation.createHTMLDocument("");

    scheduleRedraw(detached, () => {
      calls.push("draw");
    });

    expect(detached.defaultView).toBeNull();
    expect(calls).toEqual(["draw"]);
  });
});
